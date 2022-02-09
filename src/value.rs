use std::borrow::Cow;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Value<'a>(pub Cow<'a, str>);
impl<'a> std::ops::Deref for Value<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}
impl<'a> Value<'a> {
    pub fn and_then_to<T: FromStr, U>(&'a self, dst: &mut U, f: impl Fn(T) -> Option<U>) -> bool {
        if let Some(x) = self.parse().ok().and_then(f) {
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
        u32::from_str_radix(&self.0, 16).ok()
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
        if let Some(x) = self.parse().ok() {
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
        Value(Cow::Owned(self.0.into_owned()))
    }
}

impl<'a> std::fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
