//! Validate the catalog against the documented rules and installed lint tools.
//! spec: docs/repo/lint-rules.md#bindings
use super::{Catalog, Failure, details};
use serde_json::Value;
use std::{collections::BTreeSet, fs, process::Command};

/// User-actionable drift between documentation, catalog and configuration.
#[derive(Debug)]
pub(super) struct Drift(String);

impl std::fmt::Display for Drift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for Drift {}

/// Extract literal lint classifications under stable headings.
fn expected(path: &str, source: &str) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    let mut id = None;
    for line in source.lines() {
        if line.starts_with('#') {
            id = line
                .strip_prefix("### BP-")
                .and_then(|h| h.split_once(':'))
                .map(|(n, _)| n);
        } else if line == "Check: lint"
            && let Some(n) = id
        {
            let _ = result.insert(format!("{path}#BP-{n}"));
        }
    }
    result
}

/// Compare the complete catalog to the active rule pages in both directions.
fn coverage(required: &BTreeSet<String>, catalog: &Catalog) -> Result<(), Failure> {
    let present = catalog.values().flatten().cloned().collect::<BTreeSet<_>>();
    let missing: Vec<_> = required.difference(&present).collect();
    let stale: Vec<_> = present.difference(required).collect();
    if missing.is_empty() && stale.is_empty() {
        Ok(())
    } else {
        Err(Drift(format!(
            "Repair the binding catalog. Missing: {missing:?}. Stale: {stale:?}."
        ))
        .into())
    }
}

/// Collect available lint identifiers from the pinned compiler's inventory.
fn inventory(program: &str) -> Result<BTreeSet<String>, Failure> {
    let output = Command::new(program).args(["-W", "help"]).output()?;
    if !output.status.success() {
        return Err(details(&output).into());
    }
    Ok(std::str::from_utf8(&output.stdout)?
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        .map(|s| s.replace('-', "_"))
        .collect())
}

/// Effective oxlint rules include the default configuration and file overrides.
fn ox_enabled(config: &Value, name: &str) -> bool {
    let key = name
        .strip_prefix("eslint/")
        .unwrap_or(name)
        .replace("jsx-a11y/", "jsx_a11y/");
    let enabled = |obj: &Value| {
        obj.get("rules").and_then(|r| r.get(&key)).is_some_and(|v| {
            let level = v.as_array().and_then(|a| a.first()).unwrap_or(v);
            matches!(level.as_str(), Some("deny" | "error" | "warn"))
        })
    };
    enabled(config)
        || config
            .get("overrides")
            .and_then(Value::as_array)
            .is_some_and(|a| a.iter().any(enabled))
}

/// Every configured Clippy restriction must carry its governing rule.
fn restrictions(config: &toml::Table, required: &BTreeSet<String>) -> Result<(), Failure> {
    for name in [
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
    ] {
        for row in config
            .get(name)
            .and_then(toml::Value::as_array)
            .into_iter()
            .flatten()
        {
            let reference = row
                .get("reason")
                .and_then(toml::Value::as_str)
                .and_then(|s| s.split_once(" Rule: "))
                .map(|(_, r)| r);
            if !reference.is_some_and(|r| required.contains(r)) {
                return Err(Drift(format!(
                    "{name} has an invalid reason. Cite a live rule for {row:?}."
                ))
                .into());
            }
        }
    }
    Ok(())
}

/// Verify that the type-check binding runs under the documented compiler settings.
fn compiler_options() -> Result<(), Failure> {
    let types: Value = serde_json::from_slice(&fs::read("tsconfig.json")?)?;
    for option in [
        "strict",
        "erasableSyntaxOnly",
        "noUncheckedIndexedAccess",
        "exactOptionalPropertyTypes",
    ] {
        if types
            .get("compilerOptions")
            .and_then(|o| o.get(option))
            .and_then(Value::as_bool)
            != Some(true)
        {
            return Err(Drift(format!(
                "The compiler option {option} is disabled. Enable it for BP-TS-058."
            ))
            .into());
        }
    }
    Ok(())
}

