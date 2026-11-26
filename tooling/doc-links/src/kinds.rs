//! Check each doc against the structure its kind requires, as declared in `kinds.toml`.
//! spec: docs/repo/doc-kinds.md#how-check-applies-a-kind
use crate::{
    Finding,
    markdown::{Heading, Page},
};
use regex::Regex;
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
    path::Path,
    process::Command,
};

/// Where the kinds file lives, relative to the repository root.
pub(crate) const FILE: &str = "tooling/doc-links/kinds.toml";

/// The kinds file as written.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConfig {
    /// Page that explains every kind.
    doc: String,
    /// Keys a page of no kind may carry.
    generic_keys: Vec<String>,
    /// Folders that skip every kind check for now.
    #[serde(default)]
    pending_migration: Vec<String>,
    /// Folders bound to one kind.
    #[serde(default)]
    folder: Vec<FolderRule>,
    /// The kinds.
    kind: Vec<RawKind>,
}

/// A folder whose every page is of one kind.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FolderRule {
    /// Folder glob.
    path: String,
    /// Kind name.
    kind: String,
}

/// One kind as written.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawKind {
    /// Kind name and heading of its section in the doc page.
    name: String,
    /// File ending, like `.screen.md`.
    ending: Option<String>,
    /// Exact file name, like `SKILL.md`.
    file: Option<String>,
    /// Folder globs where the kind may live.
    folders: Vec<String>,
    /// Pattern for the file name without its ending.
    stem: Option<String>,
    /// How to write a valid stem.
    #[serde(default)]
    stem_hint: String,
    /// Allowed frontmatter keys.
    keys: Vec<String>,
    /// Frontmatter keys that must be present.
    #[serde(default)]
    required_keys: Vec<String>,
    /// Allowed values per key.
    #[serde(default)]
    enums: BTreeMap<String, Vec<String>>,
    /// Pattern per key.
    #[serde(default)]
    patterns: BTreeMap<String, String>,
    /// A placeholder value that only an uncommitted file may hold.
    pending: Option<Pending>,
    /// H2 list; a trailing `?` marks an optional one.
    #[serde(default)]
    h2: Vec<String>,
    /// Known H2s must keep the list order.
    #[serde(default)]
    ordered: bool,
    /// No H2 outside the list.
    #[serde(default)]
    closed: bool,
    /// Page-wide shape rules.
    #[serde(default)]
    require: Vec<Require>,
    /// Numbered items written as headings.
    items: Option<RawItems>,
    /// A table whose rows must match.
    table: Option<RawTable>,
    /// Paragraph entries, like glossary terms.
    entries: Option<RawEntries>,
}

/// A page-wide shape rule.
#[derive(Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Require {
    /// Exactly one H1.
    OneH1,
    /// A paragraph right after the H1.
    ParagraphAfterH1,
    /// External links require a final References section.
    ReferencesWhenUrls,
}

impl Require {
    /// The name written in the kinds file.
    fn name(self) -> &'static str {
        match self {
            Self::OneH1 => "one-h1",
            Self::ParagraphAfterH1 => "paragraph-after-h1",
            Self::ReferencesWhenUrls => "references-when-urls",
        }
    }
}

/// The placeholder of a key that git must confirm.
#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct Pending {
    /// Frontmatter key.
    pub(crate) key: String,
    /// Placeholder value.
    pub(crate) value: String,
}

/// Items as written.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawItems {
    /// Heading level of every item.
    level: u8,
    /// Heading pattern; `{STEM}` is the upper-case file stem.
    heading: String,
    /// The heading form shown in a finding.
    example: String,
    /// Ids unique across the kind.
    #[serde(default)]
    unique: bool,
    /// One id family per page.
    #[serde(default)]
    one_family: bool,
    /// Each item has prose besides its required lines.
    #[serde(default)]
    body: bool,
    /// Lines each item must hold.
    #[serde(default)]
    lines: Vec<RawLine>,
}

/// A required line as written.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawLine {
    /// Pattern.
    pattern: String,
    /// The line shown in a finding.
    add: String,
}

/// A table rule as written.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTable {
    /// H2 holding the table.
    section: String,
    /// Pattern every data row matches.
    row: String,
    /// What each row needs, for the finding.
    add: String,
}

