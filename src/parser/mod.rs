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

pub trait ParseFromNode {
    fn parse_from_node(nodes: &ParNode) -> Self;
}

#[derive(Debug, Clone)]
pub struct SourceAndTarget<'a, T> {
    pub source: &'a ParNode<'a>,
    pub target: T,
}
impl<'a, T: ParseFromNode> SourceAndTarget<'a, T> {
    fn parse_from_node(source: &'a ParNode<'a>) -> Self {
        Self {
            source,
            target: T::parse_from_node(source),
        }
    }
}
