//! Apply source and manifest policies to the selected checkout paths.
//! spec: docs/repo/lint-rules.md#bindings
use super::{Catalog, Failure, Finding, details, references};
use serde_json::Value;
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use yaai_lints::{
    ci::commands, code_organization::exports, configuration::environment, rust::inherit,
};
use yaai_lints::{dependencies::tracked_files, security::tracked_env};

/// List Git's project files, including new files and excluding ignored build output.
pub(super) fn files(args: &[&str]) -> Result<Vec<PathBuf>, Failure> {
    let output = Command::new("git").args(args).output()?;
    if !output.status.success() {
        return Err(details(&output).into());
    }
    Ok(std::str::from_utf8(&output.stdout)?
        .split('\0')
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .collect())
}

/// Add a rule violation to the report.
fn report(
    out: &mut Vec<Finding>,
    catalog: &Catalog,
    path: &Path,
    line: usize,
    key: &str,
    message: &str,
) {
    out.push(Finding {
        path: path.display().to_string(),
        line: line.try_into().unwrap_or(u64::MAX),
        lint: key.into(),
        message: message.into(),
        rules: references(catalog, key, ""),
    });
}

/// Validate package exports and literal package scripts.
fn package(path: &Path, catalog: &Catalog, out: &mut Vec<Finding>) -> Result<(), Failure> {
    let value: Value = serde_json::from_slice(&fs::read(path)?)?;
    if path.starts_with("packages")
        && !exports::granular(&value)
        && !path.with_file_name("Cargo.toml").exists()
    {
        report(
            out,
            catalog,
            path,
            1,
            "yaai::granular-exports",
            "The package has no subpath exports. Expose each supported module through an explicit entry or wildcard.",
        );
    }
    if let Some(scripts) = value.get("scripts").and_then(Value::as_object) {
        for script in scripts.values().filter_map(Value::as_str) {
            for (lint, message) in commands::violations(script, false) {
                report(out, catalog, path, 1, lint, message);
            }
        }
    }
    Ok(())
}

/// Check an environment read against its workspace template and task declarations.
fn rust_source(
    path: &Path,
    turbo: &Value,
    catalog: &Catalog,
    out: &mut Vec<Finding>,
) -> Result<(), Failure> {
    let source = fs::read_to_string(path)?;
    let syntax = yaai_lints::parse(&source)?;
    let Some(dir) = path
        .ancestors()
        .skip(1)
        .find(|dir| dir.join("Cargo.toml").is_file())
    else {
        return Ok(());
    };
    for read in environment::reads(&syntax) {
        let template = dir.join(".env.example");
        let entries = if template.exists() {
            fs::read_to_string(template)?
        } else {
            String::new()
        };
        if !entries
            .lines()
            .filter_map(|l| l.split_once('='))
            .any(|(key, _)| key.trim() == read.name)
        {
            report(
                out,
                catalog,
                path,
                read.line,
                "yaai::env-template",
                &format!(
                    "{} is missing from the workspace template. Add a documented .env.example entry.",
                    read.name
                ),
            );
        }
        let globals = ["globalEnv", "globalPassThroughEnv"]
            .into_iter()
            .filter_map(|k| turbo.get(k));
        let locals = turbo
            .get("tasks")
            .and_then(Value::as_object)
            .into_iter()
            .flat_map(|tasks| tasks.values())
            .flat_map(|task| {
                ["env", "passThroughEnv"]
                    .into_iter()
                    .filter_map(|k| task.get(k))
            });
        if !globals
            .chain(locals)
            .filter_map(Value::as_array)
            .flatten()
            .any(|v| v.as_str() == Some(read.name.as_str()))
        {
            report(
                out,
                catalog,
                path,
                read.line,
                "yaai::task-env",
                &format!(
                    "{} has no task declaration. Declare it in the consuming Turborepo task.",
                    read.name
                ),
            );
        }
    }
    if (path == dir.join("src/main.rs")
        || path.parent().is_some_and(|d| d.ends_with("src/bin"))
        || path.starts_with(dir.join("src/bin")) && path.ends_with("main.rs"))
        && let Some(line) = yaai_lints::missing_status(&syntax)
    {
        report(
            out,
            catalog,
            path,
            line,
            "yaai::main-exit-code",
            "The binary entry point returns unit. Return ExitCode or a Result.",
        );
    }
    Ok(())
}

