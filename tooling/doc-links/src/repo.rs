//! Build a deterministic snapshot for validation, queries and graph enrichment.
//! spec: docs/repo/doc-links.md#what-check-verifies
use crate::{
    Finding, archived,
    graph::{Link, Place},
    kinds::{self, Config},
    markdown::{self, Page},
    regex,
    source::{self, Citation},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
    path::{Component, Path},
};
use walkdir::WalkDir;

/// Return whether this file belongs to the source/config citation scan.
fn code(file: &str) -> bool {
    !crate::extension(file, "md")
        && !crate::extension(file, "lock")
        && (!file.contains('/')
            || [
                "apps/",
                "packages/",
                "tooling/",
                "bin/",
                ".agents/hooks/",
                ".husky/",
            ]
            .iter()
            .any(|r| file.starts_with(r)))
}

/// Repository files, resolved relationships and actionable diagnostics.
#[derive(Default)]
pub(crate) struct Repo {
    /// Vault pages, including history.
    pub(crate) pages: BTreeMap<String, Page>,
    /// Rule ID to page and optional heading.
    rules: BTreeMap<String, (String, Option<String>)>,
    /// Source path to citations.
    sources: BTreeMap<String, Vec<Citation>>,
    /// Existing code paths (including those without citations).
    code: BTreeSet<String>,
    /// Findings from both read and resolution passes.
    pub(crate) findings: Vec<Finding>,
    /// Checked relationships shared between queries and graph output.
    pub(crate) links: Vec<Link>,
    /// The doc kinds, when the repository declares them.
    pub(crate) kinds: Option<Config>,
}

/// A template of its kind: a file name starting with `_`.
pub(crate) fn template(file: &str) -> bool {
    file.rsplit('/').next().is_some_and(|b| b.starts_with('_'))
}

/// Existing coverage for a file, or a directory written with a trailing slash.
pub(crate) fn matches_path(entry: &str, path: &str) -> bool {
    entry == path || (entry.ends_with('/') && path.starts_with(entry))
}

/// Decide which generated or private directories to prune before reading files.
fn included(entry: &walkdir::DirEntry) -> bool {
    entry.depth() == 0
        || !matches!(
            entry.file_name().to_str(),
            Some(
                ".git"
                    | "vendor"
                    | "node_modules"
                    | "target"
                    | "graphify-out"
                    | ".obsidian"
                    | ".turbo"
                    | "dist"
                    | ".env"
            )
        ) && !entry.file_name().to_string_lossy().starts_with(".env.")
}

