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
            log::error!(
                "Value::get_to({:?}, dst: &mut {}) failed",
                self,
                std::any::type_name::<T>(),
            );
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

impl<'a, T: FromValue<'a>> FromValue<'a> for Option<T> {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        Some(T::from_value(value))
    }
}

impl<'a> FromValue<'a> for &'a str {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}
impl<'a> FromValue<'a> for String {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::String(s) => Some(s.as_ref().to_owned()),
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
impl<'a> FromValue<'a> for u32 {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::Handle(x) => Some(*x),
            _ => None,
        }
    }
}
impl<'a> FromValue<'a> for usize {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::I16(x) if *x >= 0 => Some(*x as _),
            Value::I32(x) if *x >= 0 => Some(*x as _),
            Value::I64(x) if *x >= 0 => Some(*x as _),
            _ => None,
        }
    }
}
