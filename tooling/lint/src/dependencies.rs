//! Delegate dependency analysis to its existing tools.
//! spec: docs/repo/lint-rules.md#bindings
use super::{Catalog, Failure, Finding, details, references};
use serde_json::Value;
use std::{path::Path, process::Command};

/// Check advisories and license policy using Cargo's resolved graph.
pub(super) fn check(catalog: &Catalog, findings: &mut Vec<Finding>) -> Result<(), Failure> {
    let output = Command::new("cargo")
        .args([
            "deny",
            "--config",
            "packages/lints/cargo-deny.toml",
            "--format",
            "json",
            "check",
        ])
        .output()?;
    let mut rejected = false;
    for line in std::str::from_utf8(&output.stderr)?.lines() {
        let report: Value =
            serde_json::from_str(line).map_err(|e| format!("{e}: {}", details(&output)))?;
        if report.pointer("/fields/severity").and_then(Value::as_str) != Some("error") {
            continue;
        }
        let code = report
            .pointer("/fields/code")
            .and_then(Value::as_str)
            .unwrap_or("policy");
        let package = report
            .pointer("/fields/graphs/0/Krate/name")
            .and_then(Value::as_str)
            .unwrap_or("dependency");
        let message = report
            .pointer("/fields/message")
            .and_then(Value::as_str)
            .unwrap_or("Dependency failed policy");
        findings.push(Finding {
            path: "Cargo.lock".into(),
            line: 1,
            lint: format!("cargo-deny::{code}"),
            message: format!("{package}: {message}. Update the dependency or its source."),
            rules: references(catalog, "cargo-deny::check", ""),
        });
        rejected = true;
    }
    if !output.status.success() && !rejected {
        return Err(details(&output).into());
    }
    let mut cmd = Command::new("cargo");
    let _ = cmd.arg("machete");
    for directory in ["apps", "packages", "tooling"] {
        if Path::new(directory).is_dir() {
            let _ = cmd.arg(directory);
        }
    }
    let output = cmd.output()?;
    match output.status.code() {
        Some(0) => Ok(()),
        Some(1) => {
            let mut manifest = None;
            let mut found = false;
            for line in std::str::from_utf8(&output.stdout)?.lines() {
                if let Some((_, file)) = line.strip_suffix(':').and_then(|l| l.split_once(" -- ")) {
                    manifest = Some(file);
                } else if let Some(name) = line.strip_prefix('\t')
                    && let Some(file) = manifest
                {
                    let text = std::fs::read_to_string(file)?;
                    let row = text
                        .lines()
                        .position(|l| {
                            l.split_once('=')
                                .is_some_and(|(key, _)| key.trim().trim_matches('"') == name)
                        })
                        .map_or(1, |i| i.saturating_add(1));
                    findings.push(Finding {
                        path: file.to_owned(),
                        line: row.try_into()?,
                        lint: "cargo-machete::unused".into(),
                        message: format!(
                            "{name} is unused. Remove the declaration from this crate."
                        ),
                        rules: references(catalog, "cargo-machete::unused", ""),
                    });
                    found = true;
                }
            }
            if found {
                Ok(())
            } else {
                Err(details(&output).into())
            }
        }
        _ => Err(details(&output).into()),
    }
}

/// Run the installed JavaScript dependency checker on workspace manifests.
pub(super) fn javascript(catalog: &Catalog, findings: &mut Vec<Finding>) -> Result<(), Failure> {
    let output = Command::new("node_modules/.bin/knip")
        .args([
            "--include",
            "dependencies,devDependencies",
            "--reporter",
            "json",
            "--cache",
        ])
        .output()?;
    let report: Value =
        serde_json::from_slice(&output.stdout).map_err(|e| format!("{e}: {}", details(&output)))?;
    let issues = report
        .get("issues")
        .and_then(Value::as_array)
        .ok_or("missing Knip issues")?;
    if issues.is_empty() && !output.status.success() {
        return Err(details(&output).into());
    }
    for issue in issues {
        let path = issue
            .get("file")
            .and_then(Value::as_str)
            .ok_or("missing Knip manifest")?;
        for dependency in ["dependencies", "devDependencies"]
            .iter()
            .filter_map(|k| issue.get(k)?.as_array())
            .flatten()
        {
            let name = dependency
                .get("name")
                .and_then(Value::as_str)
                .ok_or("missing dependency name")?;
            findings.push(Finding {
                path: path.into(),
                line: dependency.get("line").and_then(Value::as_u64).unwrap_or(1),
                lint: "knip::unused".into(),
                message: format!("{name} is unused. Remove the declaration from this workspace."),
                rules: references(catalog, "knip::unused", ""),
            });
        }
    }
    Ok(())
}
