//! Collect static process-environment reads from Rust syntax.
//! spec: docs/repo/lint-rules.md#bindings
use syn::{
    Expr, ExprCall, ExprLit, File, Lit, LitStr, Macro,
    visit::{self, Visit},
};

/// One literal environment variable and its one-based source line.
#[derive(Debug, Eq, PartialEq)]
pub struct Read {
    /// Name required in the template and task configuration.
    pub name: String,
    /// Line of the read.
    pub line: usize,
}

/// Collect direct environment calls and compile-time environment macros.
#[must_use]
pub fn reads(file: &File) -> Vec<Read> {
    let mut collector = Collector(Vec::new());
    collector.visit_file(file);
    collector.0
}

/// Visitor that records literal names without scanning comments or string contents.
struct Collector(Vec<Read>);

impl Collector {
    /// Record a literal read location.
    fn record(&mut self, lit: &LitStr) {
        // Cargo provides build metadata to rustc and integration tests independently of user configuration.
        let name = lit.value();
        if name == "CARGO_MANIFEST_DIR"
            || name.starts_with("CARGO_PKG_")
            || name.starts_with("CARGO_BIN_EXE_")
        {
            return;
        }
        self.0.push(Read {
            name,
            line: lit.span().start().line,
        });
    }
}

impl<'ast> Visit<'ast> for Collector {
    fn visit_expr_call(&mut self, call: &'ast ExprCall) {
        if let Expr::Path(path) = call.func.as_ref() {
            let mut names = path.path.segments.iter().rev();
            if names
                .next()
                .is_some_and(|s| s.ident == "var" || s.ident == "var_os")
                && names.next().is_some_and(|s| s.ident == "env")
                && let Some(Expr::Lit(ExprLit {
                    lit: Lit::Str(name),
                    ..
                })) = call.args.first()
            {
                self.record(name);
            }
        }
        visit::visit_expr_call(self, call);
    }
    fn visit_macro(&mut self, mac: &'ast Macro) {
        if (mac.path.is_ident("env") || mac.path.is_ident("option_env"))
            && let Ok(name) = mac.parse_body::<LitStr>()
        {
            self.record(&name);
        }
        visit::visit_macro(self, mac);
    }
}

#[cfg(test)]
mod tests {
    //! Collect actual code while ignoring comments and unrelated calls.
    use super::reads;
    /// Both common environment accessors are recognized.
    #[test]
    fn literal_reads() {
        let file =
            syn::parse_file("fn f(){std::env::var_os(\"A\"); option_env!(\"B\"); obj.var(\"C\");}")
                .unwrap();
        let result = reads(&file);
        assert_eq!(
            result.iter().map(|r| r.name.as_str()).collect::<Vec<_>>(),
            ["A", "B"]
        );
    }
}
