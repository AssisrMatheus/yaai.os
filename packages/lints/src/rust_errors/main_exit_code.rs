//! Inspect the top-level binary entry point (BP-RUST-057).
//! spec: docs/repo/lint-rules.md#bindings
use syn::{File, Item, ReturnType};

/// Locate the entry point if its implicit unit return loses the process status.
#[must_use]
pub fn missing_status(file: &File) -> Option<usize> {
    file.items.iter().find_map(|item| {
        let Item::Fn(function) = item else {
            return None;
        };
        let unit = match &function.sig.output {
            ReturnType::Default => true,
            ReturnType::Type(_, ty) => {
                matches!(ty.as_ref(),syn::Type::Tuple(tuple) if tuple.elems.is_empty())
            }
        };
        (function.sig.ident == "main" && unit).then(|| {
            let span: proc_macro2::Span = function.sig.ident.span();
            span.start().line
        })
    })
}

#[cfg(test)]
mod tests {
    //! UI cases for BP-RUST-057.
    use crate::check_entry;

    /// Report the entry point's source line while accepting an explicit return type.
    #[test]
    fn entry_status() {
        assert_eq!(check_entry("\nfn main() {}").unwrap(), Some(2));
        assert_eq!(check_entry("fn main() -> () {}").unwrap(), Some(1));
        assert_eq!(
            check_entry("fn main() -> Result<(), Error> { Ok(()) }").unwrap(),
            None
        );
    }
}
