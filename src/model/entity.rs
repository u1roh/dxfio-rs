use crate::Atom;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntityNode {
    pub header: EntityHeader,
    pub entity: Entity,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct EntityHeader {
    pub handle: u32,                     // 5    String
    pub space: Space,                    // 67   i16     ModelSpace
    pub layer: String,                   // 8    String
    pub line_type: LineTypeRef,          // 6    String  ByLayer
    pub color_number: ColorNumber,       // 62   i16     ByLayer
    pub line_weight: Option<i16>,        // 370  i16
    pub line_type_scale: Option<f64>,    // 48   f64
    pub is_visible: bool,                // 60   i16     true
    pub color_rgb: Option<Rgb>,          // 420  i32
    pub color_name: Option<String>,      // 430  String
    pub transparency: Option<i32>,       // 440  i32
    pub shadow_mode: Option<ShadowMode>, // 284  i16
    pub extras: Vec<Atom<'static>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LineTypeRef {
    ByLayer,
    ByBlock,
    ByName(String),
}
impl Default for LineTypeRef {
    fn default() -> Self {
        Self::ByLayer
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ShadowMode {
    CastsAndReceivesShadows,
    CastsShadows,
    ReceivesShadows,
    IgnoresShadows,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ColorNumber {
    ByLayer,
    ByEntity,
    ByBlock,
    TurnedOff,
    Number(u8),
}
impl Default for ColorNumber {
    fn default() -> Self {
        Self::ByLayer
    }
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
impl Default for Space {
    fn default() -> Self {
        Self::ModelSpace
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Entity {
    Insert(Insert),
    Line(Line),
    Text(Text),
    Dimension(Box<Dimension>),
    NotSupported(String, Vec<Atom<'static>>),
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
impl Default for Insert {
    fn default() -> Self {
        Self {
            block_name: String::default(),
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Line {
    pub p1: [f64; 3],
    pub p2: [f64; 3],
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Text {
    pub thickness: Option<f64>,                // 39 (default = 0)
    pub point1: [f64; 3],                      // 10, 20, 30
    pub point2: [f64; 3],                      // 11, 21, 31 (ignored if alignment is default)
    pub height: f64,                           // 40
    pub text: String,                          // 1
    pub rotation_degree: Option<f64>,          // 50 (default = 0)
    pub relative_x_scale_factor: Option<f64>,  // 41 (default = 1)
    pub oblique_degree: Option<f64>,           // 51 (default = 0)
    pub style_name: Option<String>,            // 7 (default = STANDARD)
    pub mirror_flags: Option<TextMirrorFlags>, // 71 (default = 0)
    pub alignment: TextAlignment,              // 72, 73
    pub extrusion_vector: Option<[f64; 3]>,    // 210, 220, 230 (default = [0, 0, 1])
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub struct TextMirrorFlags {
    pub x: bool,
    pub y: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TextAlignment {
    Combo(TextHorizontalAlignment, TextVerticalAlignment),
    Aligned,
    Middle,
    Fit,
}
impl Default for TextAlignment {
    fn default() -> Self {
        Self::Combo(
            TextHorizontalAlignment::Left,
            TextVerticalAlignment::Baseline,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TextHorizontalAlignment {
    Left,
    Center,
    Right,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TextVerticalAlignment {
    Baseline,
    Bottom,
    Middle,
    Top,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Dimension {
    pub version: i16,                                  // 280
    pub block_name: String,                            // 2
    pub definition_point: [f64; 3],                    // 10, 20, 30 (WCS)
    pub text_mid_point: [f64; 3],                      // 11, 21, 31 (OCS)
    pub dimension_type: DimensionType,                 // 70
    pub dimension_flags: DimensionFlags,               // 70
    pub attachment_point: AttachmentPoint,             // 71
    pub text_line_spacing_style: TextLineSpacingStyle, // 72
    pub text_line_spacing_factor: Option<f64>,         // 41
    pub actual_measurement: Option<f64>,               // 42
    pub text: Option<String>,                          // 1
    pub text_rotation_angle: Option<f64>,              // 53
    pub horizontal_direction_angle: Option<f64>,       // 51
    pub extrusion_direction: Option<[f64; 3]>,         // 210, 220, 230
    pub definition_point2: Option<[f64; 3]>,           // 13, 23, 33 (WCS)
    pub definition_point3: Option<[f64; 3]>,           // 14, 24, 34 (WCS)
    pub definition_point4: Option<[f64; 3]>,           // 15, 25, 35 (WCS)
    pub insertion_point: Option<[f64; 3]>,             // 12, 22, 32 (OCS)
    pub arc_location: Option<[f64; 3]>,                // 16, 26, 36 (OCS)
    pub rotation_angle: Option<f64>,                   // 50
    pub oblique_angle: Option<f64>,                    // 52
    pub leader_length: Option<f64>,                    // 40
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DimensionType {
    RotatedOrHorizontalOrVertical,
    Aligned,
    Angular,
    Diameter,
    Radius,
    Angular3Point,
    Ordinate(OrdinateType),
}
impl Default for DimensionType {
    fn default() -> Self {
        Self::RotatedOrHorizontalOrVertical
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum OrdinateType {
    X,
    Y,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DimensionFlags {
    pub block_is_referenced_by_this_dimension_only: bool,
    pub dimension_text_is_positioned_at_user_defined_location: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AttachmentPoint {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}
impl Default for AttachmentPoint {
    fn default() -> Self {
        Self::TopLeft
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TextLineSpacingStyle {
    AtLeast,
    Exact,
}
impl Default for TextLineSpacingStyle {
    fn default() -> Self {
        Self::AtLeast
    }
}
