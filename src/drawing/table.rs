#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableNode {
    pub handle: u32,
    pub entries: Vec<TableEntry>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableEntry {
    pub handle: u32,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Layer {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LineType {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextStyle {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ucs {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct View {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Viewport {}
