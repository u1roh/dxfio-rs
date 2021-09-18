mod block;
mod entity;
mod table;

use crate::{Atom, AtomList, Drawing, Node};
use std::borrow::Cow;

impl Drawing {
    pub fn parse_nodes(nodes: &[Node]) -> Self {
        let mut drawing = Self {
            headers: Vec::new(),
            tables: Vec::new(),
            blocks: Vec::new(),
            entities: Vec::new(),
        };
        for section in nodes {
            match section.atoms.get_value(2) {
                Some("HEADER") => {
                    drawing.headers = section.nodes.iter().map(Node::to_owned).collect();
                }
                Some("CLASSES") => {}
                Some("TABLES") => {
                    drawing.tables = section.nodes.iter().map(FromNode::from_node).collect();
                }
                Some("BLOCKS") => {
                    drawing.blocks = section.nodes.iter().map(FromNode::from_node).collect();
                }
                Some("ENTITIES") => {
                    drawing.entities = section.nodes.iter().map(FromNode::from_node).collect();
                }
                Some("OBJECTS") => {}
                Some(unknown) => {
                    println!("unknown section: {}", unknown);
                }
                None => {
                    println!("section type not found");
                }
            }
        }
        drawing
    }
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
#[error("{:?}", self)]
pub struct EncodingError;

pub fn bytes_to_string(bytes: &[u8]) -> Result<Cow<str>, EncodingError> {
    match std::str::from_utf8(bytes) {
        Ok(s) => Ok(s.into()),
        Err(_) => {
            use encoding_rs::*;
            [SHIFT_JIS, EUC_JP]
                .iter()
                .find_map(|encoding| {
                    let (s, _, malformed) = encoding.decode(bytes);
                    if malformed {
                        None
                    } else {
                        Some(s)
                    }
                })
                .ok_or(EncodingError)
        }
    }
}

pub trait FromNode {
    fn from_node(source: &Node) -> Self;
}

pub trait SetAtom: Default {
    fn set_atom(&mut self, atom: &Atom) -> bool;
}

impl<T: SetAtom> FromNode for T {
    fn from_node(source: &Node) -> Self {
        let mut dst = Self::default();
        for atom in source.atoms.iter() {
            dst.set_atom(atom);
        }
        dst
    }
}
