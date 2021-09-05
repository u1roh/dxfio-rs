pub mod parser;

mod drawing;
pub use drawing::*;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DxfAtom {
    pub code: i16,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DxfNode {
    pub node_type: String,
    pub atoms: Vec<DxfAtom>,
    pub nodes: Vec<Self>,
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
