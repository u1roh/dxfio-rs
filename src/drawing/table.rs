#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableNode {
    pub handle: u32,
    pub table: Table,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Table {
    RegApp(RegApp), // APPID
    Block(Block),   // BLOCK_RECORD
    DimStyle(DimStyle),
    Layer(Layer),
    LineType(LineType),
    TextStyle(TextStyle),
    Ucs(Ucs),
    View(View),
    Viewport(Viewport),
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
