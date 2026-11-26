//! Extract explicit citations in code and classify their owners.
//! spec: docs/repo/doc-links.md#what-check-verifies
use crate::regex;

/// A source citation with its prospective graph endpoint.
pub(crate) struct Citation {
    /// Location of the written reference.
    pub(crate) line: usize,
    /// Code item governed by the comment.
    pub(crate) item: Option<usize>,
    /// True for test code.
    pub(crate) test: bool,
    /// Normalized path with optional anchor, invariant or decision.
    pub(crate) target: String,
}

/// Files that execute tests, including integration and end-to-end suites.
fn test_file(path: &str) -> bool {
    path.split('/')
        .any(|p| matches!(p, "tests" | "e2e" | "__tests__"))
        || [".test.", ".spec.", "_test.rs", "_tests.rs"]
            .iter()
            .any(|p| path.contains(p))
}

/// Read citations in source comments and test names.
pub(crate) fn scan(file: &str, content: &str) -> Vec<Citation> {
    let lines: Vec<_> = content.lines().collect();
    let mut tests = TestBlock::default();
    let mut out = Vec::new();
    let suite = test_file(file);
    for (i, line) in lines.iter().enumerate() {
        let test = (crate::extension(file, "rs") && tests.step(&lines, i)) || suite;
        if test
            && let Some(caps) =
                regex!(r"^\s*(?:async\s+)?fn\s+inv_([a-z][a-z0-9]*)_(\d{3})(?:_|\s*\()")
                    .captures(line)
            && let (Some(domain), Some(number)) = (caps.get(1), caps.get(2))
        {
            out.push(Citation {
                line: i.saturating_add(1),
                item: Some(i.saturating_add(1)),
                test,
                target: format!(
                    "INV-{}-{}",
                    domain.as_str().to_ascii_uppercase(),
                    number.as_str()
                ),
            });
        }
        let comment = is_comment(line.trim_start());
        if !(comment
            || test
                && ["test(", "it(", "describe("]
                    .iter()
                    .any(|s| line.trim_start().starts_with(s)))
        {
            continue;
        }
        let pattern = regex!(
            r"\b(?:spec:\s*(docs/[\w./-]+\.md(?:#[\pL\pN_-]+)?)|screen:\s*(docs/screens/[\w./-]+\.md)|((?:INV-[A-Z][A-Z0-9]*-\d{3}|ADR[ -]\d{4})\b))"
        );
        for captures in pattern.captures_iter(line) {
            if let Some(target) = captures.iter().skip(1).flatten().next() {
                out.push(Citation {
                    line: i.saturating_add(1),
                    item: item_after(&lines, i),
                    test,
                    target: target
                        .as_str()
                        .strip_prefix("ADR-")
                        .map_or_else(|| target.as_str().to_owned(), |id| format!("ADR {id}")),
                });
            }
        }
    }
    out
}

/// Tracks whether a line of a Rust file sits inside a `#[cfg(test)] mod … { … }` block.
#[derive(Debug, Default)]
struct TestBlock {
    /// Inside the block.
    active: bool,
    /// Open braces since the block started.
    depth: usize,
    /// The module's own `{` has been seen.
    opened: bool,
}

impl TestBlock {
    /// Advances over line `index`; true when that line belongs to a test module.
    fn step(&mut self, lines: &[&str], index: usize) -> bool {
        let line = lines.get(index).map_or("", |l| l.trim());
        if !self.active {
            if line != "#[cfg(test)]" {
                return false;
            }
            let module = lines
                .iter()
                .skip(index.saturating_add(1))
                .map(|l| l.trim())
                .find(|l| !l.starts_with("#["))
                .is_some_and(|l| l.contains("mod ") && l.ends_with('{'));
            if module {
                *self = Self {
                    active: true,
                    depth: 0,
                    opened: false,
                };
            }
            return module;
        }
        for c in line.chars() {
            match c {
                '{' => {
                    self.depth = self.depth.saturating_add(1);
                    self.opened = true;
                }
                '}' => self.depth = self.depth.saturating_sub(1),
                _ => {}
            }
        }
        if self.opened && self.depth == 0 {
            self.active = false;
        }
        true
    }
}

/// True when a trimmed line is a comment line in Rust, TypeScript, JavaScript or WGSL.
fn is_comment(trimmed: &str) -> bool {
    trimmed.starts_with('#')
        || trimmed.starts_with("//")
        || trimmed.starts_with("/*")
        || trimmed.starts_with('*')
}

/// Line (1-based) of the item the comment at `index` documents, if the citation sits in such a comment.
fn item_after(lines: &[&str], index: usize) -> Option<usize> {
    let here = lines.get(index)?.trim_start();
    if !is_comment(here) || here.starts_with("//!") || here.starts_with("/*!") {
        return None;
    }
    for (offset, line) in lines
        .iter()
        .enumerate()
        .skip(index.saturating_add(1))
        .take(40)
    {
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            return None;
        }
        if is_comment(trimmed) || trimmed.starts_with("#[") || trimmed.starts_with('@') {
            continue;
        }
        return Some(offset.saturating_add(1));
    }
    None
}
