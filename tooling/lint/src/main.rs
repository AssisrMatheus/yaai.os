//! Run workspace checks and annotate diagnostics.
//! spec: docs/repo/lint-rules.md#diagnostics
mod bindings;
mod dependencies;
mod directives;
mod documents;
mod format;
mod logging;
mod repository;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    error::Error,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, ExitCode, Output},
};

/// Check source files against the best-practice rules.
#[derive(Parser)]
struct Args {
    /// Log backend execution as JSON on stderr.
    #[arg(short, long)]
    verbose: bool,
    /// Emit the findings as JSON.
    #[arg(long)]
    json: bool,
    /// Repository-relative source files. Omit to check the entire workspace.
    paths: Vec<PathBuf>,
}

/// A diagnostic with a stable best-practice citation.
#[derive(Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct Finding {
    /// Repository-relative file.
    path: String,
    /// One-based source line.
    line: u64,
    /// Diagnostic identifier.
    lint: String,
    /// Diagnostic text.
    message: String,
    /// Best-practice references.
    rules: Vec<String>,
}

/// Errors that must abort a lint run.
type Failure = Box<dyn Error>;

/// Lookup table from lint name to stable references.
type Catalog = BTreeMap<String, Vec<String>>;

/// Retrieve the exact binding or its governing lint group.
fn references(catalog: &Catalog, key: &str, group: &str) -> Vec<String> {
    catalog
        .get(key)
        .or_else(|| catalog.get(group))
        .cloned()
        .unwrap_or_default()
}

/// Oxlint machine output envelope.
#[derive(Deserialize)]
struct OxOutput {
    /// Findings from stock and JS rules.
    diagnostics: Vec<OxDiagnostic>,
}

/// Required diagnostic fields in oxlint JSON.
#[derive(Deserialize)]
struct OxDiagnostic {
    /// Human-readable explanation.
    message: String,
    /// Lint plugin and rule.
    code: String,
    /// Path to the source.
    filename: String,
    /// Reported locations.
    labels: Vec<Value>,
}

/// Run oxlint and translate its machine diagnostics.
fn oxlint(
    paths: &[PathBuf],
    catalog: &Catalog,
    findings: &mut Vec<Finding>,
) -> Result<(), Failure> {
    let output = Command::new("node_modules/.bin/oxlint")
        .args([
            "--type-aware",
            "--type-check",
            "--format",
            "json",
            "--deny-warnings",
            "--",
        ])
        .args(paths)
        .output()?;
    let report: OxOutput =
        serde_json::from_slice(&output.stdout).map_err(|e| format!("{e}: {}", details(&output)))?;
    if !output.status.success() && report.diagnostics.is_empty() {
        return Err(details(&output).into());
    }
    for diagnostic in report.diagnostics {
        let lint = diagnostic
            .code
            .trim_end_matches(')')
            .replace('(', "/")
            .replace("jsx_a11y/", "jsx-a11y/");
        let group = if lint.starts_with("typescript/TS") {
            "oxlint::type-check"
        } else {
            "oxlint::categories"
        };
        let rules = references(catalog, &format!("oxlint::{lint}"), group);
        let message = diagnostic
            .message
            .split(" Rule: ")
            .next()
            .unwrap_or_default()
            .to_owned();
        findings.push(Finding {
            path: diagnostic.filename,
            line: diagnostic
                .labels
                .first()
                .and_then(|l| l.pointer("/span/line"))
                .and_then(Value::as_u64)
                .unwrap_or(1),
            lint,
            message,
            rules,
        });
    }
    Ok(())
}

