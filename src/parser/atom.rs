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