/// Paragraph entries as written.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawEntries {
    /// Pattern of an entry's first line, with an `id` group.
    pattern: String,
    /// Other allowed paragraph openings.
    #[serde(default)]
    also: Vec<String>,
    /// The entry form shown in a finding.
    example: String,
}

/// One H2 the kind knows.
struct Section {
    /// Heading text.
    name: String,
    /// May be left out.
    optional: bool,
}

/// A required item line.
struct Line {
    /// Compiled pattern.
    pattern: Regex,
    /// The line shown in a finding.
    add: String,
}

/// Compiled items rule.
struct Items {
    /// Heading level.
    level: u8,
    /// Heading pattern source, before `{STEM}` substitution.
    heading: String,
    /// Example heading.
    example: String,
    /// Ids unique across the kind.
    unique: bool,
    /// One id family per page.
    one_family: bool,
    /// Prose required.
    body: bool,
    /// Required lines.
    lines: Vec<Line>,
}

/// Compiled table rule.
struct Table {
    /// H2 holding the table.
    section: String,
    /// Row pattern.
    row: Regex,
    /// Row hint.
    add: String,
}

/// Compiled entries rule.
struct Entries {
    /// Entry pattern.
    pattern: Regex,
    /// Other allowed openings.
    also: Vec<Regex>,
    /// Example entry.
    example: String,
}

/// A kind ready to check pages.
pub(crate) struct Kind {
    /// Kind name.
    pub(crate) name: String,
    /// File ending.
    ending: Option<String>,
    /// Exact file name.
    file: Option<String>,
    /// Folder globs.
    folders: Vec<String>,
    /// Stem pattern and hint.
    stem: Option<(Regex, String)>,
    /// Allowed keys.
    keys: Vec<String>,
    /// Required keys.
    required_keys: Vec<String>,
    /// Allowed values.
    enums: BTreeMap<String, Vec<String>>,
    /// Key patterns.
    patterns: BTreeMap<String, Regex>,
    /// Placeholder confirmed by git.
    pub(crate) pending: Option<Pending>,
    /// Known H2s.
    h2: Vec<Section>,
    /// Order matters.
    ordered: bool,
    /// No other H2.
    closed: bool,
    /// Page-wide shape rules.
    require: Vec<Require>,
    /// Items rule.
    items: Option<Items>,
    /// Table rule.
    table: Option<Table>,
    /// Entries rule.
    entries: Option<Entries>,
}

/// The compiled kinds file.
pub(crate) struct Config {
    /// Page that explains every kind.
    pub(crate) doc: String,
    /// Keys for pages of no kind.
    generic_keys: Vec<String>,
    /// Folders exempt for now.
    pending_migration: Vec<String>,
    /// Folder bindings.
    folders: Vec<FolderRule>,
    /// Kinds in file order.
    pub(crate) kinds: Vec<Kind>,
}

/// An item id seen on a page, for the uniqueness pass.
pub(crate) struct Seen {
    /// Kind name.
    kind: String,
    /// Id.
    id: String,
    /// Page.
    file: String,
    /// Line.
    line: usize,
}

/// Compile one pattern from the kinds file, naming the faulty entry.
fn compile(src: &str) -> io::Result<Regex> {
    Regex::new(src).map_err(|e| io::Error::other(format!("{FILE}: invalid pattern {src}: {e}")))
}

/// Match a repository folder against a glob where `*` is one segment and `**` any number.
pub(crate) fn glob(pattern: &str, folder: &str) -> bool {
    /// Segment-wise match with backtracking over `**`.
    fn walk(p: &[&str], f: &[&str]) -> bool {
        match (p.split_first(), f.split_first()) {
            (None, None) => true,
            (Some((&"**", rest)), _) => {
                walk(rest, f) || f.split_first().is_some_and(|(_, tail)| walk(p, tail))
            }
            (Some((seg, rest)), Some((name, tail))) => {
                (*seg == "*" || seg == name) && walk(rest, tail)
            }
            _ => false,
        }
    }
    let p: Vec<_> = pattern.split('/').collect();
    let f: Vec<_> = folder.split('/').collect();
    walk(&p, &f)
}

