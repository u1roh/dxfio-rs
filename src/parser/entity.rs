use super::{FromNode, ParAtom, ParNode, SetAtom};
use super::{FromNode2, SetAtom2};
use crate::*;

impl FromNode for EntityNode {
    fn from_node(source: &ParNode) -> Self {
        match source.node_type {
            "INSERT" => parse_by(source, Entity::Insert),
            "DIMENSION" => parse_by(source, Entity::Dimension),
            "LINE" => parse_by(source, Entity::Line),
            _ => parse_by(source, |atoms| {
                Entity::NotSupported(source.node_type.to_owned(), atoms)
            }),
        }
    }
}
fn parse_by<T: SetAtom>(source: &ParNode, f: impl Fn(T) -> Entity) -> EntityNode {
    let (header, entity) = FromNode::from_node(source);
    EntityNode {
        header,
        entity: f(entity),
    }
}

impl<T: SetAtom> SetAtom for (EntityHeader, T) {
    fn set_atom(&mut self, atom: &ParAtom) -> bool {
        if SetAtom::set_atom(&mut self.0, atom) || self.1.set_atom(atom) {
            true
        } else {
            self.0.extras.push((*atom).into());
            false
        }
    }
}

impl SetAtom for Vec<DxfAtom> {
    fn set_atom(&mut self, atom: &ParAtom) -> bool {
        self.push((*atom).into());
        true
    }
}

impl SetAtom for Line {
    fn set_atom(&mut self, atom: &ParAtom) -> bool {
        match atom.code {
            10 => atom.get_to(&mut self.p1[0]),
            20 => atom.get_to(&mut self.p1[1]),
            30 => atom.get_to(&mut self.p1[2]),
            11 => atom.get_to(&mut self.p2[0]),
            21 => atom.get_to(&mut self.p2[1]),
            31 => atom.get_to(&mut self.p2[2]),
            _ => return false,
        }
        true
    }
}

impl SetAtom2 for Line {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        match atom.code {
            10 => atom.value.get_to(&mut self.p1[0]),
            20 => atom.value.get_to(&mut self.p1[1]),
            30 => atom.value.get_to(&mut self.p1[2]),
            11 => atom.value.get_to(&mut self.p2[0]),
            21 => atom.value.get_to(&mut self.p2[1]),
            31 => atom.value.get_to(&mut self.p2[2]),
            _ => false,
        }
    }
}

impl SetAtom for Insert {
    fn set_atom(&mut self, atom: &ParAtom) -> bool {
        match atom.code {
            2 => atom.get_to(&mut self.block_name),
            10 => atom.get_to(&mut self.insertion_point[0]),
            20 => atom.get_to(&mut self.insertion_point[1]),
            30 => atom.get_to(&mut self.insertion_point[2]),
            41 => atom.get_to(&mut self.scale_factor[0]),
            42 => atom.get_to(&mut self.scale_factor[1]),
            43 => atom.get_to(&mut self.scale_factor[2]),
            50 => atom.get_to(&mut self.rotation_degree),
            70 => atom.get_to(&mut self.column_count),
            71 => atom.get_to(&mut self.row_count),
            44 => atom.get_to(&mut self.column_spacing),
            45 => atom.get_to(&mut self.row_spacing),
            210 => atom.get_to(&mut self.extrusion_direction[0]),
            220 => atom.get_to(&mut self.extrusion_direction[1]),
            230 => atom.get_to(&mut self.extrusion_direction[2]),
            _ => return false,
        }
        true
    }
}

