use crate::{Atom, ParseResult};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Node<'a> {
    pub node_type: Cow<'a, str>,
    pub atoms: Cow<'a, [Atom<'a>]>,
    pub nodes: Vec<Self>,
}

impl Node<'static> {
    pub fn open(path: impl AsRef<std::path::Path>) -> ParseResult<Vec<Self>> {
        let bytes = std::fs::read(path)?;
        Self::parse_bytes(&bytes)
    }
    pub fn parse_bytes(bytes: &[u8]) -> ParseResult<Vec<Self>> {
        let s = crate::parser::bytes_to_string(bytes)?;
        Self::parse_str(&s)
    }
    pub fn parse_str(s: &str) -> ParseResult<Vec<Self>> {
        let atoms = Atom::parse_str(s)?;
        Ok(Node::parse_atoms(&atoms)
            .into_iter()
            .map(|node| node.to_owned())
            .collect())
    }
}

impl<'a> Node<'a> {
    pub fn to_owned(&self) -> Node<'static> {
        Node {
            node_type: Cow::Owned(self.node_type.clone().into_owned()),
            atoms: Cow::Owned(self.atoms.iter().map(|a| a.to_owned()).collect()),
            nodes: self.nodes.iter().map(|n| n.to_owned()).collect(),
        }
    }
    pub fn parse_atoms(atoms: &'a [Atom<'a>]) -> Vec<Self> {
        NodeParser { atoms }.parse_nodes(0).unwrap_or_default().0
    }
}

// ---------------------------

struct NodeParser<'a> {
    atoms: &'a [Atom<'a>],
}
impl<'a> NodeParser<'a> {
    fn parse_nodes(&self, mut start: usize) -> Option<(Vec<Node<'a>>, usize)> {
        let mut nodes = vec![];
        while let Some((node, end)) = self.parse_node(start) {
            if !node.node_type.starts_with('$') && node.node_type.contains("END") {
                return Some((nodes, end));
            }
            start = end;
            nodes.push(node);
        }
        if self.atoms[start].value.get() == Some("EOF") {
            Some((nodes, start))
        } else {
            None
        }
    }
    fn parse_node(&self, start: usize) -> Option<(Node<'a>, usize)> {
        const CONTAINER_TYPES: &[&str] = &["SECTION", "BLOCK", "TABLE", "POLYLINE"];
        assert!(is_node_starting_code(self.atoms[start].code));
        let node_type = self.atoms[start].value.get().unwrap_or_default();
        if CONTAINER_TYPES.contains(&node_type) {
            self.parse_container(node_type, start + 1)
        } else {
            self.parse_element(node_type, start + 1)
        }
    }
    fn parse_container(&self, node_type: &'a str, start: usize) -> Option<(Node<'a>, usize)> {
        let (mut node, start) = self.parse_element(node_type, start)?;
        let (nodes, end) = self.parse_nodes(start)?;
        node.nodes = nodes;
        Some((node, end))
    }
    fn parse_element(&self, node_type: &'a str, start: usize) -> Option<(Node<'a>, usize)> {
        (start..self.atoms.len())
            .find(|i| is_node_starting_code(self.atoms[*i].code))
            .map(|end| {
                let entity = Node {
                    node_type: Cow::Borrowed(node_type),
                    atoms: Cow::Borrowed(&self.atoms[start..end]),
                    nodes: vec![],
                };
                (entity, end)
            })
    }
}

fn is_node_starting_code(code: i16) -> bool {
    code == 0 || code == 9
}
