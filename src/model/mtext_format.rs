#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MTextParagraph {
    Text(String, Vec<(MTextFormat, std::ops::Range<usize>)>),
    Stacked(Box<Self>, Box<Self>, MTextStackType, MTextStackAlignment),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MTextStackType {
    Slash,  // '/'
    Number, // '#'
    Hat,    // '^'
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MTextStackAlignment {
    Bottom,
    Middle,
    Top,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MTextFormat {
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
}