impl Repo {
    /// Read the complete repository; git runs in `git_root` only to confirm a pending placeholder.
    pub(crate) fn load(root: &Path, git_root: &Path) -> io::Result<Self> {
        if !root.join("docs").is_dir() {
            return Err(io::Error::other(
                "missing docs directory; run from the repository root",
            ));
        }
        let mut repo = Self {
            kinds: Config::load(root)?,
            ..Self::default()
        };
        for entry in WalkDir::new(root).into_iter().filter_entry(included) {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }
            let file = entry
                .path()
                .strip_prefix(root)
                .map_err(io::Error::other)?
                .to_str()
                .ok_or_else(|| io::Error::other("repository path is not UTF-8"))?;
            let bytes = fs::read(entry.path())
                .map_err(|e| io::Error::new(e.kind(), format!("{file}: {e}")))?;
            let doc = file.starts_with("docs/") && crate::extension(file, "md");
            if code(file) {
                let _ = repo.code.insert(file.to_owned());
            }
            let Ok(text) = std::str::from_utf8(&bytes) else {
                if doc {
                    return Err(io::Error::other(format!("{file}: use UTF-8 Markdown")));
                }
                continue;
            };
            if doc {
                let page = markdown::parse(file, text, true, &mut repo.findings);
                repo.index(file, &page);
                let _ = repo.pages.insert(file.to_owned(), page);
            } else {
                if !archived(file) && text.contains(".old/") {
                    let page = markdown::parse(file, text, false, &mut repo.findings);
                    for (line, raw) in page.wiki {
                        if archived(markdown::target(&raw)) {
                            repo.findings.push(Finding::new(
                                file,
                                line,
                                "E_ARCHIVE",
                                "Link to the replacement page.",
                            ));
                        }
                    }
                }
                if code(file) && !archived(file) {
                    let _ = repo
                        .sources
                        .insert(file.to_owned(), source::scan(file, text));
                }
            }
        }
        repo.validate(root);
        repo.validate_kinds(git_root);
        repo.findings.sort();
        repo.findings.dedup();
        Ok(repo)
    }

    /// Record only canonical invariant and ADR definitions.
    fn index(&mut self, file: &str, page: &Page) {
        if file
            .rsplit_once('/')
            .is_some_and(|(p, _)| p == "docs/invariants")
        {
            for (id, slug) in &page.rules {
                let _ = self
                    .rules
                    .insert(id.clone(), (file.to_owned(), Some(slug.clone())));
            }
        }
        if let Some(cap) = regex!(r"^docs/adr/(\d{4})-[^/]+\.adr\.md$").captures(file)
            && let Some(n) = cap.get(1)
        {
            let _ = self
                .rules
                .insert(format!("ADR {}", n.as_str()), (file.to_owned(), None));
        }
    }

    /// Resolve a spec, screen, invariant or decision, including heading checks.
    pub(crate) fn resolve(&self, query: &str) -> Option<(String, Option<String>)> {
        if let Some(rule) = self.rules.get(query) {
            return Some(rule.clone());
        }
        let (path, slug) = query
            .split_once('#')
            .map_or((query, None), |(p, s)| (p, Some(s)));
        let page = self.pages.get(path)?;
        if archived(path)
            || slug.is_some_and(|s| !page.headings.iter().any(|(_, anchor)| anchor == s))
        {
            return None;
        }
        Some((path.to_owned(), slug.map(str::to_owned)))
    }

    /// Resolve the vault, its navigation tree and all code citations after indexing.
    fn validate(&mut self, root: &Path) {
        let mut nav = BTreeSet::new();
        for (file, page) in &self.pages {
            for (line, raw) in &page.wiki {
                let (dest, heading) = markdown::destination(file, raw);
                if !archived(file) && archived(&dest) {
                    self.findings.push(Finding::new(
                        file,
                        *line,
                        "E_ARCHIVE",
                        "Use the replacement page instead of the archived target.",
                    ));
                } else if !self.pages.get(&dest).is_some_and(|p| {
                    heading
                        .as_ref()
                        .is_none_or(|h| p.headings.iter().any(|(name, _)| name == h))
                }) {
                    self.findings.push(Finding::new(
                        file,
                        *line,
                        "E_WIKI",
                        format!("Unknown [[{raw}]]. Use an existing vault path and heading text."),
                    ));
                } else {
                    let _ = nav.insert((file.clone(), dest));
                }
            }
            if archived(file) {
                continue;
            }
            if let Some(entries) = &page.covers {
                if template(file) {
                    continue;
                }
                if entries.is_empty() {
                    self.findings.push(Finding::new(
                        file,
                        1,
                        "E_COVERS",
                        "List repository paths in backticks under ## Covers.",
                    ));
                }
                for (line, path) in entries {
                    if archived(path) {
                        self.findings.push(Finding::new(
                            file,
                            *line,
                            "E_ARCHIVE",
                            "Replace the archive entry with an active path.",
                        ));
                    } else if Path::new(path)
                        .components()
                        .any(|c| !matches!(c, Component::Normal(_)))
                        || !root.join(path).exists()
                    {
                        let remedy = format!(
                            "Replace missing or non-repository path {path} with an existing path."
                        );
                        self.findings
                            .push(Finding::new(file, *line, "E_COVERS", remedy));
                    } else if root.join(path).is_dir() && !path.ends_with('/') {
                        self.findings.push(Finding::new(
                            file,
                            *line,
                            "E_COVERS",
                            format!("Write directory {path}/ with a trailing slash."),
                        ));
                    } else {
                        for to in self.code.iter().filter(|p| matches_path(path, p)) {
                            self.links.push(Link {
                                from: Place::Heading(file.clone(), "covers".into()),
                                to: Place::File(to.clone()),
                                relation: "covers",
                                line: *line,
                                source_file: file.clone(),
                                context: path.clone(),
                            });
                        }
                    }
                }
            }
        }
        self.validate_indexes(&nav);
        self.validate_citations();
    }

    /// Check every page against its kind, ids across pages, the kinds page, and stale placeholders.
    fn validate_kinds(&mut self, git_root: &Path) {
        let Some(config) = &self.kinds else { return };
        let mut seen = Vec::new();
        let mut pending = Vec::new();
        for (file, page) in &self.pages {
            config.check(file, page, &mut seen, &mut self.findings);
            if let Some(p) = config.kind_of(file).and_then(|k| k.pending.as_ref())
                && page
                    .meta
                    .iter()
                    .any(|(_, k, v)| *k == p.key && v.as_deref() == Some(p.value.as_str()))
            {
                pending.push(file.clone());
            }
        }
        config.unique(seen, &mut self.findings);
        for file in kinds::stale(git_root, &pending) {
            self.findings.push(Finding::new(
                &file,
                1,
                "E_PENDING",
                format!(
                    "replace commit: pending with the hash from git log --diff-filter=A -1 --format=%h -- {file} (kind old, {}#old)",
                    config.doc
                ),
            ));
        }
        let headings = self.pages.get(&config.doc).map(|p| {
            p.headings
                .iter()
                .map(|(h, _)| h.as_str())
                .collect::<BTreeSet<_>>()
        });
        for kind in &config.kinds {
            if !headings
                .as_ref()
                .is_some_and(|h| h.contains(kind.name.as_str()))
            {
                self.findings.push(Finding::new(
                    kinds::FILE,
                    1,
                    "E_KINDS",
                    format!(
                        "add a heading \"{}\" to {} that explains the kind",
                        kind.name, config.doc
                    ),
                ));
            }
        }
    }

    /// Check child-to-parent index reachability for every active page.
    fn validate_indexes(&mut self, nav: &BTreeSet<(String, String)>) {
        let mut required = BTreeSet::from(["docs/SKILL.md".to_owned()]);
        for file in self.pages.keys().filter(|f| !archived(f)) {
            for parent in Path::new(file)
                .ancestors()
                .skip(1)
                .take_while(|p| !p.as_os_str().is_empty())
            {
                let _ = required.insert(parent.join("SKILL.md").display().to_string());
            }
            if file == "docs/SKILL.md" {
                continue;
            }
            let mut parent = Path::new(file).parent();
            if file.ends_with("/SKILL.md") {
                parent = parent.and_then(Path::parent);
            }
            if let Some(parent) = parent {
                let index = parent.join("SKILL.md").display().to_string();
                if !nav.contains(&(index.clone(), file.clone())) {
                    self.findings.push(Finding::new(
                        file,
                        1,
                        "E_INDEX",
                        format!(
                            "Add [[{}]] to {index}.",
                            file.trim_start_matches("docs/").trim_end_matches(".md")
                        ),
                    ));
                }
            }
        }
        for index in required {
            if !self.pages.contains_key(&index) {
                self.findings.push(Finding::new(
                    &index,
                    1,
                    "E_SCOPE",
                    "Create the scope index with name, description, tags and links to its pages.",
                ));
            }
        }
    }

    /// Place only successfully resolved source references in the graph.
    fn validate_citations(&mut self) {
        for (file, cites) in &self.sources {
            for cite in cites {
                if let Some((doc, slug)) = self.resolve(&cite.target) {
                    self.links.push(Link {
                        from: cite.item.map_or_else(
                            || Place::File(file.clone()),
                            |l| Place::ItemAt(file.clone(), l),
                        ),
                        to: slug.map_or_else(
                            || Place::File(doc.clone()),
                            |s| Place::Heading(doc.clone(), s),
                        ),
                        relation: if cite.test { "validates" } else { "cites" },
                        source_file: file.clone(),
                        line: cite.line,
                        context: cite.target.clone(),
                    });
                } else {
                    self.findings.push(Finding::new(
                        file,
                        cite.line,
                        "E_CITE",
                        format!(
                            "Replace {} with an existing active page, heading or rule ID.",
                            cite.target
                        ),
                    ));
                }
            }
        }
    }

    /// Invariants and covered docs that still need a test citation.
    pub(crate) fn matrix(&self) -> Vec<String> {
        let mut missing: BTreeSet<String> = self
            .pages
            .iter()
            .filter(|(p, d)| !archived(p) && !template(p) && d.covers.is_some())
            .map(|(p, _)| p.clone())
            .chain(self.rules.keys().filter(|k| k.starts_with("INV-")).cloned())
            .collect();
        for link in self.links.iter().filter(|l| l.relation == "validates") {
            let (file, heading) = link.to.location();
            let _ = missing.remove(file);
            for (id, (doc, anchor)) in &self.rules {
                if doc == file && anchor.as_deref() == heading {
                    let _ = missing.remove(id);
                }
            }
        }
        missing.into_iter().collect()
    }
}

impl Place {
    /// File and optional document anchor.
    pub(crate) fn location(&self) -> (&str, Option<&str>) {
        match self {
            Self::File(f) | Self::ItemAt(f, _) => (f, None),
            Self::Heading(f, s) => (f, Some(s)),
        }
    }
}
