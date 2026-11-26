//! Select test files by measured duration. spec: docs/tooling/test-shard.tool.md
use clap::Parser;
use serde::Deserialize;
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    fs::File,
    io::{self, BufRead, Write},
    num::NonZeroUsize,
    path::PathBuf,
    process::ExitCode,
};

/// Assign newline-delimited test IDs from stdin to balanced shards.
#[derive(Debug, Parser)]
#[command(
    after_help = "Exit: 0 success, 2 invalid input, 1 I/O failure.\nExample: echo root-hooks.test.ts | bin/test-shard --timings .github/ci/timings.json --shard 1 --count 2"
)]
struct Args {
    /// Bun version 1 timing file, in milliseconds.
    #[arg(long)]
    timings: PathBuf,
    /// Shard to print, starting at one.
    #[arg(long)]
    shard: NonZeroUsize,
    /// Number of shards.
    #[arg(long)]
    count: NonZeroUsize,
    /// Print the selected files as a JSON object instead of a summary.
    #[arg(long)]
    json: bool,
}

/// Bun's portable timing snapshot.
#[derive(Debug, Deserialize)]
struct Timings {
    /// Timing format version.
    version: u32,
    /// Milliseconds keyed by file ID.
    files: BTreeMap<String, u32>,
}

/// Assign each eligible file exactly once. Doubled weights preserve half-millisecond medians.
fn select(
    ids: BTreeSet<String>,
    times: &BTreeMap<String, u32>,
    shard: usize,
    count: usize,
) -> Vec<String> {
    let mut known: Vec<_> = times.values().copied().collect();
    known.sort_unstable();
    let middle = known.len() / 2;
    let median = u64::from(known.get(middle).copied().unwrap_or(1)).saturating_add(u64::from(
        known
            .get(known.len().saturating_sub(1) / 2)
            .copied()
            .unwrap_or(1),
    ));
    let mut weighted: Vec<_> = ids
        .into_iter()
        .map(|id| {
            let weight = times.get(&id).map_or(median, |&t| u64::from(t) * 2);
            (Reverse(weight.max(1)), id)
        })
        .collect();
    weighted.sort_unstable();
    let mut loads: BinaryHeap<_> = (1..=count.min(weighted.len()))
        .map(|i| Reverse((0u128, i)))
        .collect();
    let mut selected = Vec::new();
    for (Reverse(weight), id) in weighted {
        if let Some(mut lightest) = loads.peek_mut() {
            let Reverse((load, index)) = *lightest;
            if index == shard {
                selected.push(id);
            }
            *lightest = Reverse((load.saturating_add(u128::from(weight)), index));
        }
    }
    selected
}

/// Read inputs and write the selected shard.
fn run(args: &Args) -> io::Result<()> {
    if args.shard > args.count {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "set --shard at or below --count",
        ));
    }
    let file = File::open(&args.timings).map_err(|error| {
        let kind = if error.kind() == io::ErrorKind::NotFound {
            io::ErrorKind::InvalidInput
        } else {
            error.kind()
        };
        io::Error::new(kind, format!("read {}: {error}", args.timings.display()))
    })?;
    let times: Timings = serde_json::from_reader(file)?;
    if times.version != 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "use a version 1 timing file",
        ));
    }
    let ids = io::stdin().lock().lines().collect::<io::Result<_>>()?;
    let files = select(ids, &times.files, args.shard.get(), args.count.get());
    let mut out = io::stdout().lock();
    if args.json {
        serde_json::to_writer(&mut out, &serde_json::json!({"files": files}))?;
        writeln!(out)
    } else {
        writeln!(
            out,
            "test-shard {}/{} ok files={}",
            args.shard,
            args.count,
            files.len()
        )
    }
}

/// Preserve distinct caller and operational failures.
fn main() -> ExitCode {
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(error) => {
            let _ = error.print();
            return if error.use_stderr() {
                ExitCode::from(2)
            } else {
                ExitCode::SUCCESS
            };
        }
    };
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let _ = writeln!(io::stderr().lock(), "test-shard failed: {error}");
            if matches!(
                error.kind(),
                io::ErrorKind::InvalidData
                    | io::ErrorKind::InvalidInput
                    | io::ErrorKind::UnexpectedEof
            ) {
                ExitCode::from(2)
            } else {
                ExitCode::FAILURE
            }
        }
    }
}

#[cfg(test)]
mod tests {
    //! Allocation contract. spec: docs/tooling/test-shard.tool.md#commands
    use super::{BTreeMap, BTreeSet, select};

    /// An unseen file receives the median. A long file occupies a shard by itself.
    #[test]
    fn measured_assignment() {
        let times = BTreeMap::from([("a".into(), 9), ("b".into(), 2), ("c".into(), 1)]);
        let ids: BTreeSet<_> = ["a", "b", "c", "d"].map(String::from).into();
        assert_eq!(select(ids.clone(), &times, 1, 2), ["a"]);
        assert_eq!(select(ids.clone(), &times, 2, 2), ["b", "d", "c"]);
        assert_eq!(select(ids, &times, 5, 5), Vec::<String>::new());
    }
}
