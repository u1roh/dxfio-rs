pub mod parser;

mod drawing;
pub use drawing::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxfAtom {
    pub code: i16,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxfNode {
    pub node_type: String,
    pub atoms: Vec<DxfAtom>,
    pub nodes: Vec<Self>,
}
