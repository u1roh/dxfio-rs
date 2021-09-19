use super::{FromNode, SetAtom};
use crate::*;

impl FromNode for EntityNode {
    fn from_node(source: &Node) -> Self {
        match source.node_type.as_ref() {
            "INSERT" => parse_by(source, Entity::Insert),
            "DIMENSION" => parse_by(source, Entity::Dimension),
            "LINE" => parse_by(source, Entity::Line),
            "TEXT" => parse_by(source, Entity::Text),
            "MTEXT" => parse_by(source, Entity::MText),
            _ => parse_by(source, |atoms| {
                Entity::NotSupported((*source.node_type).to_owned(), atoms)
            }),
        }
    }
}
fn parse_by<T: SetAtom>(source: &Node, f: impl Fn(T) -> Entity) -> EntityNode {
    let (header, entity) = FromNode::from_node(source);
    EntityNode {
        header,
        entity: f(entity),
    }
}

impl<T: SetAtom> SetAtom for (EntityHeader, T) {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        if SetAtom::set_atom(&mut self.0, atom) || self.1.set_atom(atom) {
            true
        } else {
            self.0.extras.push(atom.to_owned());
            false
        }
    }
}

impl<'a> SetAtom for Vec<Atom<'static>> {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        self.push(atom.to_owned());
        true
    }
}

impl SetAtom for Insert {
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

impl SetAtom for Line {
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

impl SetAtom for Text {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        match atom.code {
            1 => atom.value.get_to(&mut self.text),
            7 => atom.value.get_to(&mut self.style_name),
            10 => atom.value.get_to(&mut self.point1[0]),
            20 => atom.value.get_to(&mut self.point1[1]),
            30 => atom.value.get_to(&mut self.point1[2]),
            11 => atom.value.get_to(&mut self.point2[0]),
            21 => atom.value.get_to(&mut self.point2[1]),
            31 => atom.value.get_to(&mut self.point2[2]),
            39 => atom.value.get_to(&mut self.thickness),
            40 => atom.value.get_to(&mut self.height),
            41 => atom.value.get_to(&mut self.relative_x_scale_factor),
            50 => atom.value.get_to(&mut self.rotation_degree),
            51 => atom.value.get_to(&mut self.oblique_degree),
            71 => {
                self.mirror_flags = atom.value.get::<i16>().map(|flags| TextMirrorFlags {
                    x: (flags & 0b010) != 0,
                    y: (flags & 0b100) != 0,
                });
                self.mirror_flags.is_some()
            }
            72 => {
                if let Some(alignment) = atom.value.get::<i16>().and_then(|a| {
                    let h = match a {
                        0 => TextHorizontalAlignment::Left,
                        1 => TextHorizontalAlignment::Center,
                        2 => TextHorizontalAlignment::Right,
                        3 => return Some(TextAlignment::Aligned),
                        4 => return Some(TextAlignment::Middle),
                        5 => return Some(TextAlignment::Fit),
                        _ => return None,
                    };
                    Some(match self.alignment {
                        TextAlignment::Combo(_, v) => TextAlignment::Combo(h, v),
                        _ => TextAlignment::Combo(h, TextVerticalAlignment::Baseline),
                    })
                }) {
                    self.alignment = alignment;
                    true
                } else {
                    false
                }
            }
            73 => {
                if let Some(alignment) = atom.value.get::<i16>().and_then(|a| {
                    let v = match a {
                        0 => TextVerticalAlignment::Baseline,
                        1 => TextVerticalAlignment::Bottom,
                        2 => TextVerticalAlignment::Middle,
                        3 => TextVerticalAlignment::Top,
                        _ => return None,
                    };
                    Some(match self.alignment {
                        TextAlignment::Combo(h, _) => TextAlignment::Combo(h, v),
                        _ => TextAlignment::Combo(TextHorizontalAlignment::Left, v),
                    })
                }) {
                    self.alignment = alignment;
                    true
                } else {
                    false
                }
            }
            210 => atom
                .value
                .get_optional_coord_to(0, &mut self.extrusion_vector),
            220 => atom
                .value
                .get_optional_coord_to(1, &mut self.extrusion_vector),
            230 => atom
                .value
                .get_optional_coord_to(2, &mut self.extrusion_vector),
            _ => false,
        }
    }
}

impl SetAtom for MText {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        match atom.code {
            1 | 3 => {
                self.text += atom.value.get().unwrap_or_default();
                true
            }
            7 => atom.value.get_to(&mut self.style_name),
            10 => atom.value.get_to(&mut self.point[0]),
            20 => atom.value.get_to(&mut self.point[1]),
            30 => atom.value.get_to(&mut self.point[2]),
            11 => atom.value.get_to(&mut self.x_axis[0]),
            21 => atom.value.get_to(&mut self.x_axis[1]),
            31 => atom.value.get_to(&mut self.x_axis[2]),
            40 => atom.value.get_to(&mut self.height),
            41 => atom.value.get_to(&mut self.rectangle_width),
            42 => atom.value.get_to(&mut self.character_width),
            43 => atom.value.get_to(&mut self.character_height),
            50 => atom.value.get_to(&mut self.rotation_radian),
            210 => atom
                .value
                .get_optional_coord_to(0, &mut self.extrusion_vector),
            220 => atom
                .value
                .get_optional_coord_to(1, &mut self.extrusion_vector),
            230 => atom
                .value
                .get_optional_coord_to(2, &mut self.extrusion_vector),
            71 => atom.value.get_to(&mut self.attachment_point),
            72 => {
                if let Some(dir) = atom.value.get::<i16>().and_then(|value| {
                    Some(match value {
                        1 => MTextDirection::LeftToRight,
                        3 => MTextDirection::TopToBottom,
                        5 => MTextDirection::ByStyle,
                        _ => return None,
                    })
                }) {
                    self.drawing_direction = dir;
                    true
                } else {
                    false
                }
            }
            73 => atom.value.get_to(&mut self.line_spacing_style),
            44 => atom.value.get_to(&mut self.line_spacing_factor),
            90 => {
                self.background_fill_color =
                    match (self.background_fill_color, atom.value.get::<i32>()) {
                        (Some(MTextBackground::ColorNumber(_)), Some(1)) => return true,
                        (_, Some(0)) => None,
                        (_, Some(1)) => Some(MTextBackground::ColorNumber(0)),
                        (_, Some(2)) => Some(MTextBackground::WindowColor),
                        _ => return false,
                    };
                true
            }
            63 => {
                if let Some(color) = atom.value.get::<i16>() {
                    self.background_fill_color = Some(MTextBackground::ColorNumber(color));
                    true
                } else {
                    false
                }
            }
            45 => atom.value.get_to(&mut self.fill_box_scale),
            _ => false,
        }
    }
}

impl SetAtom for Box<Dimension> {
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
    fn set_atom(&mut self, atom: &super::Atom) -> bool {
        match atom.code {
            5 => atom.value.as_handle_to(&mut self.handle),
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
