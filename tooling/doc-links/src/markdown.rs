//! Read vault pages and keep source lines for each repair.
//! spec: docs/repo/doc-links.md#what-check-verifies

use crate::{Finding, archived, regex};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use serde_yaml_ng::Value;
use std::collections::BTreeSet;

/// A reference with its original location.
pub(crate) type Located = (usize, String);
/// Data used after every page has been indexed.
#[derive(Default)]
pub(crate) struct Page {
    /// Rendered text and GitHub anchor of each heading.
    pub(crate) headings: Vec<(String, String)>,
    /// Rule IDs and their heading anchors.
    pub(crate) rules: Vec<(String, String)>,
    /// Vault targets, before resolution.
    pub(crate) wiki: Vec<Located>,
    /// Paths in the governing section, including directory prefixes.
    pub(crate) covers: Option<Vec<Located>>,
    /// Every heading in order, with its level and line.
    pub(crate) outline: Vec<Heading>,
    /// Body lines with their line number; lines inside fenced or indented code are left out.
    pub(crate) lines: Vec<Located>,
    /// First line of each top-level paragraph, and whether an H2 precedes it.
    pub(crate) paragraphs: Vec<(usize, String, bool)>,
    /// Whether the first block after the H1 is a paragraph, once an H1 was seen.
    pub(crate) opens_with_paragraph: Option<bool>,
    /// Frontmatter keys with their line and their value when it is a string.
    pub(crate) meta: Vec<(usize, String, Option<String>)>,
}

/// A heading as written in the page.
pub(crate) struct Heading {
    /// 1 for an H1, 2 for an H2, and so on.
    pub(crate) level: u8,
    /// Rendered text.
    pub(crate) text: String,
    /// One-based line.
    pub(crate) line: usize,
}

/// GitHub heading anchor before duplicate disambiguation.
pub(crate) fn github_slug(title: &str) -> String {
    title
        .chars()
        .filter(|c| c.is_alphanumeric() || matches!(c, '_' | '-' | ' '))
        .flat_map(char::to_lowercase)
        .map(|c| if c == ' ' { '-' } else { c })
        .collect()
}

/// Extract the target before a display alias, including an escaped table separator.
pub(crate) fn target(raw: &str) -> &str {
    raw.split('|')
        .next()
        .unwrap_or(raw)
        .trim_end_matches('\\')
        .trim()
}

/// Interpret a vault-root path or same-page fragment.
pub(crate) fn destination(file: &str, raw: &str) -> (String, Option<String>) {
    let (path, heading) = target(raw)
        .split_once('#')
        .map_or((target(raw), None), |(p, h)| (p, Some(h.to_owned())));
    (
        if path.is_empty() {
            file.to_owned()
        } else {
            format!("docs/{}.md", path.trim_end_matches(".md"))
        },
        heading,
    )
}

/// Read the YAML envelope, returning the byte offset of the body.
fn frontmatter(file: &str, text: &str, page: &mut Page, out: &mut Vec<Finding>) -> usize {
    let mut lines = text.split_inclusive('\n');
    let Some(start) = lines.next().filter(|l| l.trim_end() == "---") else {
        out.push(Finding::new(
            file,
            1,
            "E_YAML",
            "Add YAML frontmatter delimited by ---.",
        ));
        return 0;
    };
    let mut end = start.len();
    for (i, line) in lines.enumerate() {
        if line.trim_end() == "---" {
            let yaml = text.get(start.len()..end).unwrap_or_default();
            if i + 2 > 12 {
                out.push(Finding::new(
                    file,
                    1,
                    "E_LENGTH",
                    "Shorten the frontmatter to 12 lines including delimiters.",
                ));
            }
            match serde_yaml_ng::from_str::<Value>(yaml) {
                Ok(value) => {
                    record_meta(&value, yaml, page);
                    validate_meta(file, &value, yaml, page, out);
                }
                Err(e) => out.push(Finding::new(
                    file,
                    e.location().map_or(1, |l| l.line() + 1),
                    "E_YAML",
                    "Correct the YAML syntax in the frontmatter.",
                )),
            }
            return end + line.len();
        }
        end += line.len();
    }
    out.push(Finding::new(
        file,
        1,
        "E_YAML",
        "Close the frontmatter with ---.",
    ));
    0
}