/// Confirm each active document has a binding and each stock lint exists and is enabled.
pub(super) fn check(catalog: &Catalog) -> Result<(), Failure> {
    let mut required = BTreeSet::new();
    for entry in fs::read_dir("docs/best-practices")? {
        let path = entry?.path();
        if path.to_string_lossy().ends_with(".rules.md") {
            required.extend(expected(
                &path.to_string_lossy(),
                &fs::read_to_string(&path)?,
            ));
        }
    }
    let inherited = fs::read_to_string("docs/best-practices/typescript.rules.md")?;
    for line in inherited.lines().filter_map(|s| s.strip_prefix("- BP-TS-")) {
        if let Some((suffix, target)) = line.split_once(": Same as [[best-practices/")
            && let Some((page, heading)) = target.split_once('#')
            && let Some((canonical, _)) = heading.split_once(':')
            && required.contains(&format!("docs/best-practices/{page}.md#{canonical}"))
        {
            let _ = required.insert(format!(
                "docs/best-practices/typescript.rules.md#BP-TS-{suffix}"
            ));
        }
    }
    coverage(&required, catalog)?;
    let cargo: toml::Table = toml::from_str(&fs::read_to_string("Cargo.toml")?)?;
    restrictions(
        &toml::from_str(&fs::read_to_string("clippy.toml")?)?,
        &required,
    )?;
    compiler_options()?;
    let compilers = inventory("clippy-driver")?;
    let rustdocs = inventory("rustdoc")?;
    let output = Command::new("node_modules/.bin/oxlint")
        .arg("--print-config")
        .output()?;
    if !output.status.success() {
        return Err(details(&output).into());
    }
    let config: Value = serde_json::from_slice(&output.stdout)?;
    for (key, refs) in catalog {
        let (tool, name) = key.split_once("::").ok_or("invalid lint key")?;
        let enabled = match tool {
            "clippy" | "rustc" | "rustdoc" => {
                let table = if tool == "rustc" { "rust" } else { tool };
                let level = cargo
                    .get("workspace")
                    .and_then(|v| v.get("lints"))
                    .and_then(|v| v.get(table))
                    .and_then(|v| v.get(name));
                let active = level
                    .and_then(|v| v.as_str().or_else(|| v.get("level")?.as_str()))
                    .is_some_and(|l| matches!(l, "warn" | "deny" | "forbid"));
                active
                    && if tool == "rustdoc" {
                        rustdocs.contains(key)
                    } else if tool == "clippy" {
                        compilers.contains(key)
                    } else {
                        compilers.contains(name)
                    }
            }
            "oxlint" => matches!(name, "categories" | "type-check") || ox_enabled(&config, name),
            "yaai" => matches!(
                name,
                "main-exit-code"
                    | "granular-exports"
                    | "workspace-lints"
                    | "package-manager"
                    | "env-template"
                    | "task-env"
                    | "verify-hooks"
                    | "check-mode"
                    | "frozen-install"
                    | "tracked-lockfiles"
                    | "tracked-env"
                    | "vendor-changes"
            ),
            _ => matches!(
                key.as_str(),
                "doc-links::E_CITE"
                    | "doc-links::E_TOOL_DOC"
                    | "rustfmt::check"
                    | "oxfmt::check"
                    | "cargo-deny::check"
                    | "cargo-machete::unused"
                    | "knip::unused"
            ),
        };
        if !enabled || refs.is_empty() {
            return Err(Drift(format!(
                "{key} is unavailable or disabled. Enable the bound lint for {refs:?}."
            ))
            .into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    //! Reject stale or missing rule references and disabled lint settings.
    use super::{Catalog, coverage, expected, ox_enabled};
    use serde_json::json;
    /// Validate a mapping, then remove and mistype its reference.
    #[test]
    fn catalog_drift() {
        let ids = expected(
            "page",
            "### BP-X-001: Example\nCheck: lint\n### BP-X-002: Human\nCheck: review",
        );
        assert_eq!(ids.len(), 1);
        assert!(coverage(&ids, &Catalog::new()).is_err());
        let mut catalog = Catalog::from([("lint".into(), vec!["page#BP-X-001".into()])]);
        assert!(coverage(&ids, &catalog).is_ok());
        catalog
            .entry("other".into())
            .or_default()
            .push("page#BP-X-002".into());
        assert!(coverage(&ids, &catalog).is_err());
        assert!(!ox_enabled(
            &json!({"rules":{"no-console":"allow"}}),
            "eslint/no-console"
        ));
        assert!(ox_enabled(
            &json!({"rules":{"no-console":"deny"}}),
            "eslint/no-console"
        ));
    }
}
