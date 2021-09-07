use crate::DxfAtom;

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
    Insert(Insert),
    Unknown(crate::DxfNode),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Line {
    pub p1: [f64; 3],
    pub p2: [f64; 3],
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Insert {
    pub block_name: String,
    pub insertion_point: [f64; 3],
    pub scale_factor: [f64; 3],
    pub rotation_degree: f64,
    pub column_count: usize,
    pub row_count: usize,
    pub column_spacing: f64,
    pub row_spacing: f64,
    pub extrusion_direction: [f64; 3],
}
impl Insert {
    pub fn new(block_name: String) -> Self {
        Self {
            block_name,
            insertion_point: [0.0, 0.0, 0.0],
            scale_factor: [1.0, 1.0, 1.0],
            rotation_degree: 0.0,
            column_count: 1,
            row_count: 1,
            column_spacing: 0.0,
            row_spacing: 0.0,
            extrusion_direction: [0.0, 0.0, 1.0],
        }
    }
}
