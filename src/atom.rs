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
        let value = Value(Cow::Borrowed(line2));
        Ok(Self { code, value })
    }
}

impl<'a> std::fmt::Display for Atom<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:3}", self.code)?;
        writeln!(f, "{}", self.value)?;
        Ok(())
    }
}
