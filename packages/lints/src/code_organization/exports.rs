//! Check entry points of JavaScript libraries (BP-ORG-007).
//! spec: docs/repo/lint-rules.md#bindings
use serde_json::Value;

/// Packages expose subpaths so consumers can depend on one module.
pub fn granular(package: &Value) -> bool {
    package
        .get("exports")
        .and_then(Value::as_object)
        .is_some_and(|exports| exports.keys().any(|key| key.starts_with("./")))
}

#[cfg(test)]
mod tests {
    //! Cover both legal forms of granular exports.
    use super::granular;
    use serde_json::json;
    /// A wildcard or named entry exposes a submodule, a root-only export does not.
    #[test]
    fn package_subpaths() {
        assert!(granular(&json!({"exports":{"./*":"./src/*.ts"}})));
        assert!(granular(&json!({"exports":{"./config":"./config.json"}})));
        assert!(!granular(&json!({"exports":{".":"./index.ts"}})));
    }
}