impl Kind {
    /// Compile a kind; every pattern error names the kinds file.
    fn compile(raw: RawKind) -> io::Result<Self> {
        let patterns = raw
            .patterns
            .iter()
            .map(|(k, v)| Ok((k.clone(), compile(v)?)))
            .collect::<io::Result<_>>()?;
        let items = match raw.items {
            Some(i) => {
                // The {STEM} placeholder must compile once replaced.
                let _ = compile(&i.heading.replace("{STEM}", "STEM"))?;
                Some(Items {
                    level: i.level,
                    heading: i.heading,
                    example: i.example,
                    unique: i.unique,
                    one_family: i.one_family,
                    body: i.body,
                    lines: i
                        .lines
                        .into_iter()
                        .map(|l| {
                            Ok(Line {
                                pattern: compile(&l.pattern)?,
                                add: l.add,
                            })
                        })
                        .collect::<io::Result<_>>()?,
                })
            }
            None => None,
        };
        let table = match raw.table {
            Some(t) => Some(Table {
                row: compile(&t.row)?,
                section: t.section,
                add: t.add,
            }),
            None => None,
        };
        let entries = match raw.entries {
            Some(e) => Some(Entries {
                pattern: compile(&e.pattern)?,
                also: e
                    .also
                    .iter()
                    .map(|a| compile(a))
                    .collect::<io::Result<_>>()?,
                example: e.example,
            }),
            None => None,
        };
        if raw.ending.is_some() == raw.file.is_some() {
            return Err(io::Error::other(format!(
                "{FILE}: kind {} needs exactly one of ending or file",
                raw.name
            )));
        }
        Ok(Self {
            stem: match raw.stem {
                Some(s) => Some((compile(&s)?, raw.stem_hint)),
                None => None,
            },
            h2: raw
                .h2
                .iter()
                .map(|h| Section {
                    name: h.trim_end_matches('?').to_owned(),
                    optional: h.ends_with('?'),
                })
                .collect(),
            name: raw.name,
            ending: raw.ending,
            file: raw.file,
            folders: raw.folders,
            keys: raw.keys,
            required_keys: raw.required_keys,
            enums: raw.enums,
            patterns,
            pending: raw.pending,
            ordered: raw.ordered,
            closed: raw.closed,
            require: raw.require,
            items,
            table,
            entries,
        })
    }

    /// The file pattern as shown to a reader: `*.screen.md` or `SKILL.md`.
    pub(crate) fn pattern(&self) -> String {
        self.ending.as_ref().map_or_else(
            || self.file.clone().unwrap_or_default(),
            |e| format!("*{e}"),
        )
    }

    /// The folder globs, comma-separated.
    pub(crate) fn folders_list(&self) -> String {
        self.folders.join(",")
    }

    /// One line describing what the kind requires.
    pub(crate) fn summary(&self) -> String {
        let mut parts = vec![format!("keys={}", self.keys.join(","))];
        if !self.required_keys.is_empty() {
            parts.push(format!("required={}", self.required_keys.join(",")));
        }
        for (k, v) in &self.enums {
            parts.push(format!("{k}={}", v.join("|")));
        }
        for (k, v) in &self.patterns {
            parts.push(format!("{k}~{}", v.as_str()));
        }
        if !self.h2.is_empty() {
            let list: Vec<_> = self
                .h2
                .iter()
                .map(|s| format!("{}{}", s.name, if s.optional { "?" } else { "" }))
                .collect();
            let mut mode = Vec::new();
            if self.ordered {
                mode.push("ordered");
            }
            if self.closed {
                mode.push("closed");
            }
            parts.push(format!(
                "h2=[{}]{}",
                list.join("; "),
                if mode.is_empty() {
                    String::new()
                } else {
                    format!("({})", mode.join(","))
                }
            ));
        }
        parts.extend(self.require.iter().map(|r| r.name().to_owned()));
        if let Some(i) = &self.items {
            let lines: Vec<_> = i.lines.iter().map(|l| l.add.as_str()).collect();
            parts.push(format!(
                "items=h{} \"{}\"{}{} lines=[{}]",
                i.level,
                i.example,
                if i.unique { " unique" } else { "" },
                if i.one_family { " one-family" } else { "" },
                lines.join("; ")
            ));
        }
        if let Some(t) = &self.table {
            parts.push(format!("table={}: {}", t.section, t.add));
        }
        if let Some(e) = &self.entries {
            parts.push(format!("entries=\"{}\" unique", e.example));
        }
        parts.join(" ")
    }
}

