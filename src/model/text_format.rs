#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MTextFormatString {
    pub raw: String,
    pub nodes: Vec<MTextNode>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MTextNode {
    Text(String),
    Command(MTextCommand),
    Block(Vec<Self>),
    Stacked(Vec<Self>, Vec<Self>, MTextStackType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MTextStackType {
    Slash,  // '/'
    Number, // '#'
    Hat,    // '^'
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MTextCommand {
    OStart,
    OEnd,
    LStart,
    LEnd,
    KStart,
    KEnd,
    C(i16),
    F(String),
    H(f64),
    Hx(f64),
    T(f64),
    Q(f64),
    W(f64),
    A(MTextAlignment),
    P,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MTextAlignment {
    Bottom,
    Center,
    Top,
}
