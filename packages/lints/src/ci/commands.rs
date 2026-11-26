//! Inspect literal commands in CI steps, hooks and package scripts.
//! spec: docs/repo/lint-rules.md#bindings
use serde_json::Value;

/// Return shell scripts in workflow run steps.
pub fn workflow_steps(value: &Value) -> Vec<&str> {
    value
        .get("jobs")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(|jobs| jobs.values())
        .filter_map(|job| job.get("steps")?.as_array())
        .flatten()
        .filter_map(|step| step.get("run")?.as_str())
        .collect()
}

/// Parse a workflow using the same YAML reader as the documentation checker.
///
/// # Errors
/// Returns the parser diagnostic for malformed workflows.
pub fn parse_workflow(source: &str) -> Result<Value, noyalib::Error> {
    noyalib::from_str(source)
}

/// Check direct shell invocations in automation; keep quoted examples and comments out.
pub fn violations(script: &str, ci: bool) -> Vec<(&'static str, &'static str)> {
    let mut findings = Vec::new();
    for line in script.lines() {
        let Ok(words) = shell_words::split(line) else {
            continue;
        };
        for command in words.split(|w| matches!(w.as_str(), "&&" | "||" | ";" | "|")) {
            let mut tokens = command
                .iter()
                .map(String::as_str)
                .skip_while(|w| w.contains('=') || matches!(*w, "exec" | "command"));
            let Some(program) = tokens.next() else {
                continue;
            };
            let arguments: Vec<_> = tokens.collect();
            let (exe, args) = match (program, arguments.as_slice()) {
                ("pnpm", ["exec", tool, remaining @ ..]) => (*tool, remaining),
                (_, remaining) => (program, remaining),
            };
            let first = args.first().copied();
            if exe == "git"
                && (args.contains(&"--no-verify")
                    || (first == Some("commit") && args.contains(&"-n")))
            {
                findings.push((
                    "yaai::verify-hooks",
                    "The command skips Git verification. Run the hooks and fix their findings.",
                ));
            }
            if matches!(exe, "npm" | "npx" | "yarn" | "bunx")
                || (exe == "bun" && matches!(first, Some("install" | "add" | "remove")))
            {
                findings.push(("yaai::package-manager","The command uses an unsupported package manager. Install with pnpm and run TypeScript with bun."));
            }
            if !ci {
                continue;
            }
            if (exe == "pnpm" && first == Some("install") && !args.contains(&"--frozen-lockfile"))
                || (exe == "cargo"
                    && matches!(first, Some("build" | "test" | "check" | "clippy"))
                    && !args.contains(&"--locked")
                    && !args.contains(&"--frozen"))
            {
                findings.push((
                    "yaai::frozen-install",
                    "CI can change the lockfile. Use pnpm --frozen-lockfile or cargo --locked.",
                ));
            }
            let formatting =
                exe == "oxfmt" || exe == "rustfmt" || (exe == "cargo" && first == Some("fmt"));
            let writes = if formatting {
                !args.contains(&"--check") && !args.contains(&"--list-different")
            } else {
                matches!(exe, "oxlint" | "cargo") && args.contains(&"--fix")
            };
            if writes && !script.contains("git diff --exit-code") {
                findings.push(("yaai::check-mode","The CI command modifies files without checking the diff. Use check mode or fail on changes."));
            }
        }
    }
    findings
}

#[cfg(test)]
mod tests {
    //! Exercise each supported command policy in both directions.
    use super::violations;
    /// Correct commands pass and each invalid command names the governing check.
    #[test]
    fn automation_rules() {
        for (bad, good, key) in [
            ("git commit --no-verify", "git commit", "yaai::verify-hooks"),
            (
                "npm install",
                "pnpm install --frozen-lockfile",
                "yaai::package-manager",
            ),
            (
                "cargo build",
                "cargo build --locked",
                "yaai::frozen-install",
            ),
            ("pnpm exec oxfmt", "oxfmt --check", "yaai::check-mode"),
            ("cargo fmt", "cargo fmt -- --check", "yaai::check-mode"),
            ("oxlint --fix", "oxlint --deny-warnings", "yaai::check-mode"),
        ] {
            assert!(violations(bad, true).iter().any(|(k, _)| *k == key));
            assert_eq!(violations(good, true), Vec::new());
        }
    }
}
