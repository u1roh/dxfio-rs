#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableNode {
    pub handle: u32,
    pub entries: Vec<TableEntry>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableEntry {
    pub handle: u32,
    pub name: String,
    pub record: TableRecord,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TableRecord {
    RegApp(RegApp),       // APPID
    Block(Block),         // BLOCK_RECORD
    DimStyle(DimStyle),   // DIMSTYLE
    Layer(Layer),         // LAYER
    LineType(LineType),   // LTYPE
    TextStyle(TextStyle), // STYLE
    Ucs(Ucs),             // UCS
    View(View),           // VIEW
    Viewport(Viewport),   // VPORT
    Unknown(crate::DxfNode),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegApp {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Block {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DimStyle {}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Layer {
    pub is_plotted: bool,
    pub flags: u16,
    pub color_number: Option<u8>, // None means turned-off
    pub line_type: Option<String>,
    pub line_weight: Option<u16>,
    pub plot_style_handle: Option<u32>,
    pub material_handle: Option<u32>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LineType {
    pub flags: u16,
    pub description: String,
    pub total_pattern_length: f64,
    pub pattern_lengths: Vec<f64>,
}
impl LineType {
    pub fn is_continuous(&self) -> bool {
        self.pattern_lengths.is_empty()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextStyle {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ucs {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct View {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Viewport {}
