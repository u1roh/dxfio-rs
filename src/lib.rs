pub mod parser;

mod model;
pub use model::*;

mod atom;
pub use atom::Atom;

mod node;
pub use node::Node;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error(transparent)]
    EncodingError(#[from] parser::EncodingError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("failed to parse \"{source_str}\" to `{target_type}`")]
    ParseValueError {
        source_str: String,
        target_type: &'static str,
    },

    #[error("value of group-code {code} not found")]
    ValueNotFound { code: i16 },
}

pub type ParseResult<T> = Result<T, ParseError>;

pub trait AtomList {
    fn find(&self, code: i16) -> Option<&str>;
    fn get(&self, code: i16) -> ParseResult<&str> {
        self.find(code).ok_or(ParseError::ValueNotFound { code })
    }
    fn get_point(&self, i: usize) -> [f64; 3] {
        let get = |code| {
            self.find(code)
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or_default()
        };
        [get(10 + i as i16), get(20 + i as i16), get(30 + i as i16)]
    }
}

impl<'a> AtomList for [Atom<'a>] {
    fn find(&self, code: i16) -> Option<&str> {
        self.iter()
            .find(|item| item.code == code)
            .map(|item| &item.value as &str)
    }
}

impl<'a> AtomList for std::borrow::Cow<'a, [Atom<'a>]> {
    fn find(&self, code: i16) -> Option<&str> {
        (**self).find(code)
    }
}
