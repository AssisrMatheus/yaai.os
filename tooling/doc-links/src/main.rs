//! Check code and documentation relationships from an agent-friendly CLI.
//! spec: docs/repo/doc-links.md#commands
use clap::{Parser, Subcommand, error::ErrorKind};
use serde::Serialize;
use serde_json::{Value, json};
use std::{
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::ExitCode,
    time::{Instant, SystemTime},
};

/// Cache each static pattern at its call site.
macro_rules! regex {
    ($pattern:literal) => {{
        /// Compiled citation or Markdown pattern.
        static RE: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| crate::pattern($pattern));
        &*RE
    }};
}
pub(crate) use regex;

/// Compile a literal; every pattern is exercised by the integration fixture.
#[expect(
    clippy::expect_used,
    reason = "Only static expressions compiled into the binary reach this function."
)]
fn pattern(src: &str) -> regex::Regex {
    regex::Regex::new(src).expect("valid static pattern")
}

/// Compare a repository file extension.
fn extension(file: &str, ext: &str) -> bool {
    Path::new(file)
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case(ext))
}

mod commit;
mod graph;
mod kinds;
mod markdown;
mod repo;
mod source;

/// Identify an archive directory by path component.
fn archived(path: &str) -> bool {
    path.split('/').any(|p| p == ".old")
}

/// An actionable diagnostic sorted by repository path and line.
#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct Finding {
    /// Repository-relative location.
    file: String,
    /// One-based line.
    line: usize,
    /// Stable diagnostic identifier.
    code: &'static str,
    /// Suggested repair.
    message: String,
}
impl Finding {
    /// Construct a diagnostic at its owner.
    fn new(file: &str, line: usize, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            file: file.to_owned(),
            line,
            code,
            message: message.into(),
        }
    }
}

/// Verify the links between code, tests and the documentation vault.
#[derive(Parser)]
#[command(name="doc-links",color=clap::ColorChoice::Never,after_help="Example: doc-links check docs/repo\nExit 0 completed, 2 findings, 1 usage or operational failure.")]
struct Args {
    /// Repository root (default: nearest enclosing repository).
    #[arg(long, global = true)]
    root: Option<PathBuf>,
    /// Print one JSON result on stdout; findings also go to stderr.
    #[arg(long, global = true)]
    json: bool,
    /// Enable structured diagnostic events on stderr (also `DOC_LINKS_LOG`).
    #[arg(short = 'v', global = true)]
    verbose: bool,
    /// Maximum displayed rows; counts include every result.
    #[arg(long, global = true, default_value_t = 50)]
    limit: usize,
    /// Operation to perform.
    #[command(subcommand)]
    command: Operation,
}

/// The repository operations.
#[derive(Subcommand)]
enum Operation {
    /// Validate links and encyclopedia shape; optionally filter by repo-relative paths.
    #[command(
        after_help = "Example: doc-links check docs/repo\nExit 0 clean, 2 findings, 1 failure."
    )]
    Check {
        /// Files or directory prefixes; omit for all.
        paths: Vec<PathBuf>,
    },
    /// List code, tests and docs related to a path or rule ID.
    #[command(
        after_help = "Example: doc-links refs docs/repo/doc-links.md\nExit 0 completed, 1 failure."
    )]
    Refs {
        /// Repository path, doc anchor, invariant ID or quoted ADR ID.
        query: String,
    },
    /// List invariants and governing pages that have no test citation.
    #[command(after_help = "Example: doc-links matrix --json\nExit 0 completed, 1 failure.")]
    Matrix,
    /// Replace extracted relationships in graphify-out/graph.json if present.
    #[command(after_help = "Example: doc-links graph\nExit 0 completed or skipped, 1 failure.")]
    Graph,
    /// Print each doc kind: its file pattern, folders and what it requires.
    #[command(after_help = "Example: doc-links kinds --json\nExit 0 completed, 1 failure.")]
    Kinds,
    /// Check staged files and require documentation for covered code changes.
    #[command(
        after_help = "Example: doc-links commit .git/COMMIT_EDITMSG\nExit 0 clean, 2 findings, 1 failure."
    )]
    Commit {
        /// Path to the commit message from the current directory.
        message_file: PathBuf,
    },
}

