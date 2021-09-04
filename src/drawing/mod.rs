#[derive(Debug)]
pub struct Drawing {
    pub entities: Vec<EntityNode>,
}
impl Drawing {
    pub fn parse_str(s: &str) -> Result<Self, std::num::ParseIntError> {
        crate::parser::ParAtom::parse(s).map(|atoms| Self::parse_atoms(&atoms))
    }
    pub fn parse_atoms(atoms: &[crate::parser::ParAtom]) -> Self {
        Self::parse_nodes(&crate::parser::ParNode::parse(atoms))
    }
    pub fn parse_nodes(nodes: &[crate::parser::ParNode]) -> Self {
        crate::parser::ParDrawing::parse(nodes).into()
    }
}

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineType {
    ByLayer,
    ByBlock,
    Continuous,
    Dashed,
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadowMode {
    CastsAndReceivesShadows = 0,
    CastsShadows = 1,
    ReceivesShadows = 2,
    IgnoresShadows = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorNumber {
    ByLayer,
    ByEntity,
    ByBlock,
    TurnedOff,
    Number(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    ModelSpace,
    PaperSpace,
}

#[derive(Debug, Clone)]
pub struct EntityNode {
    pub header: EntityHeader,
    pub entity: Entity,
}

#[derive(Debug, Clone)]
pub enum Entity {
    Line(Line),
    Unknown(crate::DxfNode),
}

#[derive(Debug, Clone)]
pub struct Line {
    pub p1: [f64; 3],
    pub p2: [f64; 3],
}
