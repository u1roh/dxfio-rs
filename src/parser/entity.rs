use super::{FromNode, ParNode, SetAtom};
use crate::*;

impl FromNode for EntityNode {
    fn from_node(source: &ParNode) -> Self {
        Self {
            header: FromNode::from_node(source),
            entity: FromNode::from_node(source),
        }
    }
}

impl FromNode for Entity {
    fn from_node(source: &ParNode) -> Self {
        match source.node_type {
            "INSERT" => Self::Insert(FromNode::from_node(source)),
            "LINE" => Self::Line(FromNode::from_node(source)),
            "DIMENSION" => Self::Dimension(Box::new(FromNode::from_node(source))),
            _ => Entity::NotSupported(source.into()),
        }
    }
}

impl FromNode for Line {
    fn from_node(source: &ParNode) -> Self {
        assert_eq!(source.node_type, "LINE");
        Self {
            p1: source.atoms.get_point(0),
            p2: source.atoms.get_point(1),
        }
    }
}

impl FromNode for Insert {
    fn from_node(source: &ParNode) -> Self {
        assert_eq!(source.node_type, "INSERT");
        let mut insert = Self::new(source.atoms.get_or_default(2));
        for atom in source.atoms {
            match atom.code {
                10 => atom.get_to(&mut insert.insertion_point[0]),
                20 => atom.get_to(&mut insert.insertion_point[1]),
                30 => atom.get_to(&mut insert.insertion_point[2]),
                41 => atom.get_to(&mut insert.scale_factor[0]),
                42 => atom.get_to(&mut insert.scale_factor[1]),
                43 => atom.get_to(&mut insert.scale_factor[2]),
                50 => atom.get_to(&mut insert.rotation_degree),
                70 => atom.get_to(&mut insert.column_count),
                71 => atom.get_to(&mut insert.row_count),
                44 => atom.get_to(&mut insert.column_spacing),
                45 => atom.get_to(&mut insert.row_spacing),
                210 => atom.get_to(&mut insert.extrusion_direction[0]),
                220 => atom.get_to(&mut insert.extrusion_direction[1]),
                230 => atom.get_to(&mut insert.extrusion_direction[2]),
                _ => {}
            }
        }
        insert
    }
}

impl FromNode for Dimension {
    fn from_node(source: &ParNode) -> Self {
        assert_eq!(source.node_type, "DIMENSION");
        let mut dst = Self::default();
        for atom in source.atoms {
            match atom.code {
                280 => atom.get_to(&mut dst.version),
                2 => atom.get_to(&mut dst.block_name),

                10 => atom.get_to(&mut dst.definition_point[0]),
                20 => atom.get_to(&mut dst.definition_point[1]),
                30 => atom.get_to(&mut dst.definition_point[2]),

                11 => atom.get_to(&mut dst.text_mid_point[0]),
                21 => atom.get_to(&mut dst.text_mid_point[1]),
                31 => atom.get_to(&mut dst.text_mid_point[2]),

                70 => {
                    if let Some(flags) = atom.get::<i16>() {
                        dst.dimension_type = match flags & 0b1111 {
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
                        dst.dimension_flags
                            .block_is_referenced_by_this_dimension_only = flags & 0b100000 != 0;
                        dst.dimension_flags
                            .dimension_text_is_positioned_at_user_defined_location =
                            flags & 0b10000000 != 0;
                    }
                }
                71 => {
                    dst.attachment_point = match atom.get::<i16>().unwrap_or_default() {
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
                        dst.text_line_spacing_style = TextLineSpacingStyle::Exact;
                    }
                }
                41 => dst.text_line_spacing_factor = atom.get(),
                42 => dst.actual_measurement = atom.get(),
                1 => dst.text = atom.get(),
                53 => dst.text_rotation_angle = atom.get(),
                54 => dst.horizontal_direction_angle = atom.get(),
                _ => {
                    log::warn!("unhandled atom: {:?}", atom);
                }
            }
        }
        dst
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