/// Keep each top-level key with its line for the kind checks.
fn record_meta(v: &Value, yaml: &str, page: &mut Page) {
    let Some(map) = v.as_mapping() else { return };
    for (key, value) in map {
        let Some(key) = key.as_str() else { continue };
        let line = yaml
            .lines()
            .position(|l| l.strip_prefix(key).is_some_and(|r| r.starts_with(':')))
            .map_or(1, |i| i + 2);
        let text = match value {
            Value::String(s) => Some(s.trim().to_owned()),
            Value::Number(n) => Some(n.to_string()),
            _ => None,
        };
        page.meta.push((line, key.to_owned(), text));
    }
}

/// Validate metadata and treat the quoted archive successor as a real wiki edge.
fn validate_meta(file: &str, v: &Value, yaml: &str, page: &mut Page, out: &mut Vec<Finding>) {
    let field = |key: &str| {
        v.get(key)
            .and_then(Value::as_str)
            .map(str::trim)
            .unwrap_or_default()
    };
    let mut issue = |msg: &str| out.push(Finding::new(file, 1, "E_META", msg));
    if archived(file) {
        // WHY: whether `commit` is present and well formed is the old kind's rule, in kinds.toml.
        let replacement = field("superseded_by");
        let quoted = yaml.lines().enumerate().find(|(_, l)| {
            l.strip_prefix("superseded_by:")
                .is_some_and(|s| s.trim_start().starts_with(['\'', '"']))
        });
        if let Some((line, _)) = quoted
            && let Some(raw) = replacement
                .strip_prefix("[[")
                .and_then(|v| v.strip_suffix("]]"))
            && !target(raw).is_empty()
            && !raw.contains("]]")
            && destination(file, raw).0 != file
        {
            page.wiki.push((line + 2, raw.to_owned()));
        } else {
            issue("Set superseded_by to one quoted wikilink to the replacement page.");
        }
        return;
    }
    let skill = file.ends_with("/SKILL.md");
    if skill && field("name").is_empty() {
        issue("Give this scope index a non-empty name.");
    }
    let limit = if skill { 1024 } else { 300 };
    let length = field("description").chars().count();
    if length == 0 || length > limit {
        issue(&format!("Write a description of 1 to {limit} characters."));
    }
    if !v.get("tags").is_some_and(Value::is_sequence) {
        issue("Set tags to a YAML list.");
    }
}

/// Disambiguate repeated headings in document order.
fn unique_slug(title: &str, seen: &mut BTreeSet<String>) -> String {
    let stem = github_slug(title);
    let mut slug = stem.clone();
    let mut suffix = 0;
    while !seen.insert(slug.clone()) {
        suffix += 1;
        slug = format!("{stem}-{suffix}");
    }
    slug
}

impl Page {
    /// Record a finished heading; true when it opens the Covers section.
    fn add_heading(
        &mut self,
        h: String,
        level: u8,
        line: usize,
        seen: &mut BTreeSet<String>,
    ) -> bool {
        let slug = unique_slug(&h, seen);
        self.outline.push(Heading {
            level,
            text: h.clone(),
            line,
        });
        let mut covers = false;
        if level == 2 {
            if h == "Covers" {
                covers = true;
                let _ = self.covers.get_or_insert_with(Vec::new);
            }
            if let Some((id, _)) = h.split_once(':')
                && regex!(r"^INV-[A-Z][A-Z0-9]*-\d{3}$").is_match(id)
            {
                self.rules.push((id.to_owned(), slug.clone()));
            }
        }
        self.headings.push((h, slug));
        covers
    }
}

/// Block-level facts the kind checks read: code lines, top-level paragraphs, the block after the H1.
#[derive(Default)]
struct Blocks {
    /// Per body line: inside a code block.
    code: Vec<bool>,
    /// Open lists, quotes and tables.
    depth: usize,
    /// The H1 just ended and no block followed yet.
    after_h1: bool,
}

