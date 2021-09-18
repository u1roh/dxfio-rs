use crate::{ParseResult, Value};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Atom<'a> {
    pub code: i16,
    pub value: Value<'a>,
}

impl<'a> Atom<'a> {
    pub fn to_owned(&self) -> Atom<'static> {
        Atom {
            code: self.code,
            value: self.value.clone().into_owned(),
        }
    }

    pub fn parse_str(s: &'a str) -> ParseResult<Vec<Self>> {
        Self::parse_lines(&s.lines().collect::<Vec<_>>())
    }
    pub fn parse_lines(lines: &[&'a str]) -> ParseResult<Vec<Self>> {
        lines
            .chunks(2)
            .map(|chunk| Self::parse_line_pair(chunk[0].trim(), chunk[1].trim()))
            .collect()
    }
    pub fn parse_line_pair(line1: &'a str, line2: &'a str) -> ParseResult<Self> {
        let code = line1.parse::<i16>()?;
        let value = match code {
            10..=59 | 110..=149 | 210..=239 | 460..=469 | 1010..=1059 => Value::F64(line2.parse()?),
            60..=79 | 170..=179 | 370..=389 | 400..=409 | 1060..=1070 => Value::I16(line2.parse()?),
            90..=99 | 420..=429 | 440..=449 | 1071 => Value::I32(line2.parse()?),
            160..=169 => Value::I64(line2.parse()?),
            290..=299 => Value::Bool(line2 != "0"),
            105 | 320..=329 | 390..=399 | 480..=481 => {
                Value::Handle(u32::from_str_radix(line2, 16)?)
            }
            310..=319 => Value::Bytes(
                (0..line2.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&line2[i..i + 2], 16))
                    .collect::<Result<_, _>>()?,
            ),
            _ => Value::String(Cow::Borrowed(line2)),
        };
        Ok(Self { code, value })
    }
}
