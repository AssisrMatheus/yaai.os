//! spec: docs/repo/doc-links.md#edges
//! Adds the resolved links to a graphify graph (`graphify-out/graph.json`) as `EXTRACTED` edges between the
//! nodes graphify already made for the files, symbols, pages and headings.
//!
//! Edges this tool adds carry `"_origin": "doc-links"`; each run removes them first, so running it again after
//! `graphify update` gives the same graph.

use std::collections::{BTreeSet, HashMap};

use serde_json::{Map, Value, json};

use crate::markdown::github_slug;

/// Value of `_origin` on every edge this tool adds.
pub(crate) const ORIGIN: &str = "doc-links";

/// A place in the repository an edge can start or end at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Place {
    /// A whole file: a code file, or a document's page node.
    File(String),
    /// The code item that starts at a 1-based line of a file; falls back to the file.
    ItemAt(String, usize),
    /// A heading of a document, by anchor; falls back to the page.
    Heading(String, String),
}

/// One edge to add.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Link {
    /// Where it starts.
    pub(crate) from: Place,
    /// Where it ends.
    pub(crate) to: Place,
    /// `cites` (code to doc), `validates` (test to doc), `covers` (spec to code).
    pub(crate) relation: &'static str,
    /// File that states the link.
    pub(crate) source_file: String,
    /// 1-based line that states it.
    pub(crate) line: usize,
    /// The link as written.
    pub(crate) context: String,
}

/// One graph node, as far as matching needs it.
#[derive(Clone, Debug)]
struct NodeInfo {
    /// Node id.
    id: String,
    /// Label (file name, symbol name, heading text).
    label: String,
    /// 1-based line from `source_location` (`L12`).
    line: Option<usize>,
    /// `code`, `document`, `concept`, …
    file_type: String,
    /// `page` or `heading` for documents.
    node_kind: String,
}

/// The graph's nodes grouped by source file.
struct NodeIndex {
    /// Nodes of each repository-relative source file.
    by_file: HashMap<String, Vec<NodeInfo>>,
    /// Every node id, so a node this tool adds never reuses one.
    ids: BTreeSet<String>,
}

impl Place {
    /// The repository-relative file this place is in.
    fn file(&self) -> &str {
        match self {
            Self::File(file) | Self::ItemAt(file, _) | Self::Heading(file, _) => file,
        }
    }
}

/// A string field of a JSON object, or the empty string.
fn text_field(object: &Map<String, Value>, key: &str) -> String {
    object
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}

impl NodeIndex {
    /// Indexes the `nodes` array of a graph.
    fn new(graph: &Value) -> Self {
        let mut by_file: HashMap<String, Vec<NodeInfo>> = HashMap::new();
        let mut ids = BTreeSet::new();
        let nodes = graph.get("nodes").and_then(Value::as_array);
        for node in nodes.into_iter().flatten().filter_map(Value::as_object) {
            let _ = ids.insert(text_field(node, "id"));
            let line = node
                .get("source_location")
                .and_then(Value::as_str)
                .and_then(|loc| loc.strip_prefix('L'))
                .and_then(|n| n.parse().ok());
            by_file
                .entry(text_field(node, "source_file"))
                .or_default()
                .push(NodeInfo {
                    id: text_field(node, "id"),
                    label: text_field(node, "label"),
                    line,
                    file_type: text_field(node, "file_type"),
                    node_kind: text_field(node, "node_kind"),
                });
        }
        Self { by_file, ids }
    }