impl Config {
    /// Read the kinds file under `root`; `None` when the repository has none.
    pub(crate) fn load(root: &Path) -> io::Result<Option<Self>> {
        let text = match fs::read_to_string(root.join(FILE)) {
            Ok(t) => t,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(io::Error::new(e.kind(), format!("{FILE}: {e}"))),
        };
        let raw: RawConfig =
            toml::from_str(&text).map_err(|e| io::Error::other(format!("{FILE}: {e}")))?;
        let kinds = raw
            .kind
            .into_iter()
            .map(Kind::compile)
            .collect::<io::Result<Vec<_>>>()?;
        for rule in &raw.folder {
            if !kinds.iter().any(|k| k.name == rule.kind) {
                return Err(io::Error::other(format!(
                    "{FILE}: folder {} names unknown kind {}",
                    rule.path, rule.kind
                )));
            }
        }
        Ok(Some(Self {
            doc: raw.doc,
            generic_keys: raw.generic_keys,
            pending_migration: raw.pending_migration,
            folders: raw.folder,
            kinds,
        }))
    }

    /// The kind a file name belongs to: an exact name first, then the longest ending.
    pub(crate) fn kind_of(&self, file: &str) -> Option<&Kind> {
        let base = file.rsplit('/').next().unwrap_or(file);
        self.kinds
            .iter()
            .find(|k| k.file.as_deref() == Some(base))
            .or_else(|| {
                self.kinds
                    .iter()
                    .filter(|k| {
                        k.ending
                            .as_ref()
                            .is_some_and(|e| base.ends_with(e.as_str()))
                    })
                    .max_by_key(|k| k.ending.as_ref().map_or(0, String::len))
            })
    }

    /// Whether a page still waits for its rename.
    fn migrating(&self, file: &str) -> bool {
        self.pending_migration
            .iter()
            .any(|p| Path::new(file).starts_with(p))
    }

    /// The suffix every kind finding ends with.
    fn cite(&self, kind: &str) -> String {
        format!("(kind {kind}, {}#{kind})", self.doc)
    }

    /// Check one page against its kind and its folder; record item ids for [`Config::unique`].
    pub(crate) fn check(
        &self,
        file: &str,
        page: &Page,
        seen: &mut Vec<Seen>,
        out: &mut Vec<Finding>,
    ) {
        if self.migrating(file) {
            return;
        }
        let folder = file.rsplit_once('/').map_or("", |(d, _)| d);
        let base = file.rsplit('/').next().unwrap_or(file);
        let kind = self.kind_of(file);
        // A kind named by its exact file name (an index) may sit in any folder it lists.
        let fixed =
            kind.is_some_and(|k| k.file.is_some() && k.folders.iter().any(|g| glob(g, folder)));
        let mut misplaced = false;
        for rule in self.folders.iter().filter(|r| glob(&r.path, folder)) {
            if !fixed
                && kind.is_none_or(|k| k.name != rule.kind)
                && let Some(want) = self.kinds.iter().find(|k| k.name == rule.kind)
            {
                let stem = base.trim_end_matches(".md");
                let name = want.ending.as_ref().map_or_else(
                    || want.pattern(),
                    |e| {
                        if stem.starts_with('_')
                            || want.stem.as_ref().is_some_and(|(re, _)| re.is_match(stem))
                        {
                            format!("{stem}{e}")
                        } else {
                            format!(
                                "<{}>{e}",
                                want.stem.as_ref().map_or("name", |(_, h)| h.as_str())
                            )
                        }
                    },
                );
                out.push(Finding::new(
                    file,
                    1,
                    "E_PLACE",
                    format!(
                        "rename to {name}: {folder}/ holds only {} pages {}",
                        want.pattern(),
                        self.cite(&want.name)
                    ),
                ));
                misplaced = true;
            }
        }
        if misplaced {
            return;
        }
        let Some(kind) = kind else {
            self.keys(file, page, &self.generic_keys, out, "plain page");
            return;
        };
        let cite = self.cite(&kind.name);
        if !kind.folders.iter().any(|g| glob(g, folder)) {
            out.push(Finding::new(
                file,
                1,
                "E_PLACE",
                format!(
                    "move this {} page under {} {cite}",
                    kind.pattern(),
                    kind.folders.join(" or ")
                ),
            ));
        }
        let template = base.starts_with('_');
        let stem = kind
            .ending
            .as_ref()
            .and_then(|e| base.strip_suffix(e.as_str()))
            .unwrap_or_default();
        if !template
            && let Some((re, hint)) = &kind.stem
            && !re.is_match(stem)
        {
            out.push(Finding::new(
                file,
                1,
                "E_NAME",
                format!(
                    "rename the file to {hint}{} {cite}",
                    kind.ending.as_deref().unwrap_or("")
                ),
            ));
        }
        self.keys(file, page, &kind.keys, out, &cite);
        values(file, page, kind, &cite, out);
        sections(file, page, kind, template, &cite, out);
        if let Some(items) = &kind.items {
            self.items(file, page, kind, items, template, seen, out);
        }
        if !template && let Some(table) = &kind.table {
            rows(file, page, table, &cite, out);
        }
        if !template && let Some(entries) = &kind.entries {
            paragraphs(file, page, entries, &cite, out);
        }
    }

