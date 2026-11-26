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

    /// Copy the small reference vault and the real kinds file without its temporary allowance; materialize source
    /// templates outside the live repo scan.
    fn fixture() -> TempDir {
        let root = tempfile::tempdir().unwrap();
        let kinds =
            fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("kinds.toml")).unwrap();
        // Comments hold the file's own citation; the fixture vault has no page for it.
        let kinds: String = kinds
            .lines()
            .filter(|l| !l.starts_with('#'))
            .map(|l| {
                if l.starts_with("pending_migration") {
                    "pending_migration = []"
                } else {
                    l
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::create_dir_all(root.path().join("tooling/doc-links")).unwrap();
        fs::write(root.path().join("tooling/doc-links/kinds.toml"), kinds).unwrap();
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

    /// Workflow citations must resolve just like citations in repository tools.
    #[test]
    fn workflow_citations_are_checked() {
        let temp = fixture();
        let workflows = temp.path().join(".github/workflows");
        fs::create_dir_all(&workflows).unwrap();
        fs::write(workflows.join("check.yml"), "# spec: docs/missing.md\n").unwrap();
        let result = cli(temp.path(), &["check", "--json"]);
        let findings = report(&result, 2);
        assert!(findings.to_string().contains(".github/workflows/check.yml"));
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
        assert_eq!(refs["total"], 6);
        let matrix = report(&cli(root, &["matrix", "--json"]), 0);
        assert_eq!(
            matrix["rows"],
            serde_json::json!([
                "INV-DOCS-008",
                "docs/repo/doc-links.md",
                "docs/specs/sample.spec.md"
            ])
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
            root.join("docs/repo/.old/earlier.2026-11-26.old.md"),
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
        for removed in [0, 6] {
            let result = report(&cli(temp.path(), &["graph", "--json"]), 0);
            assert_eq!(
                result["rows"][0],
                format!("added=6 removed={removed} nodes=0 unplaced=0")
            );
            let graph: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            let links = graph["links"].as_array().unwrap();
            assert_eq!(links.len(), 7);
            assert!(links.iter().any(|l| l["relation"] == "validates"
                && l["source"] == "apps_main_links_resolve"
                && l["target"] == "docs_invariants_docs_inv_docs_007_links"
                && l["confidence"] == "EXTRACTED"));
        }
    }
    /// A linked file graphify made no node for gets one file node, replaced on each run; git-ignored files and
    /// placeholders are never scanned.
    #[test]
    fn graph_file_nodes_and_ignored_files() {
        let temp = fixture();
        let root = temp.path();
        let extra = root.join("apps/extra");
        fs::create_dir_all(extra.join("gen")).unwrap();
        fs::write(
            extra.join("config.toml"),
            "# spec: docs/specs/sample.spec.md\n",
        )
        .unwrap();
        fs::write(extra.join(".gitignore"), "gen/\n").unwrap();
        fs::write(extra.join(".gitkeep"), "// spec: docs/missing.md\n").unwrap();
        fs::write(extra.join("gen/out.rs"), "// spec: docs/missing.md\n").unwrap();
        assert_eq!(report(&cli(root, &["check", "--json"]), 0)["total"], 0);
        let dir = root.join("graphify-out");
        fs::create_dir(&dir).unwrap();
        let path = dir.join("graph.json");
        fs::write(&path, include_bytes!("fixtures/graph.json")).unwrap();
        let before: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        let native = before["nodes"].as_array().unwrap().len();
        for removed in [0, 7] {
            let result = report(&cli(root, &["graph", "--json"]), 0);
            let graph: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            let nodes = graph["nodes"].as_array().unwrap();
            assert_eq!(
                result["rows"][0],
                format!("added=7 removed={removed} nodes=1 unplaced=0")
            );
            assert_eq!(nodes.len(), native + 1);
            let made: Vec<_> = nodes
                .iter()
                .filter(|n| n["_origin"] == "doc-links")
                .collect();
            assert_eq!(made.len(), 1);
            assert_eq!(made[0]["source_file"], "apps/extra/config.toml");
            assert_eq!(made[0]["id"], "apps_extra_config_toml");
            assert!(
                graph["links"].as_array().unwrap().iter().any(|l| {
                    l["relation"] == "cites" && l["source"] == "apps_extra_config_toml"
                })
            );
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

    /// The findings of `check` on one path, as (code, message) pairs, after asserting exit 2.
    fn findings(root: &Path, path: &str) -> Vec<(String, String)> {
        let out = cli(root, &["check", path, "--json", "--limit", "500"]);
        assert_eq!(out.status.code(), Some(2), "{path} passed");
        let result: Value = serde_json::from_slice(&out.stdout).unwrap();
        result["findings"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| {
                (
                    f["code"].as_str().unwrap().to_owned(),
                    f["message"].as_str().unwrap().to_owned(),
                )
            })
            .collect()
    }

    /// Assert that one finding has the code and contains the text.
    fn expect(found: &[(String, String)], code: &str, text: &str) {
        assert!(
            found.iter().any(|(c, m)| c == code && m.contains(text)),
            "want {code} with {text:?}, got {found:?}"
        );
    }

    /// The clean vault holds one passing file of every kind; this breaks each one and checks the repair it names.
    #[test]
    fn every_kind_fails_with_its_repair() {
        let temp = fixture();
        let root = temp.path();
        let screen = fs::read_to_string(root.join("docs/screens/home.screen.md")).unwrap();
        let cases: [(&str, String, &str, &str); 12] = [
            (
                "docs/screens/home.screen.md",
                screen.replace("## Lifecycle\n\nText.\n", ""),
                "E_KIND",
                "add the H2 \"Lifecycle\" after \"States\" (kind screen, docs/repo/doc-kinds.md#screen)",
            ),
            (
                "docs/screens/home.screen.md",
                screen.replace("## Data", "## Extra\n\nText.\n\n## Data"),
                "E_KIND",
                "remove or rename the H2 \"Extra\"",
            ),
            (
                "docs/adr/0013-links.adr.md",
                "---\ndescription: D.\ntags: [x]\nstatus: maybe\n---\n\n# A\n\n- list first\n\n# B\n\n## Notes\n\nx\n".into(),
                "E_META",
                "set status to one of proposed, accepted, superseded (kind adr",
            ),
            (
                "docs/invariants/docs.invariants.md",
                "---\ndescription: D.\ntags: [x]\n---\n\n# R\n\n## INV-DOCS-007: Links\n\nCode cites docs.\n\n## INV-REPO-001: Wrong\n\nActive · Decided: 2026-11-26, owner\n\nBody.\n".into(),
                "E_ITEM",
                "add the line \"Active · Decided: YYYY-MM-DD, <who>\" under \"INV-DOCS-007: Links\"",
            ),
            (
                "docs/repo/.old/earlier.2026-11-26.old.md",
                "---\nsuperseded_by: \"[[repo/doc-links]]\"\ncommit: later\n---\n# E\n".into(),
                "E_META",
                "set commit to a value matching",
            ),
            (
                "docs/repo/.old/earlier.md",
                "---\nsuperseded_by: \"[[repo/doc-links]]\"\ncommit: abc1234\n---\n# E\n".into(),
                "E_PLACE",
                "rename to <<replaced file name without .md>.<YYYY-MM-DD>",
            ),
            (
                "docs/specs/sample.spec.md",
                "---\ndescription: D.\ntags: [x]\n---\n\n# S\n\n## Covers\n\n- `apps/main.rs`\n".into(),
                "E_KIND",
                "add the H2 \"Rules\" before \"Covers\" (kind spec",
            ),
            (
                "docs/best-practices/sample.rules.md",
                "---\ndescription: D.\ntags: [x]\n---\n\n# P\n\n### BP-SAMPLE-001: One\n\nSee [x](https://example.invalid).\nWhy: w.\nCheck: review\n\n### BP-OTHER-002: Two\n\nText.\nWhy: w.\nCheck: sometimes\n\n### Loose heading\n".into(),
                "E_ITEM",
                "use this page's prefix BP-SAMPLE",
            ),
            (
                "docs/repo/SKILL.md",
                "---\ndescription: D.\ntags: [x]\n---\n\n# R\n\n## Pages\n\n[[repo/doc-links]] [[repo/doc-kinds]]\n".into(),
                "E_KIND",
                "add the H2 \"Which file to read\" after the H1 (kind index",
            ),
            (
                "docs/specs/SKILL.md",
                "---\nname: specs\ndescription: D.\ntags: [x]\n---\n\n# S\n\n## Which file to read\n\n| You want to | Read |\n| --- | --- |\n| Open it | nothing |\n\n[[specs/sample.spec]]\n".into(),
                "E_KIND",
                "give this row of \"Which file to read\" a wikilink",
            ),
            (
                "docs/glossary.md",
                "---\ndescription: D.\ntags: [x]\n---\n\n# G\n\n## P\n\n**Screen**: One.\n\n**Screen**: Two.\n\nLoose prose.\n".into(),
                "E_ITEM",
                "Screen is already defined at line",
            ),
            (
                "docs/screens/stray.md",
                "---\ndescription: D.\ntags: [x]\n---\n\n# S\n".into(),
                "E_PLACE",
                "rename to stray.screen.md: docs/screens/ holds only *.screen.md pages",
            ),
        ];
        for (path, text, code, fragment) in cases {
            let before = fs::read_to_string(root.join(path)).ok();
            fs::write(root.join(path), &text).unwrap();
            let found = findings(root, path);
            expect(&found, code, fragment);
            match before {
                Some(b) => fs::write(root.join(path), b).unwrap(),
                None => fs::remove_file(root.join(path)).unwrap(),
            }
        }
        assert_eq!(report(&cli(root, &["check", "--json"]), 0)["total"], 0);
    }

    /// Several findings of one broken page: the rules page names every missing piece.
    #[test]
    fn rules_page_lists_each_repair() {
        let temp = fixture();
        let root = temp.path();
        let page = "docs/best-practices/sample.rules.md";
        fs::write(
            root.join(page),
            "---\ndescription: D.\ntags: [x]\nowner: me\n---\n\n# P\n\n### BP-SAMPLE-001: One\n\nSee [x](https://example.invalid).\nCheck: sometimes\n\n### Loose heading\n\n### BP-SAMPLE-002: Two\n\nWhy: w.\nCheck: lint\n",
        )
        .unwrap();
        let found = findings(root, page);
        expect(
            &found,
            "E_META",
            "remove the frontmatter key owner; allowed: description, tags",
        );
        expect(
            &found,
            "E_ITEM",
            "add the line \"Why: <the reason>\" under \"BP-SAMPLE-001: One\"",
        );
        expect(&found, "E_ITEM", "add the line \"Check: lint|test|review\"");
        expect(
            &found,
            "E_ITEM",
            "rename the heading \"Loose heading\" to ### BP-<AREA>-NNN: <rule>",
        );
        expect(&found, "E_ITEM", "write the text of \"BP-SAMPLE-002: Two\"");
        expect(&found, "E_KIND", "add a final H2 \"References\"");
        let adr = "docs/adr/0013-links.adr.md";
        fs::write(
            root.join(adr),
            "---\ndescription: D.\ntags: [x]\n---\n\n# A\n\n- list first\n\n# B\n\n## Consequences\n\nx\n\n## Considered options\n\ny\n",
        )
        .unwrap();
        let found = findings(root, adr);
        expect(&found, "E_META", "add the frontmatter key status (kind adr");
        expect(&found, "E_KIND", "keep exactly one H1 title");
        expect(
            &found,
            "E_KIND",
            "start the body with one paragraph right after the H1",
        );
        expect(
            &found,
            "E_KIND",
            "move the H2 \"Considered options\" above \"Consequences\"",
        );
    }

    /// Ids are unique across the files of a kind; a template carries headings but skips content checks.
    #[test]
    fn unique_ids_and_templates() {
        let temp = fixture();
        let root = temp.path();
        fs::write(
            root.join("docs/invariants/tooling.invariants.md"),
            "---\ndescription: D.\ntags: [x]\n---\n\n# T\n\n## INV-TOOLING-001: Fast\n\nActive · Decided: 2026-11-26, owner\n\nFast.\n\n## INV-TOOLING-001: Again\n\nRetired · Decided: 2026-11-26, owner\n\nAgain.\n",
        )
        .unwrap();
        let found = findings(root, "docs/invariants/tooling.invariants.md");
        expect(
            &found,
            "E_ITEM",
            "INV-TOOLING-001 is already used at docs/invariants/tooling.invariants.md:8; give this item the next free number",
        );
        let mut headings = String::from("---\ndescription: T.\ntags: [x]\n---\n\n# Template\n");
        for h in [
            "Name",
            "Description",
            "View",
            "Data",
            "States",
            "Lifecycle",
            "Props",
            "Events",
            "Hooks and callbacks",
            "Behaviours",
            "Navigation",
            "Input and accessibility",
            "Covers",
        ] {
            headings.push_str("\n## ");
            headings.push_str(h);
            headings.push('\n');
        }
        fs::write(root.join("docs/screens/_template.screen.md"), &headings).unwrap();
        let found = findings(root, "docs/screens/_template.screen.md");
        assert_eq!(found.len(), 1, "{found:?}");
        expect(&found, "E_INDEX", "Add [[screens/_template.screen]]");
        fs::write(
            root.join("docs/screens/_template.screen.md"),
            headings.replace("\n## Props\n", ""),
        )
        .unwrap();
        expect(
            &findings(root, "docs/screens/_template.screen.md"),
            "E_KIND",
            "add the H2 \"Props\" after \"Lifecycle\"",
        );
    }

    /// `kinds` prints one row per kind; the allowance skips a folder; a kinds page heading is required.
    #[test]
    fn kinds_command_and_allowance() {
        let temp = fixture();
        let root = temp.path();
        let rows = report(&cli(root, &["kinds", "--json"]), 0);
        let names: Vec<_> = rows["rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r.as_str().unwrap().split(' ').next().unwrap().to_owned())
            .collect();
        assert_eq!(
            names,
            [
                "screen",
                "adr",
                "invariants",
                "old",
                "spec",
                "tool",
                "rules",
                "index",
                "glossary"
            ]
        );
        let kinds = root.join("tooling/doc-links/kinds.toml");
        let text = fs::read_to_string(&kinds).unwrap();
        fs::write(
            root.join("docs/best-practices/loose.md"),
            "---\ndescription: D.\ntags: [x]\n---\n# L\n",
        )
        .unwrap();
        expect(
            &findings(root, "docs/best-practices/loose.md"),
            "E_PLACE",
            "rename to loose.rules.md",
        );
        fs::write(
            &kinds,
            text.replace(
                "pending_migration = []",
                "pending_migration = [\"docs/best-practices\"]",
            ),
        )
        .unwrap();
        let only_index = findings(root, "docs/best-practices/loose.md");
        assert_eq!(only_index.len(), 1, "{only_index:?}");
        expect(&only_index, "E_INDEX", "best-practices/loose");
        fs::remove_file(root.join("docs/best-practices/loose.md")).unwrap();
        fs::write(
            &kinds,
            text.replace("name = \"glossary\"", "name = \"lexicon\""),
        )
        .unwrap();
        let found = findings(root, "tooling");
        expect(
            &found,
            "E_KINDS",
            "add a heading \"lexicon\" to docs/repo/doc-kinds.md",
        );
        fs::write(&kinds, "kind = 3").unwrap();
        let broken = cli(root, &["check"]);
        assert_eq!(broken.status.code(), Some(1));
        assert!(
            String::from_utf8_lossy(&broken.stderr).contains("E_RUN tooling/doc-links/kinds.toml")
        );
    }

    /// `commit: pending` passes while the entry is uncommitted or added by HEAD, and blocks one commit later.
    #[test]
    fn inv_docs_004_pending_commit_expires() {
        let temp = fixture();
        let root = temp.path();
        let git = |args: &[&str]| {
            let out = Command::new("git")
                .args([
                    "-c",
                    "user.name=Fixture",
                    "-c",
                    "user.email=fixture@example.invalid",
                ])
                .args(args)
                .current_dir(root)
                .output()
                .unwrap();
            assert!(
                out.status.success(),
                "{}",
                String::from_utf8_lossy(&out.stderr)
            );
        };
        let old = "docs/repo/.old/earlier.2026-11-26.old.md";
        fs::write(
            root.join(old),
            "---\nsuperseded_by: \"[[repo/doc-links]]\"\ncommit: pending\n---\n# E\n",
        )
        .unwrap();
        assert_eq!(report(&cli(root, &["check", old, "--json"]), 0)["total"], 0);
        git(&["init", "-q"]);
        git(&["add", "-A"]);
        git(&["commit", "-q", "-m", "base"]);
        assert_eq!(report(&cli(root, &["check", old, "--json"]), 0)["total"], 0);
        fs::write(
            root.join("docs/glossary.md"),
            fs::read_to_string(root.join("docs/glossary.md")).unwrap()
                + "\n**Media center**: The device.\n",
        )
        .unwrap();
        git(&["commit", "-q", "-am", "next"]);
        expect(
            &findings(root, old),
            "E_PENDING",
            "replace commit: pending with the hash from git log --diff-filter=A",
        );
    }

    /// Write an executable script under `root/bin/<name>`.
    fn write_bin(root: &Path, name: &str) {
        let path = root.join("bin").join(name);
        fs::write(&path, "#!/bin/sh\necho ok\n").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        }
    }

    /// Populate a clean `docs/tooling/` scope with one workspace tool and one `bin/` tool.
    fn setup_tool_fixture(root: &Path) -> (String, String) {
        fs::create_dir_all(root.join("tooling/scratch")).unwrap();
        fs::create_dir_all(root.join("bin")).unwrap();
        fs::write(root.join("bin/.gitkeep"), "").unwrap();
        fs::create_dir_all(root.join("tooling/demo")).unwrap();
        fs::write(
            root.join("tooling/demo/Cargo.toml"),
            "[package]\nname = \"demo\"\n",
        )
        .unwrap();
        fs::write(
            root.join("tooling/demo/README.md"),
            "# demo\n\nRuns demo.\n",
        )
        .unwrap();
        write_bin(root, "demo");
        write_bin(root, "helper");
        fs::create_dir_all(root.join("docs/tooling")).unwrap();
        fs::write(
            root.join("docs/SKILL.md"),
            format!(
                "{}\n| Tools | [[tooling/SKILL]] |\n",
                fs::read_to_string(root.join("docs/SKILL.md")).unwrap()
            ),
        )
        .unwrap();
        fs::write(
            root.join("docs/tooling/SKILL.md"),
            "---\nname: tooling\ndescription: Tools.\ntags: [tooling, index]\n---\n\n# Tooling\n\n## Which file to read\n\n| Tool | Package | Language | Does | Read |\n| --- | --- | --- | --- | --- |\n| demo | `@yaai/demo` | Rust | Demo | [[tooling/demo.tool]] |\n| helper | none | POSIX sh | Helper | [[tooling/helper.tool]] |\n",
        )
        .unwrap();
        let demo_doc = "---\ndescription: Demo tool.\ntags: [tooling]\n---\n\n# demo\n\n## Synopsis\n\n`bin/demo`\n\n## Commands\n\n| Command | Does |\n| --- | --- |\n| `run` | Runs |\n\n## Options\n\n| Option | Default | Meaning |\n| --- | --- | --- |\n| `-h` | off | Help |\n\n## Exit codes\n\n- `0`: ok\n\n## Output\n\n```text\nok\n```\n\n## Covers\n\n- `tooling/demo/`\n- `bin/demo`\n".to_owned();
        let helper_doc = demo_doc
            .replace("demo", "helper")
            .replace("- `tooling/helper/`\n", "");
        fs::write(root.join("docs/tooling/demo.tool.md"), &demo_doc).unwrap();
        fs::write(root.join("docs/tooling/helper.tool.md"), &helper_doc).unwrap();
        (demo_doc, helper_doc)
    }

    /// Tool workspaces and `bin/` executables require `.tool.md` docs with `Covers`, non-empty READMEs, and no orphans.
    /// INV-TOOLING-006
    /// INV-TOOLING-007
    #[test]
    fn inv_tooling_006_and_007_tool_registration() {
        let temp = fixture();
        let root = temp.path();
        let (demo_doc, helper_doc) = setup_tool_fixture(root);

        assert_eq!(report(&cli(root, &["check", "--json"]), 0)["total"], 0);

        fs::remove_file(root.join("tooling/demo/README.md")).unwrap();
        expect(
            &findings(root, "tooling/demo"),
            "E_TOOL_README",
            "Create a non-empty tooling/demo/README.md",
        );
        fs::write(root.join("tooling/demo/README.md"), "   \n").unwrap();
        expect(
            &findings(root, "tooling/demo"),
            "E_TOOL_README",
            "Create a non-empty tooling/demo/README.md",
        );
        fs::write(
            root.join("tooling/demo/README.md"),
            "# demo\n\nRuns demo.\n",
        )
        .unwrap();

        fs::write(
            root.join("docs/tooling/demo.tool.md"),
            demo_doc.replace("- `bin/demo`\n", ""),
        )
        .unwrap();
        expect(
            &findings(root, "docs/tooling/demo.tool.md"),
            "E_TOOL_COVERS",
            "Add `bin/demo` under ## Covers",
        );
        fs::write(
            root.join("docs/tooling/demo.tool.md"),
            demo_doc.replace("## Exit codes\n\n- `0`: ok\n\n", ""),
        )
        .unwrap();
        expect(
            &findings(root, "docs/tooling/demo.tool.md"),
            "E_KIND",
            "add the H2 \"Exit codes\" after \"Options\" (kind tool, docs/repo/doc-kinds.md#tool)",
        );
        fs::write(root.join("docs/tooling/demo.tool.md"), &demo_doc).unwrap();

        fs::create_dir_all(root.join("tooling/extra")).unwrap();
        fs::write(
            root.join("tooling/extra/package.json"),
            "{\"name\":\"@yaai/extra\"}\n",
        )
        .unwrap();
        fs::write(root.join("tooling/extra/README.md"), "# extra\n\nExtra.\n").unwrap();
        expect(
            &findings(root, "tooling/extra"),
            "E_TOOL_DOC",
            "Create docs/tooling/extra.tool.md",
        );
        fs::remove_dir_all(root.join("tooling/extra")).unwrap();

        write_bin(root, "extra");
        expect(
            &findings(root, "bin/extra"),
            "E_TOOL_DOC",
            "Create docs/tooling/extra.tool.md",
        );
        fs::remove_file(root.join("bin/extra")).unwrap();

        fs::write(
            root.join("docs/tooling/ghost.tool.md"),
            helper_doc
                .replace("helper", "ghost")
                .replace("`bin/ghost`", "`bin/demo`"),
        )
        .unwrap();
        expect(
            &findings(root, "docs/tooling/ghost.tool.md"),
            "E_TOOL_ORPHAN",
            "Remove orphan docs/tooling/ghost.tool.md",
        );
        fs::remove_file(root.join("docs/tooling/ghost.tool.md")).unwrap();

        assert_eq!(report(&cli(root, &["check", "--json"]), 0)["total"], 0);
    }
}
