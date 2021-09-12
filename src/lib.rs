pub mod parser;

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

#[derive(Debug, thiserror::Error)]
pub enum DxfParseError {
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

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