    /// Reject frontmatter keys the kind does not allow.
    fn keys(
        &self,
        file: &str,
        page: &Page,
        allowed: &[String],
        out: &mut Vec<Finding>,
        cite: &str,
    ) {
        for (line, key, _) in &page.meta {
            if !allowed.contains(key) {
                let cite = if cite == "plain page" {
                    format!("(plain page, {}#plain-pages)", self.doc)
                } else {
                    cite.to_owned()
                };
                out.push(Finding::new(
                    file,
                    *line,
                    "E_META",
                    format!(
                        "remove the frontmatter key {key}; allowed: {} {cite}",
                        allowed.join(", ")
                    ),
                ));
            }
        }
    }

    /// Check item headings, their lines and their family; collect ids.
    #[expect(
        clippy::too_many_arguments,
        reason = "One call site passes the page context through."
    )]
    fn items(
        &self,
        file: &str,
        page: &Page,
        kind: &Kind,
        items: &Items,
        template: bool,
        seen: &mut Vec<Seen>,
        out: &mut Vec<Finding>,
    ) {
        let cite = self.cite(&kind.name);
        let base = file.rsplit('/').next().unwrap_or(file);
        let stem = kind
            .ending
            .as_ref()
            .and_then(|e| base.strip_suffix(e.as_str()))
            .unwrap_or_default()
            .to_ascii_uppercase();
        let Ok(heading) = Regex::new(&items.heading.replace("{STEM}", &regex::escape(&stem)))
        else {
            return;
        };
        let example = items.example.replace("{STEM}", &stem);
        let hashes = "#".repeat(usize::from(items.level));
        let mut family: Option<String> = None;
        for (i, h) in page.outline.iter().enumerate() {
            if h.level != items.level {
                continue;
            }
            let Some(caps) = heading.captures(&h.text) else {
                out.push(Finding::new(
                    file,
                    h.line,
                    "E_ITEM",
                    format!(
                        "rename the heading \"{}\" to {hashes} {example} {cite}",
                        h.text
                    ),
                ));
                continue;
            };
            if let Some(id) = caps.name("id")
                && items.unique
            {
                seen.push(Seen {
                    kind: kind.name.clone(),
                    id: id.as_str().to_owned(),
                    file: file.to_owned(),
                    line: h.line,
                });
            }
            if items.one_family
                && let Some(f) = caps.name("family")
            {
                match &family {
                    Some(first) if first != f.as_str() => out.push(Finding::new(
                        file,
                        h.line,
                        "E_ITEM",
                        format!(
                            "use this page's prefix {first} for \"{}\": one prefix per page {cite}",
                            h.text
                        ),
                    )),
                    Some(_) => {}
                    None => family = Some(f.as_str().to_owned()),
                }
            }
            if template {
                continue;
            }
            let end = page
                .outline
                .get(i + 1..)
                .and_then(|rest| rest.iter().find(|n| n.level <= items.level))
                .map_or(usize::MAX, |n| n.line);
            let body: Vec<&str> = page
                .lines
                .iter()
                .filter(|(n, t)| {
                    *n > h.line && *n < end && !t.trim().is_empty() && !t.starts_with('#')
                })
                .map(|(_, t)| t.as_str())
                .collect();
            for line in &items.lines {
                if !body.iter().any(|t| line.pattern.is_match(t)) {
                    out.push(Finding::new(
                        file,
                        h.line,
                        "E_ITEM",
                        format!("add the line \"{}\" under \"{}\" {cite}", line.add, h.text),
                    ));
                }
            }
            if items.body
                && !body
                    .iter()
                    .any(|t| !items.lines.iter().any(|l| l.pattern.is_match(t)))
            {
                out.push(Finding::new(
                    file,
                    h.line,
                    "E_ITEM",
                    format!("write the text of \"{}\" below its heading {cite}", h.text),
                ));
            }
        }
    }

    /// Report ids used twice within one kind, at every later use.
    pub(crate) fn unique(&self, mut seen: Vec<Seen>, out: &mut Vec<Finding>) {
        seen.sort_by(|a, b| {
            (&a.kind, &a.id, &a.file, a.line).cmp(&(&b.kind, &b.id, &b.file, b.line))
        });
        let mut first: BTreeMap<(&str, &str), (&str, usize)> = BTreeMap::new();
        for s in &seen {
            if let Some((file, line)) = first.get(&(s.kind.as_str(), s.id.as_str())) {
                out.push(Finding::new(
                    &s.file,
                    s.line,
                    "E_ITEM",
                    format!(
                        "{} is already used at {file}:{line}; give this item the next free number, never reuse one {}",
                        s.id,
                        self.cite(&s.kind)
                    ),
                ));
            } else {
                let _ = first.insert((&s.kind, &s.id), (&s.file, s.line));
            }
        }
    }
}

