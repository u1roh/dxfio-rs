use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Value<'a> {
    String(Cow<'a, str>),
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
    pub fn and_then_to<T: FromValue<'a>, U>(
        &'a self,
        dst: &mut U,
        f: impl Fn(T) -> Option<U>,
    ) -> bool {
        if let Some(x) = self.get().and_then(f) {
            *dst = x;
            true
        } else {
            log::error!(
                "Value::and_then_to::<{}>({:?}, dst: &mut {}) failed",
                std::any::type_name::<T>(),
                self,
                std::any::type_name::<U>(),
            );
            false
        }
    }
    pub fn as_handle(&self) -> Option<u32> {
        match self {
            Self::String(s) => u32::from_str_radix(s, 16).ok(),
        }
    }
    pub fn as_handle_to(&self, dst: &mut u32) -> bool {
        if let Some(handle) = self.as_handle() {
            *dst = handle;
            true
        } else {
            false
        }
    }
    pub fn get_optional_coord_to(&self, i: usize, dst: &mut Option<[f64; 3]>) -> bool {
        if let Some(x) = self.get() {
            if let Some(dst) = dst {
                dst[i] = x;
            } else {
                let mut coord = [0.0, 0.0, 0.0];
                coord[i] = x;
                *dst = Some(coord);
            }
            true
        } else {
            false
        }
    }
    pub fn into_owned(self) -> Value<'static> {
        match self {
            Self::String(s) => Value::String(Cow::Owned(s.into_owned())),
        }
    }
}

impl<'a> std::fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => s.fmt(f),
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
        }
    }
}
impl<'a> FromValue<'a> for String {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::String(s) => Some(s.as_ref().to_owned()),
        }
    }
}
impl<'a> FromValue<'a> for f64 {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::String(s) => s.parse().ok(),
        }
    }
}
impl<'a> FromValue<'a> for i64 {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::String(s) => s.parse().ok(),
        }
    }
}
impl<'a> FromValue<'a> for i32 {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::String(s) => s.parse().ok(),
        }
    }
}
impl<'a> FromValue<'a> for i16 {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::String(s) => s.parse().ok(),
        }
    }
}
impl<'a> FromValue<'a> for u32 {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::String(s) => s.parse().ok(),
        }
    }
}
impl<'a> FromValue<'a> for usize {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::String(s) => s.parse().ok(),
        }
    }
}
