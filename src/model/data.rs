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

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum MTextDirection {
    LeftToRight,
    TopToBottom,
    ByStyle,
}
impl Default for MTextDirection {
    fn default() -> Self {
        Self::LeftToRight
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum MTextBackground {
    WindowColor,
    ColorNumber(i16),
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PolylineFlags {
    pub closed_polyline: bool,
    pub curve_fit_vertices: bool,
    pub spline_fit_vertices: bool,
    pub polyline_3d: bool,
    pub polygon_mesh_3d: bool,
    pub closed_in_n_direction: bool,
    pub polyface_mesh: bool,
    pub continuous_linetype_pattern: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PolylineSmoothType {
    QuadraticBSpline,
    CubicBSpline,
    Bezier,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct VertexFlags {
    // not implemented
}