impl Operation {
    /// Stable name for reports and logs.
    fn name(&self) -> &'static str {
        match self {
            Self::Check { .. } => "check",
            Self::Refs { .. } => "refs",
            Self::Matrix => "matrix",
            Self::Graph => "graph",
            Self::Kinds => "kinds",
            Self::Commit { .. } => "commit",
        }
    }
}

/// Emit a scoped structured event, using only bounded tool-owned fields.
fn log(args: &Args, scope: &str, level: &str, event: &str, start: Instant, count: usize) {
    let filter = env::var("DOC_LINKS_LOG").unwrap_or_default().to_lowercase();
    let permits = |floor: &str| match floor {
        "debug" | "*" => true,
        "info" => level != "debug",
        "error" | "warn" => level == "error",
        _ => false,
    };
    if args.verbose
        || filter.split(',').map(str::trim).any(|t| {
            t.split_once(':').map_or_else(
                || {
                    permits(t)
                        || (!t.is_empty() && (t == scope || scope.starts_with(&format!("{t}."))))
                },
                |(s, l)| {
                    (s == "*" || scope == s || scope.starts_with(&format!("{s}."))) && permits(l)
                },
            )
        })
    {
        let _ = writeln!(
            io::stderr(),
            "{}",
            json!({"timestamp":humantime::format_rfc3339_millis(SystemTime::now()).to_string(),"level":level,"scope":scope,"event":event,"operation":args.command.name(),"correlation_id":std::process::id(),"elapsed_ms":start.elapsed().as_millis(),"count":count})
        );
    }
}

/// The bounded result; the full counts are retained in the final summary.
#[derive(Default)]
struct Report {
    /// Validation errors requiring action.
    findings: Vec<Finding>,
    /// Informational records in stable display order.
    rows: Vec<String>,
}

/// Keep a record on one line: escape control characters only, so quotes stay readable.
fn oneline(text: &str) -> String {
    text.chars()
        .flat_map(|c| {
            let escaped: Vec<char> = if c.is_control() {
                c.escape_default().collect()
            } else {
                vec![c]
            };
            escaped
        })
        .collect()
}

/// Write the result streams independently; never truncate the verdict.
fn render(args: &Args, report: &Report) -> io::Result<ExitCode> {
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();
    let failures = report.findings.len();
    let name = args.command.name();
    let total = failures + report.rows.len();
    let shown = total.min(args.limit);
    let status = if failures == 0 { "ok" } else { "blocked" };
    let summary = format!(
        "{name} {status} findings={failures} rows={} shown={shown}",
        report.rows.len()
    );
    for f in report.findings.iter().take(args.limit) {
        writeln!(
            stderr,
            "{}:{}: {} {}",
            oneline(&f.file),
            f.line,
            f.code,
            oneline(&f.message)
        )?;
    }
    if args.json {
        writeln!(
            stdout,
            "{}",
            json!({"command":name,"status":status,"total":total,"shown":shown,"findings":report.findings.iter().take(args.limit).collect::<Vec<_>>(),"rows":report.rows.iter().take(args.limit.saturating_sub(failures)).collect::<Vec<_>>()})
        )?;
    } else if failures == 0 {
        for row in report.rows.iter().take(args.limit) {
            writeln!(stdout, "{}", oneline(row))?;
        }
        writeln!(stdout, "{summary}")?;
    }
    if failures > 0 {
        writeln!(stderr, "{summary}")?;
    }
    Ok(ExitCode::from(if failures == 0 { 0 } else { 2 }))
}

/// Resolve repository root without spawning git.
fn root(args: &Args) -> io::Result<PathBuf> {
    if let Some(path) = &args.root {
        return path.canonicalize();
    }
    let cwd = env::current_dir()?;
    cwd.ancestors()
        .find(|p| p.join(".git").exists())
        .or_else(|| cwd.ancestors().find(|p| p.join("docs").is_dir()))
        .map(Path::to_owned)
        .ok_or_else(|| io::Error::other("no repository found; pass --root"))
}

