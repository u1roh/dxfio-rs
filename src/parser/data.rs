use crate::*;

impl<'a> crate::value::FromValue<'a> for Space {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        value.get::<i16>().and_then(|value| {
            Some(match value {
                0 => Self::ModelSpace,
                1 => Self::PaperSpace,
                _ => return None,
            })
        })
    }
}

impl<'a> crate::value::FromValue<'a> for LineTypeRef {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        value.get::<&str>().map(|value| match value {
            "BYLAYER" => Self::ByLayer,
            "BYBLOCK" => Self::ByBlock,
            _ => LineTypeRef::ByName(value.to_owned()),
        })
    }
}

impl<'a> crate::value::FromValue<'a> for ColorNumber {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        value.get::<i16>().and_then(|value| {
            Some(match value {
                0 => Self::ByBlock,
                256 => Self::ByLayer,
                257 => Self::ByEntity,
                i if i < 0 => Self::TurnedOff,
                i if i < 256 => Self::Number(i as u8),
                _ => return None,
            })
        })
    }
}

impl<'a> crate::value::FromValue<'a> for ShadowMode {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        value.get::<i16>().and_then(|value| {
            Some(match value {
                0 => Self::CastsAndReceivesShadows,
                1 => Self::CastsShadows,
                2 => Self::ReceivesShadows,
                3 => Self::IgnoresShadows,
                _ => return None,
            })
        })
    }
}

impl<'a> crate::value::FromValue<'a> for Rgb {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        value.get::<i32>().map(|bits| Self {
            r: ((bits & 0xff0000) >> 16) as u8,
            g: ((bits & 0x00ff00) >> 8) as u8,
            b: (bits & 0x0000ff) as u8,
        })
    }
}

impl<'a> crate::value::FromValue<'a> for DimensionType {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        value.get::<i16>().and_then(|flags| {
            Some(match flags & 0b1111 {
                0 => Self::RotatedOrHorizontalOrVertical,
                1 => Self::Aligned,
                2 => Self::Angular,
                3 => Self::Diameter,
                4 => Self::Radius,
                5 => Self::Angular3Point,
                6 => Self::Ordinate(if flags & 0b1000000 != 0 {
                    OrdinateType::X
                } else {
                    OrdinateType::Y
                }),
                _ => return None,
            })
        })
    }
}

impl<'a> crate::value::FromValue<'a> for AttachmentPoint {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        value.get::<i16>().and_then(|value| {
            Some(match value {
                0 => Self::TopLeft,
                1 => Self::TopCenter,
                2 => Self::TopRight,
                3 => Self::MiddleLeft,
                4 => Self::MiddleCenter,
                5 => Self::MiddleRight,
                6 => Self::BottomLeft,
                7 => Self::BottomCenter,
                8 => Self::BottomRight,
                _ => return None,
            })
        })
    }
}

impl<'a> crate::value::FromValue<'a> for TextLineSpacingStyle {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        value.get::<i16>().and_then(|value| {
            Some(match value {
                1 => Self::AtLeast,
                2 => Self::Exact,
                _ => return None,
            })
        })
    }
}

impl<'a> crate::value::FromValue<'a> for TextMirrorFlags {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        value.get::<i16>().map(|flags| Self {
            x: (flags & 0b010) != 0,
            y: (flags & 0b100) != 0,
        })
    }
}

impl<'a> crate::value::FromValue<'a> for TextHorizontalAlignment {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        value.get::<i16>().and_then(|a| {
            Some(match a {
                0 => Self::Left,
                1 => Self::Center,
                2 => Self::Right,
                _ => return None,
            })
        })
    }
}

impl<'a> crate::value::FromValue<'a> for TextVerticalAlignment {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        value.get::<i16>().and_then(|a| {
            Some(match a {
                0 => Self::Baseline,
                1 => Self::Bottom,
                2 => Self::Middle,
                3 => Self::Top,
                _ => return None,
            })
        })
    }
}

impl<'a> crate::value::FromValue<'a> for MTextDirection {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        value.get::<i16>().and_then(|value| {
            Some(match value {
                1 => MTextDirection::LeftToRight,
                3 => MTextDirection::TopToBottom,
                5 => MTextDirection::ByStyle,
                _ => return None,
            })
        })
    }
}