/// Explain a failed dependency, including diagnostics printed on stdout.
fn details(output: &Output) -> String {
    format!(
        "{} {} {}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    )
}

/// Collect Clippy messages. A build failure without a lint code is a tool failure.
fn clippy(
    paths: &[PathBuf],
    catalog: &Catalog,
    findings: &mut Vec<Finding>,
) -> Result<(), Failure> {
    let mut cmd = Command::new("cargo");
    let _ = cmd.args([
        "clippy",
        "--locked",
        "--all-targets",
        "--keep-going",
        "--message-format=json",
    ]);
    if paths.is_empty() {
        let _ = cmd.arg("--workspace");
    } else {
        let manifests = paths
            .iter()
            .filter_map(|p| {
                p.ancestors()
                    .skip(1)
                    .map(|d| d.join("Cargo.toml"))
                    .find(|m| m.is_file())
            })
            .collect::<std::collections::BTreeSet<_>>();
        if manifests.len() == 1
            && let Some(m) = manifests.first()
        {
            let _ = cmd.arg("--manifest-path").arg(m);
        } else {
            let _ = cmd.arg("--workspace");
        }
    }
    let output = cmd.args(["--", "-D", "warnings"]).output()?;
    cargo_diagnostics(&output, paths, catalog, findings)
}

/// Translate JSON compiler messages from Clippy and rustdoc.
fn cargo_diagnostics(
    output: &Output,
    paths: &[PathBuf],
    catalog: &Catalog,
    findings: &mut Vec<Finding>,
) -> Result<(), Failure> {
    let mut diagnosed = false;
    for line in std::str::from_utf8(&output.stdout)?.lines() {
        let obj: Value = serde_json::from_str(line)?;
        if obj.get("reason").and_then(Value::as_str) != Some("compiler-message") {
            continue;
        }
        let msg = obj.get("message").ok_or("missing compiler diagnostic")?;
        let Some(code) = msg.pointer("/code/code").and_then(Value::as_str) else {
            if msg["level"] == "error" {
                return Err(msg["message"].to_string().into());
            }
            continue;
        };
        if code.starts_with('E') {
            return Err(msg["rendered"].to_string().into());
        }
        diagnosed = true;
        let Some(span) = msg["spans"]
            .as_array()
            .and_then(|a| a.iter().find(|s| s["is_primary"] == true))
        else {
            continue;
        };
        let path = span["file_name"]
            .as_str()
            .ok_or("missing diagnostic path")?;
        if !paths.is_empty() && !paths.iter().any(|p| p == Path::new(path)) {
            continue;
        }
        let key = if code.contains("::") {
            code.to_owned()
        } else {
            format!("rustc::{code}")
        };
        findings.push(Finding {
            path: path.to_owned(),
            line: span["line_start"].as_u64().unwrap_or(1),
            lint: code.into(),
            message: msg["message"].as_str().unwrap_or_default().into(),
            rules: references(catalog, &key, "clippy::pedantic"),
        });
    }
    if !output.status.success() && !diagnosed {
        return Err(details(output).into());
    }
    Ok(())
}

/// Check custom Rust entry-point rules and collect backend diagnostics.
fn run(args: &Args) -> Result<bool, Failure> {
    let catalog: Catalog = serde_json::from_str(yaai_lints::BINDINGS)?;
    let mut findings = Vec::new();
    let check_bindings = args.paths.is_empty()
        || args.paths.iter().any(|p| {
            p.starts_with("docs/best-practices")
                || p.starts_with("packages/lints")
                || p.starts_with("packages/oxlint-config")
                || p.ends_with("Cargo.toml")
                || p.ends_with(".oxlintrc.json")
                || p.ends_with("clippy.toml")
                || p.ends_with("tsconfig.json")
        });
    if check_bindings && let Err(error) = bindings::check(&catalog) {
        if error.is::<bindings::Drift>() {
            findings.push(Finding {
                path: "packages/lints/bindings.json".into(),
                line: 1,
                lint: "lint::bindings".into(),
                message: error.to_string(),
                rules: vec!["docs/repo/lint-rules.md#bindings".into()],
            });
            return emit(args, findings);
        }
        return Err(error);
    }
    if args.paths.is_empty()
        || args.paths.iter().any(|p| {
            p.ends_with("Cargo.lock") || p.ends_with("Cargo.toml") || p.ends_with("cargo-deny.toml")
        })
    {
        tracing::debug!(scope = "lint.dependencies.rust", "backend started");
        dependencies::check(&catalog, &mut findings)?;
    }
    if args.paths.is_empty()
        || args
            .paths
            .iter()
            .any(|p| p.ends_with("package.json") || p.ends_with("pnpm-lock.yaml"))
    {
        tracing::debug!(scope = "lint.dependencies.javascript", "backend started");
        dependencies::javascript(&catalog, &mut findings)?;
    }
    let mut js = Vec::new();
    let mut rust = Vec::new();
    for path in &args.paths {
        if !path.is_file() {
            return Err(format!("source file missing: {}", path.display()).into());
        }
        match path.extension().and_then(|e| e.to_str()) {
            Some("ts" | "tsx" | "js" | "jsx" | "mts" | "cts" | "mjs" | "cjs") => {
                js.push(path.clone());
            }
            Some("rs") => rust.push(path.clone()),
            _ => {}
        }
    }
    if args.paths.is_empty() || !js.is_empty() {
        format::typescript(&js, &catalog, &mut findings)?;
        tracing::debug!(scope = "lint.oxlint", "backend started");
        oxlint(&js, &catalog, &mut findings)?;
    }
    if args.paths.is_empty() || !rust.is_empty() {
        format::rust(&rust, &catalog, &mut findings)?;
        tracing::debug!(scope = "lint.clippy", "backend started");
        clippy(&rust, &catalog, &mut findings)?;
    }
    if args.paths.is_empty() {
        let output = Command::new("cargo")
            .args([
                "doc",
                "--locked",
                "--workspace",
                "--no-deps",
                "--document-private-items",
                "--message-format=json",
            ])
            .env("RUSTDOCFLAGS", "-D warnings")
            .output()?;
        cargo_diagnostics(&output, &[], &catalog, &mut findings)?;
    }
    repository::check(&args.paths, &catalog, &mut findings)?;
    if args.paths.is_empty()
        || args
            .paths
            .iter()
            .any(|p| p.starts_with("docs") || p.starts_with("bin"))
    {
        tracing::debug!(scope = "lint.docs", "backend started");
        documents::check(&args.paths, &catalog, &mut findings)?;
    }
    emit(args, findings)
}

/// Keep machine results independent of actionable hook stderr.
fn emit(args: &Args, mut findings: Vec<Finding>) -> Result<bool, Failure> {
    tracing::info!(
        scope = "lint.cli",
        count = findings.len(),
        "check completed"
    );
    findings.sort();
    findings.dedup();
    let mut err = io::stderr().lock();
    for f in &findings {
        writeln!(
            err,
            "{}:{}: {} {} Rule: {}",
            f.path,
            f.line,
            f.lint,
            f.message,
            f.rules.join(", ")
        )?;
    }
    if args.json || !findings.is_empty() {
        writeln!(err, "lint findings={}", findings.len())?;
    } else {
        writeln!(io::stdout().lock(), "lint findings=0")?;
    }
    if args.json {
        serde_json::to_writer(
            io::stdout().lock(),
            &serde_json::json!({"total":findings.len(),"findings":findings}),
        )?;
    }
    Ok(!findings.is_empty())
}

/// Convert the lint contract to a process exit status.
fn main() -> ExitCode {
    let mut args = match Args::try_parse() {
        Ok(args) => args,
        Err(error) => {
            let help = matches!(
                error.kind(),
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion
            );
            let _ = error.print();
            return if help {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            };
        }
    };
    let result = (|| -> Result<bool, Failure> {
        logging::install(args.verbose)?;
        let span = tracing::info_span!(
            "lint",
            scope = "lint.cli",
            operation = "check",
            correlation_id = std::process::id()
        );
        let _scope = span.enter();
        tracing::info!(
            scope = "lint.cli",
            paths = args.paths.len(),
            "check started"
        );
        let root = std::env::current_dir()?.canonicalize()?;
        let mut inputs = Vec::new();
        for path in &args.paths {
            let absolute = path.canonicalize()?;
            let relative = absolute.strip_prefix(&root)?;
            if absolute.is_dir() {
                inputs.extend(
                    repository::files(&[
                        "ls-files",
                        "-co",
                        "--exclude-standard",
                        "-z",
                        "--",
                        relative.to_str().ok_or("invalid workspace path")?,
                    ])?
                    .into_iter()
                    .filter(|p| p.is_file()),
                );
            } else {
                inputs.push(relative.to_path_buf());
            }
        }
        if !args.paths.is_empty() && inputs.is_empty() {
            return Err("no project files at the requested paths".into());
        }
        args.paths = inputs;
        run(&args)
    })();
    match result {
        Ok(false) => ExitCode::SUCCESS,
        Ok(true) => ExitCode::from(2),
        Err(error) => {
            let _ = writeln!(io::stderr().lock(), "lint failed: {error}");
            ExitCode::FAILURE
        }
    }
}