impl SetAtom2 for Insert {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        match atom.code {
            2 => atom.value.get_to(&mut self.block_name),
            10 => atom.value.get_to(&mut self.insertion_point[0]),
            20 => atom.value.get_to(&mut self.insertion_point[1]),
            30 => atom.value.get_to(&mut self.insertion_point[2]),
            41 => atom.value.get_to(&mut self.scale_factor[0]),
            42 => atom.value.get_to(&mut self.scale_factor[1]),
            43 => atom.value.get_to(&mut self.scale_factor[2]),
            50 => atom.value.get_to(&mut self.rotation_degree),
            70 => atom.value.get_to(&mut self.column_count),
            71 => atom.value.get_to(&mut self.row_count),
            44 => atom.value.get_to(&mut self.column_spacing),
            45 => atom.value.get_to(&mut self.row_spacing),
            210 => atom.value.get_to(&mut self.extrusion_direction[0]),
            220 => atom.value.get_to(&mut self.extrusion_direction[1]),
            230 => atom.value.get_to(&mut self.extrusion_direction[2]),
            _ => false,
        }
    }
}

impl SetAtom for Box<Dimension> {
    fn set_atom(&mut self, atom: &ParAtom) -> bool {
        match atom.code {
            280 => atom.get_to(&mut self.version),
            2 => atom.get_to(&mut self.block_name),

            10 => atom.get_to(&mut self.definition_point[0]),
            20 => atom.get_to(&mut self.definition_point[1]),
            30 => atom.get_to(&mut self.definition_point[2]),

            11 => atom.get_to(&mut self.text_mid_point[0]),
            21 => atom.get_to(&mut self.text_mid_point[1]),
            31 => atom.get_to(&mut self.text_mid_point[2]),

            70 => {
                if let Some(flags) = atom.get::<i16>() {
                    self.dimension_type = match flags & 0b1111 {
                        0 => DimensionType::RotatedOrHorizontalOrVertical,
                        1 => DimensionType::Aligned,
                        2 => DimensionType::Angular,
                        3 => DimensionType::Diameter,
                        4 => DimensionType::Radius,
                        5 => DimensionType::Angular3Point,
                        _ => DimensionType::Ordinate(if flags & 0b1000000 != 0 {
                            OrdinateType::X
                        } else {
                            OrdinateType::Y
                        }),
                    };
                    self.dimension_flags
                        .block_is_referenced_by_this_dimension_only = flags & 0b100000 != 0;
                    self.dimension_flags
                        .dimension_text_is_positioned_at_user_defined_location =
                        flags & 0b10000000 != 0;
                }
            }
            71 => {
                self.attachment_point = match atom.get::<i16>().unwrap_or_default() {
                    0 => AttachmentPoint::TopLeft,
                    1 => AttachmentPoint::TopCenter,
                    2 => AttachmentPoint::TopRight,
                    3 => AttachmentPoint::MiddleLeft,
                    4 => AttachmentPoint::MiddleCenter,
                    5 => AttachmentPoint::MiddleRight,
                    6 => AttachmentPoint::BottomLeft,
                    7 => AttachmentPoint::BottomCenter,
                    _ => AttachmentPoint::BottomRight,
                }
            }
            72 => {
                if atom.get::<i16>() == Some(2) {
                    self.text_line_spacing_style = TextLineSpacingStyle::Exact;
                }
            }
            41 => self.text_line_spacing_factor = atom.get(),
            42 => self.actual_measurement = atom.get(),
            1 => self.text = atom.get(),
            53 => self.text_rotation_angle = atom.get(),
            54 => self.horizontal_direction_angle = atom.get(),
            _ => {
                log::warn!("unhandled atom: {:?}", atom);
                return false;
            }
        }
        true
    }
}

impl SetAtom2 for Box<Dimension> {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        match atom.code {
            280 => atom.value.get_to(&mut self.version),
            2 => atom.value.get_to(&mut self.block_name),

            10 => atom.value.get_to(&mut self.definition_point[0]),
            20 => atom.value.get_to(&mut self.definition_point[1]),
            30 => atom.value.get_to(&mut self.definition_point[2]),

            11 => atom.value.get_to(&mut self.text_mid_point[0]),
            21 => atom.value.get_to(&mut self.text_mid_point[1]),
            31 => atom.value.get_to(&mut self.text_mid_point[2]),

