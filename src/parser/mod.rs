mod block;
mod data;
mod entity;
mod table;
mod text_format;

use crate::{Atom, AtomList, Document, Node};
use std::borrow::Cow;

impl Document {
    pub fn parse_nodes(nodes: &[Node]) -> Self {
        let mut drawing = Self {
            headers: Vec::new(),
            tables: Vec::new(),
            blocks: Vec::new(),
            entities: Vec::new(),
        };
        for section in nodes {
            match section.atoms.find(2) {
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
    fn add_nodes(&mut self, nodes: &[Node]) -> bool {
        if nodes.is_empty() {
            true
        } else {
            log::error!("sub nodes are ignored @ {}", std::any::type_name::<Self>());
            log::error!("nodes.len() = {}", nodes.len());
            log::error!(
                "nodes = {:?}",
                nodes.iter().map(|n| &n.node_type).collect::<Vec<_>>()
            );
            false
        }
    }
}

impl<T: SetAtom> FromNode for T {
    fn from_node(source: &Node) -> Self {
        let mut dst = Self::default();
        for atom in source.atoms.iter() {
            dst.set_atom(atom);
        }
        dst.add_nodes(&source.nodes);
        dst
    }
}

fn parse_to<T: std::str::FromStr>(s: &str, dst: &mut T) -> bool {
    if let Ok(x) = s.parse() {
        *dst = x;
        true
    } else {
        log::error!(
            "parse_to({:?}, dst: &mut {}) failed",
            s,
            std::any::type_name::<T>(),
        );
        false
    }
}

fn parse_to_option<T: std::str::FromStr>(s: &str, dst: &mut Option<T>) -> bool {
    if let Ok(x) = s.parse() {
        *dst = Some(x);
        true
    } else {
        log::error!(
            "parse_to_option({:?}, dst: &mut {}) failed",
            s,
            std::any::type_name::<T>(),
        );
        *dst = None;
        false
    }
}
