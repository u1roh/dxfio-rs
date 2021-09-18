mod entity;
mod table;
use crate::{Node, ParseResult};
pub use entity::*;
pub use table::*;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Drawing {
    pub headers: Vec<Node<'static>>, // 暫定措置
    pub tables: Vec<TableNode>,
    pub blocks: Vec<BlockNode>,
    pub entities: Vec<EntityNode>,
}
impl Drawing {
    pub fn open(path: impl AsRef<std::path::Path>) -> ParseResult<Self> {
        let bytes = std::fs::read(path)?;
        Self::parse_bytes(&bytes)
    }
    pub fn parse_bytes(bytes: &[u8]) -> ParseResult<Self> {
        let s = crate::parser::bytes_to_string(bytes)?;
        Self::parse_str(&s)
    }
    pub fn parse_str(s: &str) -> ParseResult<Self> {
        let atoms = crate::Atom::parse_str(s)?;
        Ok(Self::parse_atoms(&atoms))
    }
    pub fn parse_atoms(atoms: &[crate::Atom]) -> Self {
        Self::parse_nodes(&crate::Node::parse(atoms))
    }
    pub fn parse_nodes<'a>(nodes: &'a [crate::Node<'a>]) -> Self {
        crate::parser::ParDrawing::parse(nodes).into()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BlockNode {
    pub handle: u32,             // 5
    pub layer: String,           // 8
    pub block_name: String,      // 2, 3
    pub block_flags: BlockFlags, // 70
    pub base_point: [f64; 3],    // 10, 20, 30
    pub xref_path_name: String,  // 1
    pub description: String,     // 4
    pub entities: Vec<EntityNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct BlockFlags {
    pub is_anonymous: bool,
    pub has_non_constant_attribute_definitions: bool,
    pub is_xref: bool, // xref = external reference
    pub is_xref_overlay: bool,
    pub is_externally_dependent: bool,
    pub is_resolved_xref_or_dependent_of_xref: bool,
    pub is_referenced_xref: bool,
}
