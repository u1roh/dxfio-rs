use super::{FromNode, ParAtom, ParNode, SetAtom};
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
        if self.0.set_atom(atom) || self.1.set_atom(atom) {
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