    /// Adds a file node for `file`, which graphify made none for, and returns it as graph JSON.
    ///
    /// The id follows graphify's style (path, lower case, `_` for other characters) and gets a suffix when taken.
    fn add_file(&mut self, file: &str) -> Value {
        let base: String = file
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() {
                    c.to_ascii_lowercase()
                } else {
                    '_'
                }
            })
            .collect();
        let base = base.trim_matches('_');
        let mut id = base.to_owned();
        let mut attempt: usize = 1;
        while !self.ids.insert(id.clone()) {
            attempt = attempt.saturating_add(1);
            id = format!("{base}_file{attempt}");
        }
        let label = file.rsplit_once('/').map_or(file, |(_, name)| name);
        self.by_file
            .entry(file.to_owned())
            .or_default()
            .push(NodeInfo {
                id: id.clone(),
                label: label.to_owned(),
                line: Some(1),
                file_type: "code".to_owned(),
                node_kind: String::new(),
            });
        json!({
            "_origin": ORIGIN,
            "file_type": "code",
            "id": id,
            "label": label,
            "metadata": { "kind": "file" },
            "norm_label": label.to_lowercase(),
            "source_file": file,
            "source_location": "L1",
        })
    }

    /// The node that stands for a whole file: a document's page, or the code node graphify made for the file.
    fn file_node(&self, file: &str) -> Option<&NodeInfo> {
        let nodes = self.by_file.get(file)?;
        let name = file.rsplit_once('/').map_or(file, |(_, name)| name);
        let top = |n: &&NodeInfo| n.file_type == "code" && n.line == Some(1);
        nodes
            .iter()
            .find(|n| n.node_kind == "page")
            .or_else(|| nodes.iter().find(|n| top(n) && n.label.ends_with(name)))
            .or_else(|| {
                // WHY: graphify labels a crate manifest's only node with the package name, not the file name.
                let mut tops = nodes.iter().filter(top);
                tops.next().filter(|_| tops.next().is_none())
            })
    }

    /// The node id for a place, falling back from an item or heading to its file.
    fn id_of(&self, place: &Place) -> Option<&str> {
        let file_id = |file: &str| self.file_node(file).map(|n| n.id.as_str());
        match place {
            Place::File(file) => file_id(file),
            Place::ItemAt(file, line) => {
                let file_node = file_id(file);
                self.by_file
                    .get(file)
                    .and_then(|nodes| {
                        nodes.iter().find(|n| {
                            n.file_type == "code"
                                && n.line == Some(*line)
                                && Some(n.id.as_str()) != file_node
                        })
                    })
                    .map(|n| n.id.as_str())
                    .or(file_node)
            }
            Place::Heading(doc, anchor) => self
                .by_file
                .get(doc)
                .and_then(|nodes| {
                    nodes
                        .iter()
                        .find(|n| n.node_kind == "heading" && github_slug(&n.label) == *anchor)
                })
                .map(|n| n.id.as_str())
                .or_else(|| file_id(doc)),
        }
    }
}

/// What [`inject`] did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct InjectStats {
    /// Edges added.
    pub(crate) added: usize,
    /// Edges from an earlier run removed first.
    pub(crate) removed: usize,
    /// File nodes added for linked files graphify made no node for.
    pub(crate) nodes: usize,
    /// Links that still have an end without a node.
    pub(crate) unplaced: usize,
}

/// Replaces this tool's edges and file nodes in `graph` with those of `links`; duplicates and self-loops are
/// dropped.
///
/// A link that ends at a file graphify made no node at all for gets a file node, so no checked link is lost. A graph
/// without a `links` (or `edges`) array gets none added.
pub(crate) fn inject(graph: &mut Value, links: &[Link]) -> InjectStats {
    if let Some(nodes) = graph.get_mut("nodes").and_then(Value::as_array_mut) {
        nodes.retain(|node| node.get("_origin").and_then(Value::as_str) != Some(ORIGIN));
    }
    let mut index = NodeIndex::new(graph);
    let mut stats = InjectStats::default();
    let mut made = Vec::new();
    for place in links.iter().flat_map(|link| [&link.from, &link.to]) {
        // WHY: only a file graphify has nothing for gets a node. A file with nodes but no file-level one is a
        // graph this tool does not understand, and stays counted as unplaced.
        if !index.by_file.contains_key(place.file()) {
            made.push(index.add_file(place.file()));
        }
    }
    stats.nodes = made.len();
    if let Some(nodes) = graph.get_mut("nodes").and_then(Value::as_array_mut) {
        nodes.extend(made);
    }
    let mut seen: BTreeSet<(String, String, &'static str)> = BTreeSet::new();
    let mut new_edges = Vec::new();
    for link in links {
        let (Some(from), Some(to)) = (index.id_of(&link.from), index.id_of(&link.to)) else {
            stats.unplaced = stats.unplaced.saturating_add(1);
            continue;
        };
        if from == to || !seen.insert((from.to_owned(), to.to_owned(), link.relation)) {
            continue;
        }
        new_edges.push(json!({
            "source": from,
            "target": to,
            "relation": link.relation,
            "_origin": ORIGIN,
            "confidence": "EXTRACTED",
            "confidence_score": 1.0,
            "source_file": link.source_file,
            "source_location": format!("L{}", link.line),
            "context": link.context,
            "weight": 1.0,
        }));
    }
    let key = if graph.get("links").is_some() {
        "links"
    } else {
        "edges"
    };
    if let Some(edges) = graph.get_mut(key).and_then(Value::as_array_mut) {
        let before = edges.len();
        edges.retain(|edge| edge.get("_origin").and_then(Value::as_str) != Some(ORIGIN));
        stats.removed = before.saturating_sub(edges.len());
        stats.added = new_edges.len();
        edges.extend(new_edges);
    }
    stats
}
