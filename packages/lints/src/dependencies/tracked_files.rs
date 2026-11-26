//! Check the lockfile and vendoring policy against a Git snapshot.
//! spec: docs/repo/lint-rules.md#bindings
use std::path::{Path, PathBuf};

/// Return lockfiles absent from the index or removed from the working tree.
pub fn missing_locks(root: &Path, tracked: &[PathBuf]) -> Vec<PathBuf> {
    ["Cargo.lock", "pnpm-lock.yaml"]
        .into_iter()
        .map(PathBuf::from)
        .filter(|file| !tracked.contains(file) || !root.join(file).is_file())
        .collect()
}

/// Return changed vendored source paths from staged and unstaged Git changes.
pub fn vendor_changes(changes: impl IntoIterator<Item = PathBuf>) -> Vec<PathBuf> {
    changes
        .into_iter()
        .filter(|p| p.starts_with("vendor"))
        .collect()
}

#[cfg(test)]
mod tests {
    //! Validate the accepted and rejected snapshots.
    use super::{PathBuf, missing_locks, vendor_changes};
    use std::fs;
    /// Missing, untracked and present lockfiles are distinct states.
    #[test]
    fn lock_inventory() {
        let root = tempfile::tempdir().unwrap();
        let paths = vec![PathBuf::from("Cargo.lock"), PathBuf::from("pnpm-lock.yaml")];
        for file in &paths {
            fs::write(root.path().join(file), "").unwrap();
        }
        assert_eq!(missing_locks(root.path(), &paths), Vec::<PathBuf>::new());
        assert_eq!(missing_locks(root.path(), &[]), paths);
        fs::remove_file(root.path().join("Cargo.lock")).unwrap();
        assert_eq!(
            missing_locks(root.path(), &paths),
            [PathBuf::from("Cargo.lock")]
        );
    }
    /// Files with similar prefixes outside the vendor directory remain editable.
    #[test]
    fn vendored_sources() {
        assert_eq!(
            vendor_changes(["vendor/x", "vendor-tools/x", "src/x"].map(PathBuf::from)),
            [PathBuf::from("vendor/x")]
        );
    }
}
