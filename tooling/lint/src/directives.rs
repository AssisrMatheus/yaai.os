//! Check suppression directives even when a source file disables the entire JS plugin.
//! spec: docs/repo/lint-rules.md#bindings
use super::{Catalog, Failure, Finding, details, references};
use serde::Deserialize;
use std::{fs, path::PathBuf, process::Command};

/// Location and action returned by the TypeScript comment parser.
#[derive(Deserialize)]
struct Row {
    /// File containing the directive.
    path: String,
    /// Physical source line.
    line: u64,
    /// Action including the governing reference.
    message: String,
}

/// Only start the parser for files containing a possible directive.
pub(super) fn check(
    inputs: &[PathBuf],
    catalog: &Catalog,
    out: &mut Vec<Finding>,
) -> Result<(), Failure> {
    let mut candidates = Vec::new();
    for path in inputs.iter().filter(|p| {
        p.is_file()
            && p.extension().and_then(|e| e.to_str()).is_some_and(|e| {
                matches!(
                    e,
                    "ts" | "tsx" | "js" | "jsx" | "mts" | "cts" | "mjs" | "cjs"
                )
            })
    }) {
        let source = fs::read_to_string(path)?;
        if source.contains("oxlint-disable") || source.contains("eslint-disable") {
            candidates.push(path);
        }
    }
    if candidates.is_empty() {
        return Ok(());
    }
    let output = Command::new("bun")
        .arg("packages/oxlint-config/directives.ts")
        .args(candidates)
        .output()?;
    if !output.status.success() {
        return Err(details(&output).into());
    }
    for row in serde_json::from_slice::<Vec<Row>>(&output.stdout)? {
        out.push(Finding {
            path: row.path,
            line: row.line,
            lint: "yaai/suppression-reason".into(),
            message: row
                .message
                .split(" Rule: ")
                .next()
                .unwrap_or_default()
                .into(),
            rules: references(catalog, "oxlint::yaai/suppression-reason", ""),
        });
    }
    Ok(())
}
