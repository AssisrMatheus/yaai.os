//! Exercise command streams, staged snapshots and vault parsing through the binary.
//! spec: docs/repo/doc-links.md#commands
#[cfg(test)]
mod tests {
    //! User-facing command contracts.
    use serde_json::Value;
    use std::{
        fs,
        path::Path,
        process::{Command, Output},
    };
    use tempfile::TempDir;

    /// Copy the small reference vault; materialize source templates outside the live repo scan.
    fn fixture() -> TempDir {
        let root = tempfile::tempdir().unwrap();
        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/vault");
        for entry in walkdir::WalkDir::new(&source) {
            let entry = entry.unwrap();
            let mut dest = root
                .path()
                .join(entry.path().strip_prefix(&source).unwrap());
            if dest.ends_with("main.rs.md") {
                let _ = dest.set_extension("");
            }
            if entry.file_type().is_dir() {
                fs::create_dir_all(dest).unwrap();
            } else {
                let _ = fs::copy(entry.path(), dest).unwrap();
            }
        }
        root
    }

    /// Run the compiled binary with independent stdout/stderr.
    fn cli(root: &Path, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_doc-links"))
            .current_dir(root)
            .args(args)
            .env_remove("DOC_LINKS_LOG")
            .output()
            .unwrap()
    }

