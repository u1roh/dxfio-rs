pub mod parser;

mod model;
pub use model::*;

mod value;
pub use value::Value;

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
}

pub type ParseResult<T> = Result<T, ParseError>;

pub trait AtomList {
    fn find(&self, code: i16) -> Option<&str>;
    fn get_value<'a, T: std::str::FromStr>(&'a self, code: i16) -> Option<T> {
        self.find(code)?.parse().ok()
    }
    fn get_or_default<'a, T: std::str::FromStr + Default>(&'a self, code: i16) -> T {
        self.get_value(code).unwrap_or_default()
    }
    fn get_point(&self, i: usize) -> [f64; 3] {
        [
            self.get_or_default(10 + i as i16),
            self.get_or_default(20 + i as i16),
            self.get_or_default(30 + i as i16),
        ]
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
