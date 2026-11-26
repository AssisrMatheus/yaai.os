//! Validate the Git index, including partially staged files.
//! spec: docs/repo/doc-links.md#the-commit-gate
use crate::{
    Finding,
    repo::{Repo, matches_path},
};
use std::{collections::BTreeSet, ffi::OsStr, fs, io, path::Path, process::Command};

/// Run a read-only Git command against the requested repository.
fn git(root: &Path, args: &[&OsStr]) -> io::Result<Vec<u8>> {
    let out = Command::new("git").current_dir(root).args(args).output()?;
    if out.status.success() {
        Ok(out.stdout)
    } else {
        Err(io::Error::other(
            String::from_utf8_lossy(&out.stderr).trim().to_owned(),
        ))
    }
}

/// Copy working-tree files that are not part of this commit over the index snapshot.
///
/// WHY: docs and code cite each other, so a scoped commit stages one side while the other is still untracked or
/// unstaged. Staged files keep their staged content; every other file is read as it is on disk, so a citation
/// blocks only when its target exists nowhere. The full `check` stays strict on the working tree.
fn overlay(root: &Path, snapshot: &Path, staged: &BTreeSet<&str>) -> io::Result<()> {
    let lists: [&[&OsStr]; 2] = [
        &[
            "ls-files".as_ref(),
            "--others".as_ref(),
            "--exclude-standard".as_ref(),
            "-z".as_ref(),
        ],
        &["diff".as_ref(), "--name-only".as_ref(), "-z".as_ref()],
    ];
    for args in lists {
        let out = git(root, args)?;
        let names = std::str::from_utf8(&out).map_err(io::Error::other)?;
        for name in names.split('\0').filter(|n| !n.is_empty()) {
            let from = root.join(name);
            if staged.contains(name) || !from.is_file() {
                continue;
            }
            let to = snapshot.join(name);
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)?;
            }
            let _ = fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

/// Check the staged snapshot and the actual final trailer paragraph.
pub(crate) fn check(root: &Path, msg: &Path) -> io::Result<Vec<Finding>> {
    let msg = fs::canonicalize(msg)?;
    let trailers = git(
        root,
        &[
            "interpret-trailers".as_ref(),
            "--parse".as_ref(),
            msg.as_os_str(),
        ],
    )?;
    let waived = String::from_utf8_lossy(&trailers).lines().any(|line| {
        line.strip_prefix("Spec-unchanged:")
            .is_some_and(|why| !why.trim().is_empty())
    });
    let changed = git(
        root,
        &[
            "diff".as_ref(),
            "--cached".as_ref(),
            "--no-renames".as_ref(),
            "--name-only".as_ref(),
            "-z".as_ref(),
        ],
    )?;
    let paths = std::str::from_utf8(&changed)
        .map_err(io::Error::other)?
        .split('\0')
        .filter(|s| !s.is_empty())
        .collect::<BTreeSet<_>>();
    if paths.is_empty() {
        return Ok(Vec::new());
    }
    let snapshot = tempfile::tempdir()?;
    let prefix = format!("--prefix={}/", snapshot.path().display());
    let _ = git(
        root,
        &["checkout-index".as_ref(), "--all".as_ref(), prefix.as_ref()],
    )?;
    overlay(root, snapshot.path(), &paths)?;
    let repo = Repo::load(snapshot.path(), root)?;
    let mut findings: Vec<_> = repo
        .findings
        .into_iter()
        .filter(|f| paths.contains(f.file.as_str()))
        .collect();
    if !waived {
        for (file, page) in repo.pages {
            if paths.contains(file.as_str()) || crate::archived(&file) {
                continue;
            }
            for path in &paths {
                if page
                    .covers
                    .iter()
                    .flatten()
                    .any(|(_, p)| matches_path(p, path))
                {
                    let remedy = format!(
                        "Stage {file} or explain why it still holds in a Spec-unchanged: trailer."
                    );
                    findings.push(Finding::new(path, 1, "E_DRIFT", remedy));
                }
            }
        }
    }
    findings.sort();
    Ok(findings)
}
