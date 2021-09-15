use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Value<'a> {
    String(Cow<'a, str>),
    F64(f64),
    I64(i64),
    I32(i32),
    I16(i16),
    Bool(bool),
    Handle(u32),
    Bytes(Vec<u8>),
}
impl<'a> Value<'a> {
    pub fn get<T: FromValue<'a>>(&'a self) -> Option<T> {
        T::from_value(self)
    }
    pub fn get_to<T: FromValue<'a>>(&'a self, dst: &mut T) -> bool {
        if let Some(x) = self.get() {
            *dst = x;
            true
        } else {
            false
        }
    }
    pub fn into_owned(self) -> Value<'static> {
        match self {
            Self::String(s) => Value::String(Cow::Owned(s.into_owned())),
            Self::F64(x) => Value::F64(x),
            Self::I64(x) => Value::I64(x),
            Self::I32(x) => Value::I32(x),
            Self::I16(x) => Value::I16(x),
            Self::Bool(x) => Value::Bool(x),
            Self::Handle(x) => Value::Handle(x),
            Self::Bytes(x) => Value::Bytes(x),
        }
    }
}

pub trait FromValue<'a>: Sized {
    fn from_value(value: &'a Value<'a>) -> Option<Self>;
}

impl<'a> FromValue<'a> for &'a str {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}
impl<'a> FromValue<'a> for f64 {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::F64(x) => Some(*x),
            _ => None,
        }
    }
}
impl<'a> FromValue<'a> for i64 {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::I64(x) => Some(*x),
            _ => None,
        }
    }
}
impl<'a> FromValue<'a> for i32 {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::I32(x) => Some(*x),
            _ => None,
        }
    }
}
impl<'a> FromValue<'a> for i16 {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::I16(x) => Some(*x),
            _ => None,
        }
    }
}
