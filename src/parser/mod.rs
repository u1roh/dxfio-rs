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

#[derive(Debug, Clone)]
pub struct SourceAndTarget<'a, T> {
    pub source: &'a ParNode<'a>,
    pub target: T,
}

use crate::{BlockNode, EntityNode, TableNode};
pub type ParEntityNode<'a> = SourceAndTarget<'a, EntityNode>;
pub type ParTableNode<'a> = SourceAndTarget<'a, TableNode>;
pub type ParBlockNode<'a> = SourceAndTarget<'a, BlockNode>;
