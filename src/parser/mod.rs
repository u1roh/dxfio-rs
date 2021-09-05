mod atom;
mod drawing;
mod node;

pub use atom::ParAtom;
pub use drawing::*;
pub use node::ParNode;

use std::borrow::Cow;

#[derive(Debug, Clone, Copy, thiserror::Error)]
#[error("{:?}", self)]
pub struct EncodingError;

pub fn bytes_to_string(bytes: &[u8]) -> Result<Cow<str>, EncodingError> {
    match std::str::from_utf8(bytes) {
        Ok(s) => Ok(s.into()),
        Err(_) => {
            use encoding_rs::*;
            [SHIFT_JIS, EUC_JP]
                .iter()
                .find_map(|encoding| {
                    let (s, _, malformed) = encoding.decode(bytes);
                    if malformed {
                        None
                    } else {
                        Some(s)
                    }
                })
                .ok_or(EncodingError)
        }
    }
}
