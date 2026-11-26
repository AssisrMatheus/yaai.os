//! Exercise all three process outcomes in an isolated checkout.
//! spec: docs/tooling/lint.tool.md#exit-codes
use serde_json::Value;
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};
use tempfile::tempdir;

/// Invoke the built binary with machine output enabled.
fn check(root: &Path, file: &str) -> std::io::Result<Output> {
    Command::new(env!("CARGO_BIN_EXE_yaai-lint"))
        .args(["--json", file])
        .current_dir(root)
        .output()
}

/// Validate stdout, stderr and exit status for a clean workflow, a bad step and a failed read.
#[test]
fn workflow_exit_contract() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    assert!(
        Command::new("git")
            .args(["init", "-q"])
            .current_dir(root)
            .status()
            .unwrap()
            .success()
    );
    fs::write(root.join("turbo.json"), "{}").unwrap();
    fs::create_dir_all(root.join(".github/workflows")).unwrap();
    let file = ".github/workflows/build.yml";
    let workflow = |command: &str| format!("jobs:\n  build:\n    steps:\n      - run: {command}\n");
    fs::write(root.join(file), workflow("cargo build --locked")).unwrap();
    let clean = check(root, file).unwrap();
    assert_eq!(clean.status.code(), Some(0));
    let clean_report: Value = serde_json::from_slice(&clean.stdout).unwrap();
    assert_eq!(clean_report["total"], 0);
    assert_eq!(clean.stderr, b"lint findings=0\n");
    fs::write(root.join(file), workflow("cargo build")).unwrap();
    let invalid = check(root, file).unwrap();
    assert_eq!(invalid.status.code(), Some(2));
    let report: Value = serde_json::from_slice(&invalid.stdout).unwrap();
    assert_eq!(report["total"], 1);
    assert_eq!(report["findings"][0]["lint"], "yaai::frozen-install");
    assert!(
        String::from_utf8(invalid.stderr)
            .unwrap()
            .contains("Rule: docs/best-practices/ci.rules.md#BP-CI-018")
    );
    let failure = check(root, "missing.yml").unwrap();
    assert_eq!(failure.status.code(), Some(1));
    assert_eq!(failure.stdout, Vec::<u8>::new());
    assert!(
        String::from_utf8(failure.stderr)
            .unwrap()
            .contains("lint failed")
    );
}
