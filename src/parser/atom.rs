#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParAtom<'a> {
    pub code: i16,
    pub value: &'a str,
}

impl<'a> ParAtom<'a> {
    pub fn parse(s: &'a str) -> Result<Vec<Self>, std::num::ParseIntError> {
        s.lines()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| {
                let value = chunk[1].trim();
                chunk[0]
                    .trim()
                    .parse::<i16>()
                    .map(|code| Self { code, value })
            })
            .collect()
    }
    pub fn get<T: std::str::FromStr>(&self) -> Option<T>
    where
        T::Err: std::fmt::Display,
    {
        match self.value.parse() {
            Ok(value) => Some(value),
            Err(e) => {
                log::error!("{} @ {:?}", e, self);
                None
            }
        }
    }
    pub fn get_to<T: std::str::FromStr>(&self, dst: &mut T)
    where
        T::Err: std::fmt::Display,
    {
        if let Some(value) = self.get() {
            *dst = value;
        }
    }
    pub fn get_bool_to(&self, dst: &mut bool) {
        if let Some(value) = self.get::<i32>() {
            *dst = value != 0;
        }
    }
}

impl<'a> From<ParAtom<'a>> for crate::DxfAtom {
    fn from(atom: ParAtom<'a>) -> Self {
        Self {
            code: atom.code,
            value: atom.value.to_owned(),
        }
    }
}

use crate::{Atom, DxfParseResult, Value};
use std::borrow::Cow;
impl<'a> Atom<'a> {
    pub fn parse_str(s: &'a str) -> DxfParseResult<Vec<Self>> {
        Self::parse_lines(&s.lines().collect::<Vec<_>>())
    }
    pub fn parse_lines(lines: &[&'a str]) -> DxfParseResult<Vec<Self>> {
        lines
            .chunks(2)
            .map(|chunk| Self::parse_line_pair(chunk[0].trim(), chunk[1].trim()))
            .collect()
    }
    pub fn parse_line_pair(line1: &'a str, line2: &'a str) -> DxfParseResult<Self> {
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