impl Blocks {
    /// Update the block facts with one parser event.
    fn observe(
        &mut self,
        ev: &Event<'_>,
        range: &std::ops::Range<usize>,
        line: usize,
        body: &str,
        newlines: &[usize],
        page: &mut Page,
    ) {
        if let Event::Start(tag) = ev
            && self.after_h1
            && self.depth == 0
            && !matches!(
                tag,
                Tag::Heading {
                    level: HeadingLevel::H1,
                    ..
                }
            )
        {
            self.after_h1 = false;
            page.opens_with_paragraph = Some(matches!(tag, Tag::Paragraph));
        }
        match ev {
            Event::Start(
                Tag::List(_) | Tag::BlockQuote(_) | Tag::Table(_) | Tag::FootnoteDefinition(_),
            ) => {
                self.depth += 1;
            }
            Event::End(
                TagEnd::List(_)
                | TagEnd::BlockQuote(_)
                | TagEnd::Table
                | TagEnd::FootnoteDefinition,
            ) => {
                self.depth = self.depth.saturating_sub(1);
            }
            Event::Start(Tag::CodeBlock(_)) => {
                let first = newlines.partition_point(|p| *p < range.start);
                let last = newlines.partition_point(|p| *p < range.end.saturating_sub(1));
                for flag in self.code.iter_mut().take(last + 1).skip(first) {
                    *flag = true;
                }
            }
            Event::Start(Tag::Paragraph) if self.depth == 0 => {
                let first = body
                    .get(range.clone())
                    .and_then(|p| p.lines().next())
                    .unwrap_or_default();
                let under_h2 = page.outline.iter().any(|h| h.level == 2);
                page.paragraphs.push((line, first.to_owned(), under_h2));
            }
            _ => {}
        }
    }
}

/// Parse Markdown, masking code spans and blocks before scanning literal wikilinks.
pub(crate) fn parse(file: &str, text: &str, doc: bool, out: &mut Vec<Finding>) -> Page {
    let mut page = Page::default();
    let offset = if doc {
        frontmatter(file, text, &mut page, out)
    } else {
        0
    };
    let body = text.get(offset..).unwrap_or_default();
    let base = text.get(..offset).unwrap_or_default().lines().count() + 1;
    let newlines: Vec<_> = body.match_indices('\n').map(|(i, _)| i).collect();
    let mut mask = body.as_bytes().to_vec();
    let mut heading = None;
    let mut level = 0;
    let mut in_covers = false;
    let mut compact = false;
    let mut seen = BTreeSet::new();
    let mut blocks = Blocks {
        code: vec![false; newlines.len() + 1],
        ..Blocks::default()
    };
    for (ev, range) in Parser::new(body).into_offset_iter() {
        let line = base + newlines.partition_point(|p| *p < range.start);
        blocks.observe(&ev, &range, line, body, &newlines, &mut page);
        match &ev {
            Event::Start(Tag::Paragraph) => {
                compact = body.get(range.clone()).is_some_and(|p| p.starts_with('`'));
            }
            Event::End(TagEnd::Paragraph) => {
                compact = false;
            }
            Event::Start(Tag::Heading { level: l, .. }) => {
                level = *l as u8;
                heading = Some(String::new());
                if level <= 2 {
                    in_covers = false;
                }
            }
            Event::Text(t) | Event::Code(t) if heading.is_some() => {
                if let Some(h) = &mut heading {
                    h.push_str(t);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(h) = heading.take() {
                    if level == 1 {
                        blocks.after_h1 = page.opens_with_paragraph.is_none();
                    }
                    in_covers |= page.add_heading(h, level, line, &mut seen);
                }
            }
            Event::Code(t) if in_covers => {
                let prefix = body
                    .get(..range.start)
                    .unwrap_or_default()
                    .rsplit('\n')
                    .next()
                    .unwrap_or_default()
                    .trim_start();
                // A list entry begins with its path; compact prose lists begin with a code span.
                if (matches!(prefix.trim_end(), "-" | "*" | "+") || compact)
                    && let Some(entries) = &mut page.covers
                {
                    entries.push((line, t.to_string()));
                }
            }
            Event::Start(Tag::Link { dest_url, .. } | Tag::Image { dest_url, .. })
                if !archived(file) && !dest_url.contains("://") && archived(dest_url) =>
            {
                out.push(Finding::new(
                    file,
                    line,
                    "E_ARCHIVE",
                    "Link to the current replacement page.",
                ));
            }
            _ => {}
        }
        if matches!(ev, Event::Code(_) | Event::Start(Tag::CodeBlock(_)))
            && let Some(bytes) = mask.get_mut(range)
        {
            for b in bytes {
                if *b != b'\n' {
                    *b = b' ';
                }
            }
        }
    }
    for (i, text) in body.lines().enumerate() {
        if !blocks.code.get(i).copied().unwrap_or_default() {
            page.lines.push((base + i, text.to_owned()));
        }
    }
    for (i, line) in String::from_utf8_lossy(&mask).lines().enumerate() {
        for caps in regex!(r"\[\[([^\[\]\n]+)\]\]").captures_iter(line) {
            if let Some(raw) = caps.get(1) {
                page.wiki.push((base + i, raw.as_str().to_owned()));
            }
        }
    }
    page
}
