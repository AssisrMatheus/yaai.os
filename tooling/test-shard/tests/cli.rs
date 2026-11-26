//! Exercise the agent-facing output and status contract.
//! spec: docs/tooling/test-shard.tool.md#output
use std::process::Command;

/// Empty JSON is machine-readable, invalid shard numbers require repair and help succeeds.
#[test]
fn cli_contract() {
    let mut command = Command::new(env!("CARGO_BIN_EXE_test-shard"));
    let _ = command.args([
        "--timings",
        "../../.github/ci/timings.json",
        "--count",
        "2",
        "--shard",
    ]);
    let empty = command.arg("1").arg("--json").output().unwrap();
    assert!(empty.status.success());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&empty.stdout).unwrap(),
        serde_json::json!({"files":[]})
    );
    let invalid = Command::new(env!("CARGO_BIN_EXE_test-shard"))
        .args(["--shard", "0"])
        .output()
        .unwrap();
    assert_eq!(invalid.status.code(), Some(2));
    let missing = Command::new(env!("CARGO_BIN_EXE_test-shard"))
        .args([
            "--timings",
            "absent-timings.json",
            "--count",
            "1",
            "--shard",
            "1",
        ])
        .output()
        .unwrap();
    assert_eq!(missing.status.code(), Some(2));
    let help = Command::new(env!("CARGO_BIN_EXE_test-shard"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(help.status.success());
}
