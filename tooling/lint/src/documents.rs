//! Surface existing doc-links ownership and tool-registration diagnostics.
//! spec: docs/repo/lint-rules.md#bindings
use super::{Catalog, Failure, Finding, details, references};
use serde::Deserialize;
use std::{path::PathBuf, process::Command};

/// Machine-readable subset of the doc-links report.
#[derive(Deserialize)]
struct Report {
    /// Findings for the requested paths.
    findings: Vec<Row>,
}

/// One doc-links finding.
#[derive(Deserialize)]
struct Row {
    /// Repository-relative path.
    file: String,
    /// Source line.
    line: u64,
    /// Validator code.
    code: String,
    /// Requested action.
    message: String,
}

/// Collect the existing document checks bound to best-practice rules.
pub(super) fn check(
    paths: &[PathBuf],
    catalog: &Catalog,
    out: &mut Vec<Finding>,
) -> Result<(), Failure> {
    let output = Command::new("bin/doc-links")
        .args(["--json", "--limit", &usize::MAX.to_string(), "check"])
        .args(paths)
        .output()?;
    if !matches!(output.status.code(), Some(0 | 2)) {
        return Err(details(&output).into());
    }
    let report: Report = serde_json::from_slice(&output.stdout)?;
    for row in report.findings {
        let key = match row.code.as_str() {
            "E_TOOL_DOC" | "E_TOOL_COVERS" | "E_TOOL_README" | "E_TOOL_ORPHAN" => {
                "doc-links::E_TOOL_DOC"
            }
            "E_CITE" => "doc-links::E_CITE",
            _ => continue,
        };
        out.push(Finding {
            path: row.file,
            line: row.line,
            lint: format!("doc-links::{}", row.code),
            message: row.message,
            rules: references(catalog, key, ""),
        });
    }
    Ok(())
}
