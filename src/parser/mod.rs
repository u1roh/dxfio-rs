mod atom;
mod block;
mod drawing;
mod entity;
mod node;
mod table;

pub use atom::ParAtom;
pub use drawing::*;
pub use node::ParNode;

use std::borrow::Cow;

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
    fn from_node(source: &ParNode) -> Self;
}

pub trait SetAtom: Default {
    fn set_atom(&mut self, atom: &ParAtom) -> bool;
}

impl<T: SetAtom> FromNode for T {
    fn from_node(source: &ParNode) -> Self {
        let mut dst = Self::default();
        for atom in source.atoms {
            dst.set_atom(atom);
        }
        dst
    }
}

#[derive(Debug, Clone)]
pub struct SourceAndTarget<'a, T> {
    pub source: &'a ParNode<'a>,
    pub target: T,
}
impl<'a, T: FromNode> SourceAndTarget<'a, T> {
    fn from_node(source: &'a ParNode<'a>) -> Self {
        Self {
            source,
            target: T::from_node(source),
        }
    }
}

// ----------------------------------
use crate::{Atom, Node};

pub trait FromNode2 {
    fn from_node(source: &Node) -> Self;
}

pub trait SetAtom2: Default {
    fn set_atom(&mut self, atom: &Atom) -> bool;
}

impl<T: SetAtom2> FromNode2 for T {
    fn from_node(source: &Node) -> Self {
        let mut dst = Self::default();
        for atom in source.atoms.iter() {
            dst.set_atom(atom);
        }
        dst
    }
}

#[derive(Debug, Clone)]
pub struct SourceAndTarget2<'a, T> {
    pub source: &'a Node<'a>,
    pub target: T,
}
impl<'a, T: FromNode2> SourceAndTarget2<'a, T> {
    fn from_node(source: &'a Node<'a>) -> Self {
        Self {
            source,
            target: T::from_node(source),
        }
    }
}