/// Check required keys, allowed values and value patterns.
fn values(file: &str, page: &Page, kind: &Kind, cite: &str, out: &mut Vec<Finding>) {
    for key in &kind.required_keys {
        if !page.meta.iter().any(|(_, k, _)| k == key) {
            out.push(Finding::new(
                file,
                1,
                "E_META",
                format!("add the frontmatter key {key} {cite}"),
            ));
        }
    }
    for (line, key, value) in &page.meta {
        let value = value.as_deref().unwrap_or_default();
        if let Some(allowed) = kind.enums.get(key)
            && !allowed.iter().any(|a| a == value)
        {
            out.push(Finding::new(
                file,
                *line,
                "E_META",
                format!("set {key} to one of {} {cite}", allowed.join(", ")),
            ));
        }
        if let Some(re) = kind.patterns.get(key)
            && !re.is_match(value)
        {
            out.push(Finding::new(
                file,
                *line,
                "E_META",
                format!("set {key} to a value matching {} {cite}", re.as_str()),
            ));
        }
    }
}

/// Check the known H2s: presence, order, strangers, content and the H1 shape.
fn sections(
    file: &str,
    page: &Page,
    kind: &Kind,
    template: bool,
    cite: &str,
    out: &mut Vec<Finding>,
) {
    let h1: Vec<_> = page.outline.iter().filter(|h| h.level == 1).collect();
    if kind.require.contains(&Require::OneH1) && h1.len() != 1 {
        let line = h1.get(1).map_or(1, |h| h.line);
        out.push(Finding::new(
            file,
            line,
            "E_KIND",
            format!("keep exactly one H1 title {cite}"),
        ));
    }
    if kind.require.contains(&Require::ParagraphAfterH1)
        && !template
        && page.opens_with_paragraph != Some(true)
    {
        let line = h1.first().map_or(1, |h| h.line);
        out.push(Finding::new(
            file,
            line,
            "E_KIND",
            format!("start the body with one paragraph right after the H1 {cite}"),
        ));
    }
    let h2: Vec<_> = page.outline.iter().filter(|h| h.level == 2).collect();
    let index = |text: &str| kind.h2.iter().position(|s| s.name == text);
    let mut last: Option<(usize, &str)> = None;
    let mut present = BTreeMap::new();
    for h in &h2 {
        let Some(at) = index(&h.text) else {
            if kind.closed {
                let names: Vec<_> = kind.h2.iter().map(|s| s.name.as_str()).collect();
                out.push(Finding::new(
                    file,
                    h.line,
                    "E_KIND",
                    format!(
                        "remove or rename the H2 \"{}\"; allowed: {} {cite}",
                        h.text,
                        if names.is_empty() {
                            "none".to_owned()
                        } else {
                            names.join(", ")
                        }
                    ),
                ));
            }
            continue;
        };
        if present.insert(at, h.line).is_some() {
            out.push(Finding::new(
                file,
                h.line,
                "E_KIND",
                format!("keep one H2 \"{}\" {cite}", h.text),
            ));
            continue;
        }
        if kind.ordered
            && let Some((before, name)) = last
            && at < before
        {
            out.push(Finding::new(
                file,
                h.line,
                "E_KIND",
                format!("move the H2 \"{}\" above \"{name}\" {cite}", h.text),
            ));
        }
        if last.is_none_or(|(b, _)| at > b) {
            last = Some((at, &h.text));
        }
    }
    missing(
        file,
        kind,
        &present,
        h1.first().map_or(1, |h| h.line),
        cite,
        out,
    );
    if !template {
        content(file, page, kind, &h2, cite, out);
    }
}

