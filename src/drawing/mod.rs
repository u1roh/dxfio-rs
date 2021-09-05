use crate::{DxfAtom, DxfNode};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Drawing {
    pub headers: Vec<DxfNode>, // 暫定措置
    pub blocks: Vec<BlockNode>,
    pub entities: Vec<EntityNode>,
}
impl Drawing {
    pub fn parse_str(s: &str) -> crate::DxfParseResult<Self> {
        let atoms = crate::parser::ParAtom::parse(s)?;
        Ok(Self::parse_atoms(&atoms))
    }
    pub fn parse_atoms(atoms: &[crate::parser::ParAtom]) -> Self {
        Self::parse_nodes(&crate::parser::ParNode::parse(atoms))
    }
    pub fn parse_nodes(nodes: &[crate::parser::ParNode]) -> Self {
        crate::parser::ParDrawing::parse(nodes).into()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntityHeader {
    pub handle: u32,                     // 5    String
    pub space: Space,                    // 67   i16     ModelSpace
    pub layer: String,                   // 8    String
    pub line_type: LineType,             // 6    String  ByLayer
    pub color_number: ColorNumber,       // 62   i16     ByLayer
    pub lineweight: Option<i16>,         // 370  i16
    pub line_type_scale: Option<f64>,    // 48   f64
    pub is_visible: bool,                // 60   i16     true
    pub color_rgb: Option<Rgb>,          // 420  i32
    pub color_name: Option<String>,      // 430  String
    pub transparency: Option<i32>,       // 440  i32
    pub shadow_mode: Option<ShadowMode>, // 284  i16
    pub extras: Vec<DxfAtom>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LineType {
    ByLayer,
    ByBlock,
    Continuous,
    Dashed,
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ShadowMode {
    CastsAndReceivesShadows = 0,
    CastsShadows = 1,
    ReceivesShadows = 2,
    IgnoresShadows = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ColorNumber {
    ByLayer,
    ByEntity,
    ByBlock,
    TurnedOff,
    Number(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Space {
    ModelSpace,
    PaperSpace,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntityNode {
    pub header: EntityHeader,
    pub entity: Entity,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Entity {
    Line(Line),
    Unknown(crate::DxfNode),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Line {
    pub p1: [f64; 3],
    pub p2: [f64; 3],
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