    /// Return the JSON result after asserting an exact process status.
    fn report(out: &Output, status: i32) -> Value {
        assert_eq!(
            out.status.code(),
            Some(status),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        serde_json::from_slice(&out.stdout).unwrap()
    }

    /// Exercise Markdown examples, both alias syntaxes, archives, anchors, rule and test resolution.
    #[test]
    fn clean_vault() {
        let temp = fixture();
        let root = temp.path();
        let clean = cli(root, &["check", "--json"]);
        assert_eq!(report(&clean, 0)["total"], 0);
        assert_eq!(clean.stderr, b"");
        let refs = report(&cli(root, &["--json", "refs", "apps/main.rs"]), 0);
        assert_eq!(refs["total"], 5);
        let matrix = report(&cli(root, &["matrix", "--json"]), 0);
        assert_eq!(
            matrix["rows"],
            serde_json::json!(["INV-DOCS-008", "docs/repo/doc-links.md"])
        );
        let graph = report(&cli(root, &["graph", "--json"]), 0);
        assert!(graph["rows"][0].as_str().unwrap().contains("absent"));
        assert!(cli(root, &["refs", "--help"]).status.success());
        assert_eq!(cli(root, &["refs"]).status.code(), Some(1));
    }

    /// Filter locally while using all pages to resolve links; corrupt metadata and archive targets.
    #[test]
    fn repairs_and_limits() {
        let temp = fixture();
        let root = temp.path();
        let page = "docs/repo/bad.md";
        fs::write(root.join(page),"---\ndescription: ''\ntags: word\n---\n[[repo/.old/earlier]] [[repo/doc-links#missing]]\n## Covers\n- `missing.rs`\n").unwrap();
        fs::write(root.join("README.md"), "[past](docs/repo/.old/earlier.md)").unwrap();
        let out = cli(root, &["check", page, "--json", "--limit", "2"]);
        let result = report(&out, 2);
        assert_eq!(result["shown"], 2);
        assert_eq!(result["total"], 6);
        assert_ne!(out.stderr, b"");
        assert_eq!(
            report(&cli(root, &["check", "README.md", "--json"]), 2)["total"],
            1
        );
        assert_eq!(
            report(&cli(root, &["check", "apps", "--json"]), 0)["total"],
            0
        );
        fs::write(
            root.join("docs/repo/.old/earlier.md"),
            "---\nsuperseded_by: [[missing]]\n---\n",
        )
        .unwrap();
        assert_eq!(
            report(&cli(root, &["check", "docs/repo/.old", "--json"]), 2)["total"],
            2
        );
    }

    /// A disposable Git index catches partially staged changes without touching the user's index.
    #[test]
    fn staged_snapshot() {
        let temp = fixture();
        let root = temp.path();
        let git = |args: &[&str]| {
            let out = Command::new("git")
                .args(args)
                .current_dir(root)
                .output()
                .unwrap();
            assert!(
                out.status.success(),
                "{}",
                String::from_utf8_lossy(&out.stderr)
            );
            String::from_utf8(out.stdout).unwrap()
        };
        let _ = git(&["init", "-q"]);
        let _ = git(&["update-index", "--add", "--", "apps/main.rs"]);
        for e in walkdir::WalkDir::new(root.join("docs")) {
            let e = e.unwrap();
            if e.file_type().is_file() {
                let _ = git(&[
                    "update-index",
                    "--add",
                    "--",
                    e.path().strip_prefix(root).unwrap().to_str().unwrap(),
                ]);
            }
        }
        let tree = git(&["write-tree"]);
        let head = git(&[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit-tree",
            tree.trim(),
            "-m",
            "base",
        ]);
        let _ = git(&["update-ref", "HEAD", head.trim()]);
        fs::write(
            root.join("apps/main.rs"),
            "//! spec: docs/missing.md\nfn main() {}",
        )
        .unwrap();
        let _ = git(&["update-index", "apps/main.rs"]);
        fs::write(root.join("apps/main.rs"), "fn main() {}").unwrap();
        fs::write(root.join("msg"), "edit\n\nSpec-unchanged: still accurate\n").unwrap();
        let gate = || report(&cli(root, &["commit", "msg", "--json"]), 2);
        assert_eq!(gate()["findings"][0]["code"], "E_CITE");
        let _ = git(&["update-index", "apps/main.rs"]);
        assert_eq!(
            report(&cli(root, &["commit", "msg", "--json"]), 0)["total"],
            0
        );
        fs::write(root.join("msg"), "edit\n\nSpec-unchanged: \n").unwrap();
        assert_eq!(gate()["findings"][0]["code"], "E_DRIFT");
    }

    /// Keep native edges and real graphify IDs across repeated injections.
    #[test]
    fn graph_replacement() {
        let temp = fixture();
        let dir = temp.path().join("graphify-out");
        fs::create_dir(&dir).unwrap();
        let path = dir.join("graph.json");
        fs::write(&path, include_bytes!("fixtures/graph.json")).unwrap();
        for removed in [0, 5] {
            let result = report(&cli(temp.path(), &["graph", "--json"]), 0);
            assert_eq!(
                result["rows"][0],
                format!("added=5 removed={removed} unplaced=0")
            );
            let graph: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            let links = graph["links"].as_array().unwrap();
            assert_eq!(links.len(), 6);
            assert!(links.iter().any(|l| l["relation"] == "validates"
                && l["source"] == "apps_main_links_resolve"
                && l["target"] == "docs_invariants_docs_inv_docs_007_links"
                && l["confidence"] == "EXTRACTED"));
        }
    }
    /// Keep operational failures separate from validation reports and logs parseable.
    #[test]
    fn diagnostics_and_ignored_directories() {
        let temp = fixture();
        let root = temp.path();
        for dir in ["vendor", "node_modules", "target", "graphify-out"] {
            let dir = root.join("docs").join(dir);
            fs::create_dir(&dir).unwrap();
            fs::write(dir.join("broken.md"), [0xff]).unwrap();
        }
        let out = cli(root, &["check", "--json", "-v"]);
        assert_eq!(report(&out, 0)["total"], 0);
        let events: Vec<Value> = std::str::from_utf8(&out.stderr)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert!(
            events
                .iter()
                .any(|e| e["scope"] == "doc-links.scan" && e["level"] == "debug")
        );
        fs::write(root.join("docs/broken.md"), [0xff]).unwrap();
        let failed = cli(root, &["check", "--json"]);
        assert_eq!(failed.status.code(), Some(1));
        assert_eq!(failed.stdout, b"");
        assert!(String::from_utf8_lossy(&failed.stderr).contains("E_RUN"));
    }

    /// Enforce frontmatter bounds and parent-index membership independently.
    #[test]
    fn shape_boundaries() {
        let temp = fixture();
        let root = temp.path();
        let dir = root.join("docs/nested");
        fs::create_dir(&dir).unwrap();
        let page = "docs/nested/page.md";
        fs::write(
            root.join(page),
            format!(
                "---\ndescription: {}\ntags: []\n{}---\n# Header\n",
                "x".repeat(301),
                "# padding\n".repeat(9)
            ),
        )
        .unwrap();
        let result = report(&cli(root, &["check", "docs/nested", "--json"]), 2);
        let codes: Vec<_> = result["findings"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| f["code"].as_str().unwrap())
            .collect();
        assert_eq!(codes, ["E_SCOPE", "E_INDEX", "E_LENGTH", "E_META"]);
    }
}
