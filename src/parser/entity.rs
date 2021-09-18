use super::{FromNode2, SetAtom2};
use crate::*;

impl FromNode2 for EntityNode {
    fn from_node(source: &Node) -> Self {
        match source.node_type.as_ref() {
            "INSERT" => parse_by2(source, Entity::Insert),
            "DIMENSION" => parse_by2(source, Entity::Dimension),
            "LINE" => parse_by2(source, Entity::Line),
            _ => parse_by2(source, |atoms| {
                Entity::NotSupported((*source.node_type).to_owned(), atoms)
            }),
        }
    }
}
fn parse_by2<T: SetAtom2>(source: &Node, f: impl Fn(T) -> Entity) -> EntityNode {
    let (header, entity) = FromNode2::from_node(source);
    EntityNode {
        header,
        entity: f(entity),
    }
}

impl<T: SetAtom2> SetAtom2 for (EntityHeader, T) {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        if SetAtom2::set_atom(&mut self.0, atom) || self.1.set_atom(atom) {
            true
        } else {
            unimplemented!();
            // self.0.extras.push((*atom).into());
            false
        }
    }
}

impl SetAtom2 for Vec<DxfAtom> {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        unimplemented!();
        // self.push((*atom).into());
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