/// Check tracked environment files, vendored changes and required lockfiles.
fn git_policy(paths: &[PathBuf], catalog: &Catalog, out: &mut Vec<Finding>) -> Result<(), Failure> {
    let selected = |p: &Path| paths.is_empty() || paths.iter().any(|s| s == p);
    let tracked = files(&[
        "ls-files",
        "-z",
        "--",
        "**/.env*",
        ".env*",
        "Cargo.lock",
        "pnpm-lock.yaml",
    ])?;
    for path in tracked_files::missing_locks(Path::new("."), &tracked) {
        let p = path.as_path();
        if selected(p) {
            report(
                out,
                catalog,
                p,
                1,
                "yaai::tracked-lockfiles",
                "The repository lockfile is missing. Generate it and include it in version control.",
            );
        }
    }
    for p in tracked {
        if selected(&p) && tracked_env::local_values(&p) {
            report(
                out,
                catalog,
                &p,
                1,
                "yaai::tracked-env",
                "A local environment file is tracked. Keep values local and commit only the template.",
            );
        }
    }
    for p in tracked_files::vendor_changes(
        files(&["diff", "--name-only", "-z", "--", "vendor/"])?
            .into_iter()
            .chain(files(&[
                "diff",
                "--cached",
                "--name-only",
                "-z",
                "--",
                "vendor/",
            ])?),
    ) {
        if selected(&p) {
            report(
                out,
                catalog,
                &p,
                1,
                "yaai::vendor-changes",
                "A vendored source changed. Make the change upstream or in project-owned code.",
            );
        }
    }
    Ok(())
}

/// Run policies on project inputs, sharing the task configuration across files.
pub(super) fn check(
    paths: &[PathBuf],
    catalog: &Catalog,
    out: &mut Vec<Finding>,
) -> Result<(), Failure> {
    let inputs = if paths.is_empty() {
        files(&["ls-files", "-co", "--exclude-standard", "-z"])?
    } else {
        paths.to_vec()
    };
    super::directives::check(&inputs, catalog, out)?;
    let turbo = serde_json::from_slice(&fs::read("turbo.json")?)?;
    for path in inputs.into_iter().collect::<BTreeSet<_>>() {
        if !path.is_file() || path.starts_with("vendor") {
            continue;
        }
        if path.ends_with("Cargo.toml") && !inherit::workspace_policy(&fs::read_to_string(&path)?)?
        {
            report(
                out,
                catalog,
                &path,
                1,
                "yaai::workspace-lints",
                "The member bypasses workspace lint policy. Set workspace = true under [lints].",
            );
        } else if path.ends_with("package.json") {
            package(&path, catalog, out)?;
        } else if path.extension().is_some_and(|e| e == "rs") {
            rust_source(&path, &turbo, catalog, out)?;
        } else if path.starts_with(".github/workflows") {
            let workflow = commands::parse_workflow(&fs::read_to_string(&path)?)?;
            for script in commands::workflow_steps(&workflow) {
                for (lint, message) in commands::violations(script, true) {
                    report(out, catalog, &path, 1, lint, message);
                }
            }
        } else if path.starts_with(".husky") || path.starts_with("bin") {
            for (lint, message) in commands::violations(&fs::read_to_string(&path)?, false) {
                report(out, catalog, &path, 1, lint, message);
            }
        }
    }
    if paths.is_empty()
        || paths.iter().any(|p| {
            p.starts_with("vendor")
                || p.ends_with(".gitignore")
                || p.ends_with("Cargo.lock")
                || p.ends_with("pnpm-lock.yaml")
                || tracked_env::local_values(p)
        })
    {
        git_policy(paths, catalog, out)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    //! Validate actual Rust environment findings and repairs through the source checker.
    use super::{Catalog, rust_source};
    use serde_json::json;
    use std::fs;

    /// Both missing declarations cite the source line and disappear after repair.
    #[test]
    fn environment_contract() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        fs::write(root.join("Cargo.toml"), "[package]").unwrap();
        let source = root.join("clock.rs");
        fs::write(&source, "fn probe(){\nstd::env::var_os(\"CLOCK\");\n}").unwrap();
        let catalog: Catalog = serde_json::from_str(yaai_lints::BINDINGS).unwrap();
        let mut findings = Vec::new();
        rust_source(&source, &json!({}), &catalog, &mut findings).unwrap();
        assert_eq!(
            findings
                .iter()
                .map(|f| (f.line, f.lint.as_str()))
                .collect::<Vec<_>>(),
            [(2, "yaai::env-template"), (2, "yaai::task-env")]
        );
        assert!(findings.iter().all(|f| {
            f.rules
                .iter()
                .any(|r| r.contains("configuration.rules.md#BP-CONF-"))
        }));
        fs::write(root.join(".env.example"), "CLOCK=\n").unwrap();
        findings.clear();
        rust_source(
            &source,
            &json!({"tasks":{"lint":{"env":["CLOCK"]}}}),
            &catalog,
            &mut findings,
        )
        .unwrap();
        assert_eq!(findings, Vec::new());
    }
}
