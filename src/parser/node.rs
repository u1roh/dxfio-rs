use super::ParAtom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParNode<'a> {
    pub node_type: &'a str,
    pub atoms: &'a [ParAtom<'a>],
    pub nodes: Vec<Self>,
}
impl<'a> ParNode<'a> {
    pub fn parse(atoms: &'a [ParAtom<'a>]) -> Vec<Self> {
        NodeParser { atoms }.parse_nodes(0).unwrap_or_default().0
    }
}
impl<'a> crate::DxfAtomList for &[ParAtom<'a>] {
    fn find(&self, code: i16) -> Option<&str> {
        self.iter()
            .find(|item| item.code == code)
            .map(|item| item.value)
    }
}
impl<'a> From<ParNode<'a>> for crate::DxfNode {
    fn from(node: ParNode<'a>) -> Self {
        Self {
            node_type: node.node_type.to_owned(),
            atoms: node.atoms.iter().copied().map(Into::into).collect(),
            nodes: node.nodes.into_iter().map(Into::into).collect(),
        }
    }
}
impl<'a> From<&ParNode<'a>> for crate::DxfNode {
    fn from(node: &ParNode<'a>) -> Self {
        Self {
            node_type: node.node_type.to_owned(),
            atoms: node.atoms.iter().copied().map(Into::into).collect(),
            nodes: node.nodes.iter().map(Into::into).collect(),
        }
    }
}

struct NodeParser<'a> {
    atoms: &'a [ParAtom<'a>],
}
impl<'a> NodeParser<'a> {
    fn parse_nodes(&self, mut start: usize) -> Option<(Vec<ParNode<'a>>, usize)> {
        let mut nodes = vec![];
        while let Some((node, end)) = self.parse_node(start) {
            if !node.node_type.starts_with('$') && node.node_type.contains("END") {
                return Some((nodes, end));
            }
            start = end;
            nodes.push(node);
        }
        if self.atoms[start].value == "EOF" {
            Some((nodes, start))
        } else {
            None
        }
    }
    fn parse_node(&self, start: usize) -> Option<(ParNode<'a>, usize)> {
        const CONTAINER_TYPES: &[&str] = &["SECTION", "BLOCK", "TABLE", "POLYLINE"];
        assert!(is_node_starting_code(self.atoms[start].code));
        let node_type = self.atoms[start].value;
        if CONTAINER_TYPES.contains(&node_type) {
            self.parse_container(node_type, start + 1)
        } else {
            self.parse_element(node_type, start + 1)
        }
    }
    fn parse_container(&self, node_type: &'a str, start: usize) -> Option<(ParNode<'a>, usize)> {
        let (mut node, start) = self.parse_element(node_type, start)?;
        let (nodes, end) = self.parse_nodes(start)?;
        node.nodes = nodes;
        Some((node, end))
    }
    fn parse_element(&self, node_type: &'a str, start: usize) -> Option<(ParNode<'a>, usize)> {
        (start..self.atoms.len())
            .find(|i| is_node_starting_code(self.atoms[*i].code))
            .map(|end| {
                let entity = ParNode {
                    node_type,
                    atoms: &self.atoms[start..end],
                    nodes: vec![],
                };
                (entity, end)
            })
    }
}

fn is_node_starting_code(code: i16) -> bool {
    code == 0 || code == 9
}

// ----------------------------------------------

use crate::{Atom, Node};
use std::borrow::Cow;

impl<'a> Node<'a> {
    pub fn parse(atoms: &'a [Atom<'a>]) -> Vec<Self> {
        NodeParser2 { atoms }.parse_nodes(0).unwrap_or_default().0
    }
}

struct NodeParser2<'a> {
    atoms: &'a [Atom<'a>],
}
impl<'a> NodeParser2<'a> {
    fn parse_nodes(&self, mut start: usize) -> Option<(Vec<Node<'a>>, usize)> {
        let mut nodes = vec![];
        while let Some((node, end)) = self.parse_node(start) {
            if !node.node_type.starts_with('$') && node.node_type.contains("END") {
                return Some((nodes, end));
            }
            start = end;
            nodes.push(node);
        }
        if self.atoms[start].value.as_str() == Some("EOF") {
            Some((nodes, start))
        } else {
            None
        }
    }
    fn parse_node(&self, start: usize) -> Option<(Node<'a>, usize)> {
        const CONTAINER_TYPES: &[&str] = &["SECTION", "BLOCK", "TABLE", "POLYLINE"];
        assert!(is_node_starting_code(self.atoms[start].code));
        let node_type = self.atoms[start].value.as_str().unwrap_or_default();
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
