//! File-local Rust lints and stock diagnostic bindings.
//! spec: docs/repo/lint-rules.md#bindings
pub mod ci;
pub mod code_organization;
pub mod configuration;
pub mod dependencies;
pub mod rust;
mod rust_errors;
pub mod security;
pub use rust_errors::main_exit_code::missing_status;
pub use syn::parse_file as parse;

/// Embedded catalog shared with the diagnostic runner.
pub const BINDINGS: &str = include_str!("../bindings.json");

/// Inspect a Rust binary entry file for a missing explicit exit status.
///
/// # Errors
/// Returns the parser's diagnostic when the input is invalid Rust.
pub fn check_entry(source: &str) -> Result<Option<usize>, syn::Error> {
    Ok(missing_status(&syn::parse_file(source)?))
}
