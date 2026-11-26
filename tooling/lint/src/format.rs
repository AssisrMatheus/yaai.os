//! Translate stock formatter output to cited findings.
//! spec: docs/repo/lint-rules.md#bindings
use super::{Catalog, Failure, Finding, details, references};
use std::{path::PathBuf, process::Command};

/// Report each file listed by oxfmt.
pub(super) fn typescript(
    paths: &[PathBuf],
    catalog: &Catalog,
    out: &mut Vec<Finding>,
) -> Result<(), Failure> {
    let output = Command::new("node_modules/.bin/oxfmt")
        .args(["--list-different", "--no-error-on-unmatched-pattern", "--"])
        .args(paths)
        .output()?;
    if output.status.success() {
        return Ok(());
    }
    let mut matched = false;
    for path in std::str::from_utf8(&output.stdout)?
        .lines()
        .filter(|s| std::path::Path::new(s).is_file())
    {
        matched = true;
        out.push(Finding {
            path: path.into(),
            line: 1,
            lint: "oxfmt::check".into(),
            message: "Formatting differs. Run oxfmt on this file.".into(),
            rules: references(catalog, "oxfmt::check", ""),
        });
    }
    if matched && output.status.code() == Some(1) {
        Ok(())
    } else {
        Err(details(&output).into())
    }
}

/// Use rustfmt check mode, parsing only the standard diff header.
pub(super) fn rust(
    paths: &[PathBuf],
    catalog: &Catalog,
    out: &mut Vec<Finding>,
) -> Result<(), Failure> {
    let output = if paths.is_empty() {
        Command::new("cargo")
            .args(["fmt", "--all", "--", "--check"])
            .output()?
    } else {
        Command::new("rustfmt")
            .args([
                "--edition",
                "2024",
                "--check",
                "--config",
                "skip_children=true",
            ])
            .args(paths)
            .output()?
    };
    if output.status.success() {
        return Ok(());
    }
    let root = std::env::current_dir()?;
    let mut matched = false;
    for header in std::str::from_utf8(&output.stdout)?
        .lines()
        .filter_map(|s| s.strip_prefix("Diff in "))
    {
        if let Some((file, number)) = header.trim_end_matches(':').rsplit_once(':') {
            let file = std::path::Path::new(file);
            out.push(Finding {
                path: file
                    .strip_prefix(&root)
                    .unwrap_or(file)
                    .display()
                    .to_string(),
                line: number.parse()?,
                lint: "rustfmt::check".into(),
                message: "Formatting differs. Run rustfmt on this file.".into(),
                rules: references(catalog, "rustfmt::check", ""),
            });
            matched = true;
        }
    }
    if matched {
        Ok(())
    } else {
        Err(details(&output).into())
    }
}
