pub mod parser;
use std::borrow::Cow;

mod drawing;
pub use drawing::*;

mod value;
pub use value::Value;

mod atom;
pub use atom::Atom;

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
        let s = parser::bytes_to_string(bytes)?;
        Self::parse_str(&s)
    }
    pub fn parse_str(s: &str) -> ParseResult<Vec<Self>> {
        let atoms = Atom::parse_str(s)?;
        Ok(Node::parse(&atoms)
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
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error(transparent)]
    EncodingError(#[from] parser::EncodingError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type ParseResult<T> = Result<T, ParseError>;

pub trait DxfAtomList {
    fn find(&self, code: i16) -> Option<&Value>;
    fn get<'a, T: value::FromValue<'a>>(&'a self, code: i16) -> Option<T> {
        self.find(code)?.get()
    }
    fn get_or_default<'a, T: value::FromValue<'a> + Default>(&'a self, code: i16) -> T {
        self.get(code).unwrap_or_default()
    }
    fn get_point(&self, i: usize) -> [f64; 3] {
        [
            self.get_or_default(10 + i as i16),
            self.get_or_default(20 + i as i16),
            self.get_or_default(30 + i as i16),
        ]
    }
}

impl<'a> DxfAtomList for &[Atom<'a>] {
    fn find(&self, code: i16) -> Option<&Value> {
        self.iter()
            .find(|item| item.code == code)
            .map(|item| &item.value)
    }
}
