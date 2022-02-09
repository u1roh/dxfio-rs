use crate::{Atom, ParseResult};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Node<'a> {
    pub node_type: Cow<'a, str>,
    pub atoms: Cow<'a, [Atom<'a>]>,
    pub nodes: Vec<Self>,
    pub end: Option<Box<Self>>,
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
            end: self.end.as_ref().map(|n| Box::new(Self::to_owned(&*n))),
        }
    }
    pub fn parse_atoms(atoms: &'a [Atom<'a>]) -> Vec<Self> {
        NodeParser { atoms }.parse_nodes(0).unwrap_or_default().0
    }
    pub fn iter_atoms(&self) -> Box<dyn Iterator<Item = Atom<'a>> + '_> {
        Box::new(
            std::iter::once(Atom {
                code: 0,
                value: self.node_type.clone(),
            })
            .chain(self.atoms.iter().cloned())
            .chain(self.nodes.iter().flat_map(Self::iter_atoms))
            .chain(self.end.iter().flat_map(|n| n.iter_atoms())),
        )
    }
}

impl<'a> std::fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for atom in self.iter_atoms() {
            atom.fmt(f)?;
        }
        Ok(())
    }
}

// ---------------------------

struct NodeParser<'a> {
    atoms: &'a [Atom<'a>],
}
impl<'a> NodeParser<'a> {
    fn parse_nodes(&self, mut start: usize) -> Option<(Vec<Node<'a>>, Node<'a>, usize)> {
        let mut nodes = vec![];
        while let Some((node, end)) = self.parse_node(start) {
            if !node.node_type.starts_with('$') && node.node_type.contains("END") {
                return Some((nodes, node, end));
            }
            start = end;
            nodes.push(node);
        }
        if &self.atoms[start].value as &str == "EOF" {
            let eof = Node {
                node_type: Cow::Borrowed("EOF"),
                ..Default::default()
            };
            Some((nodes, eof, start))
        } else {
            None
        }
    }
    fn parse_node(&self, start: usize) -> Option<(Node<'a>, usize)> {
        fn is_container_type(node: &Node) -> bool {
            const CONTAINER_TYPES: &[&str] = &["SECTION", "BLOCK", "TABLE", "POLYLINE"];
            CONTAINER_TYPES.contains(&&*node.node_type)
                || (node.node_type == "INSERT"
                    && node
                        .atoms
                        .iter()
                        .any(|a| a.code == 66 && a.value.parse() == Ok(1i16)))
        }
        assert!(is_node_starting_code(self.atoms[start].code));
        let node_type = &self.atoms[start].value;
        let (mut node, mut pos) = self.parse_element(node_type, start + 1)?;
        if is_container_type(&node) {
            let (nodes, end_node, end_pos) = self.parse_nodes(pos)?;
            node.nodes = nodes;
            node.end = Some(Box::new(end_node));
            pos = end_pos;
        }
        Some((node, pos))
    }
    fn parse_element(&self, node_type: &'a str, start: usize) -> Option<(Node<'a>, usize)> {
        (start..self.atoms.len())
            .find(|i| is_node_starting_code(self.atoms[*i].code))
            .map(|end| {
                let entity = Node {
                    node_type: Cow::Borrowed(node_type),
                    atoms: Cow::Borrowed(&self.atoms[start..end]),
                    nodes: vec![],
                    end: None,
                };
                (entity, end)
            })
    }
}

fn is_node_starting_code(code: i16) -> bool {
    code == 0 || code == 9
}
