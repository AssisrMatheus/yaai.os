//! Validate the Cargo lint inheritance table (BP-RUST-037).
//! spec: docs/repo/lint-rules.md#bindings
use toml::Table;

/// Confirm a member opts into the root policy without local overrides.
///
/// # Errors
/// Returns the TOML parsing error for malformed manifests.
pub fn workspace_policy(source: &str) -> Result<bool, toml::de::Error> {
    let manifest: Table = toml::from_str(source)?;
    Ok(!manifest.contains_key("package")
        || manifest
            .get("lints")
            .and_then(toml::Value::as_table)
            .is_some_and(|t| {
                t.len() == 1 && t.get("workspace").and_then(toml::Value::as_bool) == Some(true)
            }))
}

#[cfg(test)]
mod tests {
    //! Valid and invalid member manifests.
    use super::workspace_policy;
    /// A member must inherit the policy explicitly.
    #[test]
    fn inheritance() {
        assert!(workspace_policy("[package]\n[lints]\nworkspace=true").unwrap());
        assert!(!workspace_policy("[package]\n[lints.rust]\nunsafe_code='allow'").unwrap());
    }
}
