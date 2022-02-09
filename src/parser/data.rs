use crate::*;
use std::str::FromStr;

fn parse_value_error<T>(s: &str) -> ParseError {
    ParseError::ParseValueError {
        source_str: s.to_owned(),
        target_type: std::any::type_name::<T>(),
    }
}

impl FromStr for Space {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<i16>()? {
            0 => Self::ModelSpace,
            1 => Self::PaperSpace,
            _ => return Err(parse_value_error::<Self>(s)),
        })
    }
}

impl FromStr for LineTypeRef {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "BYLAYER" => Self::ByLayer,
            "BYBLOCK" => Self::ByBlock,
            _ => LineTypeRef::ByName(s.to_owned()),
        })
    }
}

impl FromStr for ColorNumber {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<i16>()? {
            0 => Self::ByBlock,
            256 => Self::ByLayer,
            257 => Self::ByEntity,
            i if i < 0 => Self::TurnedOff,
            i if i < 256 => Self::Number(i as u8),
            _ => return Err(parse_value_error::<Self>(s)),
        })
    }
}

impl FromStr for ShadowMode {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<i16>()? {
            0 => Self::CastsAndReceivesShadows,
            1 => Self::CastsShadows,
            2 => Self::ReceivesShadows,
            3 => Self::IgnoresShadows,
            _ => return Err(parse_value_error::<Self>(s)),
        })
    }
}

impl FromStr for Rgb {
    type Err = <i32 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i32>().map(|bits| Self {
            r: ((bits & 0xff0000) >> 16) as u8,
            g: ((bits & 0x00ff00) >> 8) as u8,
            b: (bits & 0x0000ff) as u8,
        })
    }
}

impl FromStr for DimensionType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let flags = s.parse::<i16>()?;
        Ok(match flags & 0b1111 {
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
            _ => return Err(parse_value_error::<Self>(s)),
        })
    }
}

impl FromStr for AttachmentPoint {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<i16>()? {
            0 => Self::TopLeft,
            1 => Self::TopCenter,
            2 => Self::TopRight,
            3 => Self::MiddleLeft,
            4 => Self::MiddleCenter,
            5 => Self::MiddleRight,
            6 => Self::BottomLeft,
            7 => Self::BottomCenter,
            8 => Self::BottomRight,
            _ => return Err(parse_value_error::<Self>(s)),
        })
    }
}

impl FromStr for TextLineSpacingStyle {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<i16>()? {
            1 => Self::AtLeast,
            2 => Self::Exact,
            _ => return Err(parse_value_error::<Self>(s)),
        })
    }
}

impl FromStr for TextMirrorFlags {
    type Err = <i16 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i16>().map(|flags| Self {
            x: (flags & 0b010) != 0,
            y: (flags & 0b100) != 0,
        })
    }
}

impl FromStr for TextHorizontalAlignment {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<i16>()? {
            0 => Self::Left,
            1 => Self::Center,
            2 => Self::Right,
            _ => return Err(parse_value_error::<Self>(s)),
        })
    }
}

impl FromStr for TextVerticalAlignment {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<i16>()? {
            0 => Self::Baseline,
            1 => Self::Bottom,
            2 => Self::Middle,
            3 => Self::Top,
            _ => return Err(parse_value_error::<Self>(s)),
        })
    }
}

impl FromStr for MTextDirection {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<i16>()? {
            1 => MTextDirection::LeftToRight,
            3 => MTextDirection::TopToBottom,
            5 => MTextDirection::ByStyle,
            _ => return Err(parse_value_error::<Self>(s)),
        })
    }
}
