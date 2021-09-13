pub mod parser;
use std::borrow::Cow;

mod drawing;
pub use drawing::*;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DxfAtom {
    pub code: i16,
    pub value: String,
}
impl DxfAtom {
    pub fn as_ref(&self) -> parser::ParAtom {
        parser::ParAtom {
            code: self.code,
            value: &self.value,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DxfNode {
    pub node_type: String,
    pub atoms: Vec<DxfAtom>,
    pub nodes: Vec<Self>,
}
impl DxfNode {
    pub fn open(path: impl AsRef<std::path::Path>) -> DxfParseResult<Vec<Self>> {
        let bytes = std::fs::read(path)?;
        Self::parse_bytes(&bytes)
    }
    pub fn parse_bytes(bytes: &[u8]) -> DxfParseResult<Vec<Self>> {
        let s = parser::bytes_to_string(bytes)?;
        Self::parse_str(&s)
    }
    pub fn parse_str(s: &str) -> DxfParseResult<Vec<Self>> {
        let atoms = crate::parser::ParAtom::parse(s)?;
        Ok(parser::ParNode::parse(&atoms)
            .into_iter()
            .map(Into::into)
            .collect())
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Value<'a> {
    String(Cow<'a, str>),
    F64(f64),
    I64(i64),
    I32(i32),
    I16(i16),
    Bool(bool),
    Handle(u32),
    Bytes(Vec<u8>),
}
impl<'a> Value<'a> {
    pub fn as_str(&self) -> Option<&str> {
        if let Self::String(s) = self {
            Some(s)
        } else {
            None
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        if let Self::F64(x) = self {
            Some(*x)
        } else {
            None
        }
    }
    pub fn into_owned(self) -> Value<'static> {
        match self {
            Self::String(s) => Value::String(Cow::Owned(s.into_owned())),
            Self::F64(x) => Value::F64(x),
            Self::I64(x) => Value::I64(x),
            Self::I32(x) => Value::I32(x),
            Self::I16(x) => Value::I16(x),
            Self::Bool(x) => Value::Bool(x),
            Self::Handle(x) => Value::Handle(x),
            Self::Bytes(x) => Value::Bytes(x),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Atom<'a> {
    pub code: i16,
    pub value: Value<'a>,
}
impl<'a> Atom<'a> {
    fn into_owned(self) -> Atom<'static> {
        Atom {
            code: self.code,
            value: self.value.into_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Node<'a> {
    pub node_type: Cow<'a, str>,
    pub atoms: Cow<'a, [Atom<'a>]>,
    pub nodes: Vec<Self>,
}
impl Node<'static> {
    pub fn open(path: impl AsRef<std::path::Path>) -> DxfParseResult<Vec<Self>> {
        let bytes = std::fs::read(path)?;
        Self::parse_bytes(&bytes)
    }
    pub fn parse_bytes(bytes: &[u8]) -> DxfParseResult<Vec<Self>> {
        let s = parser::bytes_to_string(bytes)?;
        Self::parse_str(&s)
    }
    pub fn parse_str(s: &str) -> DxfParseResult<Vec<Self>> {
        let atoms = Atom::parse(s)?;
        Ok(Node::parse(&atoms)
            .into_iter()
            .map(|node| node.into_owned())
            .collect())
    }
}
impl<'a> Node<'a> {
    pub fn into_owned(self) -> Node<'static> {
        Node {
            node_type: Cow::Owned(self.node_type.into_owned()),
            atoms: Cow::Owned(
                self.atoms
                    .into_owned()
                    .into_iter()
                    .map(|a| a.into_owned())
                    .collect(),
            ),
            nodes: self.nodes.into_iter().map(|n| n.into_owned()).collect(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DxfParseError {
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error(transparent)]
    EncodingError(#[from] parser::EncodingError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type DxfParseResult<T> = Result<T, DxfParseError>;

pub trait DxfAtomList {
    fn find(&self, code: i16) -> Option<&str>;
    fn get<T: std::str::FromStr>(&self, code: i16) -> Option<T> {
        self.find(code)?.parse().ok()
    }
    fn get_or_default<T: std::str::FromStr + Default>(&self, code: i16) -> T {
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

impl DxfAtomList for &[DxfAtom] {
    fn find(&self, code: i16) -> Option<&str> {
        self.iter()
            .find(|item| item.code == code)
            .map(|item| &item.value as _)
    }
}
