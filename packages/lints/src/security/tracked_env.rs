//! Identify tracked local environment files (BP-SEC-001).
//! spec: docs/repo/lint-rules.md#bindings
use std::path::Path;

/// A workspace may publish its template; actual settings belong outside the Git index.
#[must_use]
pub fn local_values(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|s| s.starts_with(".env") && s != ".env.example")
}

#[cfg(test)]
mod tests {
    //! Distinguish local secrets from documented configuration.
    use super::{Path, local_values};
    /// Match root and nested local settings.
    #[test]
    fn settings_names() {
        assert!(local_values(Path::new("tooling/tool/.env.local")));
        assert!(local_values(Path::new(".env")));
        assert!(!local_values(Path::new(".env.example")));
    }
}