/// Resolve the endpoints of both outgoing and incoming references.
fn refs(repo: &repo::Repo, query: &str) -> Vec<String> {
    let query = query.trim_start_matches("./").replace("ADR-", "ADR ");
    let rule = repo.resolve(&query);
    let mut rows = Vec::new();
    for link in &repo.links {
        let (from, _) = link.from.location();
        let (to, slug) = link.to.location();
        let dest = slug.map_or_else(|| to.to_owned(), |s| format!("{to}#{s}"));
        if [from, to].iter().any(|p| Path::new(p).starts_with(&query))
            || rule.as_ref().is_some_and(|(doc, heading)| {
                to == doc && heading.as_deref().is_none_or(|h| Some(h) == slug)
            })
            || dest == query
            || link.context == query
        {
            rows.push(format!(
                "{}:{} {} {}",
                link.source_file,
                link.line,
                link.relation,
                if link.relation == "covers" {
                    to
                } else {
                    &link.context
                }
            ));
        }
    }
    rows.sort();
    rows.dedup();
    rows
}

/// Execute a single operation and retain only checked edges in graph output.
fn run(args: &Args, start: Instant) -> io::Result<ExitCode> {
    let root = root(args)?;
    log(args, "doc-links.cli", "debug", "command started", start, 0);
    let mut report = Report::default();
    if let Operation::Commit { message_file } = &args.command {
        report.findings = commit::check(&root, message_file)?;
    } else if matches!(args.command, Operation::Kinds) {
        let config = kinds::Config::load(&root)?
            .ok_or_else(|| io::Error::other(format!("missing {}", kinds::FILE)))?;
        for kind in &config.kinds {
            report.rows.push(format!(
                "{} {} in={} {}",
                kind.name,
                kind.pattern(),
                kind.folders_list(),
                kind.summary()
            ));
        }
    } else if matches!(args.command, Operation::Graph)
        && !root.join("graphify-out/graph.json").try_exists()?
    {
        report
            .rows
            .push("Graph absent. Run graphify update . to create it.".into());
    } else {
        let repo = repo::Repo::load(&root, &root)?;
        log(
            args,
            "doc-links.scan",
            "debug",
            "vault indexed",
            start,
            repo.pages.len(),
        );
        match &args.command {
            Operation::Check { paths } => {
                report.findings = repo
                    .findings
                    .into_iter()
                    .filter(|f| {
                        paths.is_empty()
                            || paths
                                .iter()
                                .any(|p| root.join(&f.file).starts_with(root.join(p)))
                    })
                    .collect();
            }
            Operation::Refs { query } => report.rows = refs(&repo, query),
            Operation::Matrix => report.rows = repo.matrix(),
            Operation::Graph => {
                let path = root.join("graphify-out/graph.json");
                let mut graph: Value = serde_json::from_reader(fs::File::open(&path)?)?;
                if !graph.get("nodes").is_some_and(Value::is_array)
                    || !graph
                        .get("links")
                        .or_else(|| graph.get("edges"))
                        .is_some_and(Value::is_array)
                {
                    return Err(io::Error::other(
                        "invalid graph arrays; regenerate with graphify update .",
                    ));
                }
                let stats = graph::inject(&mut graph, &repo.links);
                let mut tmp = tempfile::NamedTempFile::new_in(root.join("graphify-out"))?;
                serde_json::to_writer(tmp.as_file_mut(), &graph)?;
                let _ = tmp.persist(&path)?;
                report.rows.push(format!(
                    "added={} removed={} unplaced={}",
                    stats.added, stats.removed, stats.unplaced
                ));
            }
            Operation::Commit { .. } | Operation::Kinds => {}
        }
    }
    log(
        args,
        "doc-links.cli",
        "info",
        if report.findings.is_empty() {
            "command completed"
        } else {
            "validation blocked"
        },
        start,
        report.findings.len(),
    );
    render(args, &report)
}

/// Map usage, dependency and validation outcomes to the hook contract.
fn main() -> ExitCode {
    let args = match Args::try_parse() {
        Ok(a) => a,
        Err(e) => {
            let help = matches!(e.kind(), ErrorKind::DisplayHelp | ErrorKind::DisplayVersion);
            let _ = e.print();
            return if help {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            };
        }
    };
    let start = Instant::now();
    run(&args, start).unwrap_or_else(|e| {
        log(&args, "doc-links.cli", "error", "command failed", start, 1);
        let _ = writeln!(
            io::stderr(),
            "E_RUN {}. Verify inputs and repository access, then retry.",
            oneline(&e.to_string())
        );
        ExitCode::FAILURE
    })
}
