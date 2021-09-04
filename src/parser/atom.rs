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
                let value = chunk[1];
                chunk[0]
                    .trim()
                    .parse::<i16>()
                    .map(|code| Self { code, value })
            })
            .collect()
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