/// Report each required H2 that is absent, with where to add it.
fn missing(
    file: &str,
    kind: &Kind,
    present: &BTreeMap<usize, usize>,
    line: usize,
    cite: &str,
    out: &mut Vec<Finding>,
) {
    for (at, section) in kind.h2.iter().enumerate() {
        if section.optional || present.contains_key(&at) {
            continue;
        }
        let near = |i: &usize| present.get(i).copied();
        let after = kind
            .h2
            .get(..at)
            .and_then(|s| {
                s.iter()
                    .enumerate()
                    .rev()
                    .find_map(|(i, s)| Some((near(&i)?, s)))
            })
            .map(|(l, s)| (l, format!("after \"{}\"", s.name)));
        let (line, place) = after.unwrap_or_else(|| {
            kind.h2
                .iter()
                .enumerate()
                .skip(at + 1)
                .find_map(|(i, s)| Some((near(&i)?, s)))
                .map_or_else(
                    || (line, "after the H1".to_owned()),
                    |(l, s)| (l, format!("before \"{}\"", s.name)),
                )
        });
        out.push(Finding::new(
            file,
            line,
            "E_KIND",
            format!("add the H2 \"{}\" {place} {cite}", section.name),
        ));
    }
}

/// Check that known sections hold text and that cited sources are listed.
fn content(
    file: &str,
    page: &Page,
    kind: &Kind,
    h2: &[&Heading],
    cite: &str,
    out: &mut Vec<Finding>,
) {
    let index = |text: &str| kind.h2.iter().position(|s| s.name == text);
    for (i, h) in h2.iter().enumerate() {
        if h.text == "Covers" || index(&h.text).is_none() {
            continue;
        }
        let end = h2.get(i + 1).map_or(usize::MAX, |n| n.line);
        let filled = page
            .lines
            .iter()
            .any(|(n, t)| *n > h.line && *n < end && !t.trim().is_empty())
            || page
                .outline
                .iter()
                .any(|o| o.level > 2 && o.line > h.line && o.line < end);
        if !filled {
            out.push(Finding::new(
                file,
                h.line,
                "E_KIND",
                format!("write the content of the H2 \"{}\" {cite}", h.text),
            ));
        }
    }
    if kind.require.contains(&Require::ReferencesWhenUrls)
        && !h2.iter().any(|h| h.text == "References")
    {
        let refs = regex_url();
        if let Some((line, _)) = page.lines.iter().find(|(_, t)| refs.is_match(t)) {
            out.push(Finding::new(
                file,
                *line,
                "E_KIND",
                format!("add a final H2 \"References\" listing each cited source as - [Title](url) {cite}"),
            ));
        }
    }
}

