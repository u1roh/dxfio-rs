#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MTextNode {
    Text(std::ops::Range<usize>),
    Command(MTextCommand),
    Stacked(Box<Self>, Box<Self>, MTextStackType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MTextStackType {
    Slash,  // '/'
    Number, // '#'
    Hat,    // '^'
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MTextCommand {
    O,
    L,
    K,
    C(i16),
    F(String),
    H(i16),
    Hx(f64),
    T(f64),
    Q(f64),
    W(f64),
    A(MTextAlignment),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MTextAlignment {
    Bottom,
    Center,
    Top,
}