            70 => {
                let success1 = atom.value.get_to(&mut self.dimension_type);
                let success2 = if let Some(flags) = atom.value.get::<i16>() {
                    self.dimension_flags
                        .block_is_referenced_by_this_dimension_only = flags & 0b100000 != 0;
                    self.dimension_flags
                        .dimension_text_is_positioned_at_user_defined_location =
                        flags & 0b10000000 != 0;
                    true
                } else {
                    false
                };
                success1 && success2
            }
            71 => atom.value.get_to(&mut self.attachment_point),
            72 => atom.value.get_to(&mut self.text_line_spacing_style),
            41 => atom.value.get_to(&mut self.text_line_spacing_factor),
            42 => atom.value.get_to(&mut self.actual_measurement),
            1 => atom.value.get_to(&mut self.text),
            53 => atom.value.get_to(&mut self.text_rotation_angle),
            54 => atom.value.get_to(&mut self.horizontal_direction_angle),
            _ => {
                log::warn!("unhandled atom: {:?}", atom);
                false
            }
        }
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

impl SetAtom for EntityHeader {
    fn set_atom(&mut self, atom: &super::ParAtom) -> bool {
        match atom.code {
            5 => self.handle = u32::from_str_radix(atom.value, 16).unwrap_or_default(),
            67 => {
                self.space = match atom.value {
                    "0" => Space::ModelSpace,
                    "1" => Space::PaperSpace,
                    _ => {
                        log::error!("unknown space: {:?}", atom);
                        Space::ModelSpace // fallback
                    }
                };
            }
            8 => self.layer = atom.value.to_owned(),
            6 => {
                self.line_type = match atom.value {
                    "BYLAYER" => LineTypeRef::ByLayer,
                    "BYBLOCK" => LineTypeRef::ByBlock,
                    _ => LineTypeRef::ByName(atom.value.to_owned()),
                };
            }
            62 => {
                self.color_number = match atom.get().unwrap_or_default() {
                    0 => ColorNumber::ByBlock,
                    256 => ColorNumber::ByLayer,
                    257 => ColorNumber::ByEntity,
                    i if i < 0 => ColorNumber::TurnedOff,
                    i if i < 256 => ColorNumber::Number(i as u8),
                    i => {
                        log::error!("invalid color number: {}", i);
                        ColorNumber::ByBlock // fallback
                    }
                };
            }
            370 => self.line_weight = atom.get(),
            48 => self.line_type_scale = atom.get(),
            60 => self.is_visible = atom.value == "0",
            420 => {
                self.color_rgb = atom.get().map(|bits: u32| Rgb {
                    r: ((bits & 0xff0000) >> 16) as u8,
                    g: ((bits & 0x00ff00) >> 8) as u8,
                    b: (bits & 0x0000ff) as u8,
                });
            }
            430 => self.color_name = Some(atom.value.to_owned()),
            440 => self.transparency = atom.get(),
            284 => {
                self.shadow_mode = atom.get().map(|mode: i16| match mode {
                    0 => ShadowMode::CastsAndReceivesShadows,
                    1 => ShadowMode::CastsShadows,
                    2 => ShadowMode::ReceivesShadows,
                    _ => ShadowMode::IgnoresShadows,
                })
            }
            _ => return false,
        }
        true
    }
}

impl SetAtom2 for EntityHeader {
    fn set_atom(&mut self, atom: &super::Atom) -> bool {
        match atom.code {
            5 => atom.value.get_to(&mut self.handle),
            67 => atom.value.get_to(&mut self.space),
            8 => atom.value.get_to(&mut self.layer),
            6 => atom.value.get_to(&mut self.line_type),
            62 => atom.value.get_to(&mut self.color_number),
            370 => atom.value.get_to(&mut self.line_weight),
            48 => atom.value.get_to(&mut self.line_type_scale),
            60 => {
                self.is_visible = match atom.value.get::<i16>() {
                    Some(0) => true,
                    Some(1) => false,
                    _ => return false,
                };
                true
            }
            420 => atom.value.get_to(&mut self.color_rgb),
            430 => atom.value.get_to(&mut self.color_name),
            440 => atom.value.get_to(&mut self.transparency),
            284 => atom.value.get_to(&mut self.shadow_mode),
            _ => false,
        }
    }
}

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
