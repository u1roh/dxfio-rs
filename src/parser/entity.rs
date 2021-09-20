use super::{FromNode, SetAtom};
use crate::*;

impl FromNode for EntityNode {
    fn from_node(source: &Node) -> Self {
        match source.node_type.as_ref() {
            "INSERT" => parse_by(source, Entity::Insert),
            "DIMENSION" => parse_by(source, Entity::Dimension),
            "LINE" => parse_by(source, Entity::Line),
            "TEXT" => parse_by(source, Entity::Text),
            "MTEXT" => parse_by(source, |mut mtext: MText| {
                mtext.text.parse_and_build_nodes();
                Entity::MText(mtext)
            }),
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
            1 => {
                if let Some(text) = atom.value.get::<&str>() {
                    self.text = super::text_format::parse_control_codes(text);
                    true
                } else {
                    false
                }
            }
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
            71 => atom.value.get_to(&mut self.mirror_flags),
            72 => {
                if let Some(h) = atom.value.get() {
                    self.alignment = match self.alignment {
                        TextAlignment::Combo(_, v) => TextAlignment::Combo(h, v),
                        _ => TextAlignment::Combo(h, TextVerticalAlignment::Baseline),
                    };
                    true
                } else if let Some(align) = atom.value.get::<i16>().and_then(|value| {
                    Some(match value {
                        3 => TextAlignment::Aligned,
                        4 => TextAlignment::Middle,
                        5 => TextAlignment::Fit,
                        _ => return None,
                    })
                }) {
                    self.alignment = align;
                    true
                } else {
                    false
                }
            }
            73 => {
                if let Some(v) = atom.value.get() {
                    self.alignment = match self.alignment {
                        TextAlignment::Combo(h, _) => TextAlignment::Combo(h, v),
                        _ => TextAlignment::Combo(TextHorizontalAlignment::Left, v),
                    };
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
                self.text.raw += atom.value.get().unwrap_or_default();
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
            72 => atom.value.get_to(&mut self.drawing_direction),
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