/// An external link in Markdown or angle-bracket form.
fn regex_url() -> &'static Regex {
    crate::regex!(r"\]\(https?://|<https?://")
}

/// Check every data row of the kind's table.
fn rows(file: &str, page: &Page, table: &Table, cite: &str, out: &mut Vec<Finding>) {
    let Some(at) = page
        .outline
        .iter()
        .position(|h| h.level == 2 && h.text == table.section)
    else {
        return;
    };
    let start = page.outline.get(at).map_or(0, |h| h.line);
    let end = page
        .outline
        .get(at + 1..)
        .and_then(|r| r.iter().find(|h| h.level <= 2))
        .map_or(usize::MAX, |h| h.line);
    let table_rows: Vec<_> = page
        .lines
        .iter()
        .filter(|(n, t)| *n > start && *n < end && t.trim_start().starts_with('|'))
        .collect();
    let data = table_rows.iter().skip(2);
    if table_rows.len() < 3 {
        out.push(Finding::new(
            file,
            start,
            "E_KIND",
            format!(
                "add a table under \"{}\" with one row per page, each with {} {cite}",
                table.section, table.add
            ),
        ));
    }
    for (line, text) in data {
        if !table.row.is_match(text) {
            out.push(Finding::new(
                file,
                *line,
                "E_KIND",
                format!(
                    "give this row of \"{}\" {} {cite}",
                    table.section, table.add
                ),
            ));
        }
    }
}

/// Check paragraph entries after the first H2 and their uniqueness on the page.
fn paragraphs(file: &str, page: &Page, entries: &Entries, cite: &str, out: &mut Vec<Finding>) {
    let mut ids = BTreeMap::new();
    for (line, first, under_h2) in &page.paragraphs {
        if !under_h2 {
            continue;
        }
        if let Some(caps) = entries.pattern.captures(first) {
            let id = caps.name("id").map_or("", |m| m.as_str()).to_owned();
            if let Some(prev) = ids.insert(id.clone(), *line) {
                out.push(Finding::new(
                    file,
                    *line,
                    "E_ITEM",
                    format!("{id} is already defined at line {prev}; merge the two entries {cite}"),
                ));
            }
        } else if !entries.also.iter().any(|a| a.is_match(first)) {
            out.push(Finding::new(
                file,
                *line,
                "E_KIND",
                format!(
                    "start this paragraph as an entry: {} {cite}",
                    entries.example
                ),
            ));
        }
    }
}

/// Pages whose placeholder value is stale: committed before `HEAD`. Calls git only when a placeholder exists.
///
/// WHY: a commit cannot hold its own hash, so the placeholder is fine while the file is uncommitted or was added by
/// `HEAD`; any later commit must have filled it in. Outside a git repository nothing is committed, so nothing is stale.
pub(crate) fn stale(git_root: &Path, files: &[String]) -> BTreeSet<String> {
    if files.is_empty() {
        return BTreeSet::new();
    }
    let tree = |rev: &str| -> BTreeSet<String> {
        Command::new("git")
            .current_dir(git_root)
            .args(["ls-tree", "-r", "--name-only", rev, "--"])
            .args(files)
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| {
                String::from_utf8_lossy(&o.stdout)
                    .lines()
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default()
    };
    let before = tree("HEAD~1");
    tree("HEAD")
        .into_iter()
        .filter(|f| before.contains(f))
        .collect()
}

#[cfg(test)]
mod tests {
    //! Folder glob semantics.
    use super::glob;

    /// `*` takes one segment; `**` takes any number, including none.
    #[test]
    fn globs() {
        assert!(glob("docs/**/.old", "docs/.old"));
        assert!(glob("docs/**/.old", "docs/repo/turborepo/.old"));
        assert!(glob("docs/**", "docs"));
        assert!(glob("docs/*", "docs/repo"));
        assert!(!glob("docs/*", "docs/repo/turborepo"));
        assert!(!glob("docs/screens", "docs/screens/assets"));
    }
}
