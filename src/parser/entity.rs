use super::{FromNode, SetAtom};
use crate::*;

impl FromNode for EntityNode {
    fn from_node(source: &Node) -> Self {
        match source.node_type.as_ref() {
            "INSERT" => parse_by(source, Entity::Insert),
            "DIMENSION" => parse_by(source, Entity::Dimension),
            "TEXT" => parse_by(source, Entity::Text),
            "MTEXT" => parse_by(source, |mut mtext: MText| {
                mtext.text.parse_and_build_nodes();
                Entity::MText(mtext)
            }),
            "POINT" => parse_by(source, Entity::Point),
            "LINE" => parse_by(source, Entity::Line),
            "CIRCLE" => parse_by(source, Entity::Circle),
            "ARC" => parse_by(source, Entity::Arc),
            "LWPOLYLINE" => parse_by(source, LwPolylineBuilder::into_entity),
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
            370 => atom.value.get_to_option(&mut self.line_weight),
            48 => atom.value.get_to_option(&mut self.line_type_scale),
            60 => {
                self.is_visible = match atom.value.parse::<i16>() {
                    Ok(0) => true,
                    Ok(1) => false,
                    _ => return false,
                };
                true
            }
            420 => atom.value.get_to_option(&mut self.color_rgb),
            430 => atom.value.get_to_option(&mut self.color_name),
            440 => atom.value.get_to_option(&mut self.transparency),
            284 => atom.value.get_to_option(&mut self.shadow_mode),
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

impl SetAtom for Text {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        match atom.code {
            1 => {
                self.text = super::text_format::parse_control_codes(&atom.value);
                true
            }
            7 => atom.value.get_to_option(&mut self.style_name),
            10 => atom.value.get_to(&mut self.point1[0]),
            20 => atom.value.get_to(&mut self.point1[1]),
            30 => atom.value.get_to(&mut self.point1[2]),
            11 => atom.value.get_to(&mut self.point2[0]),
            21 => atom.value.get_to(&mut self.point2[1]),
            31 => atom.value.get_to(&mut self.point2[2]),
            39 => atom.value.get_to_option(&mut self.thickness),
            40 => atom.value.get_to(&mut self.height),
            41 => atom.value.get_to_option(&mut self.relative_x_scale_factor),
            50 => atom.value.get_to_option(&mut self.rotation_degree),
            51 => atom.value.get_to_option(&mut self.oblique_degree),
            71 => atom.value.get_to_option(&mut self.mirror_flags),
            72 => {
                if let Ok(h) = atom.value.parse() {
                    self.alignment = match self.alignment {
                        TextAlignment::Combo(_, v) => TextAlignment::Combo(h, v),
                        _ => TextAlignment::Combo(h, TextVerticalAlignment::Baseline),
                    };
                    true
                } else if let Some(align) = atom.value.parse::<i16>().ok().and_then(|value| {
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
                if let Ok(v) = atom.value.parse() {
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
                self.text.raw += &atom.value;
                true
            }
            7 => atom.value.get_to_option(&mut self.style_name),
            10 => atom.value.get_to(&mut self.point[0]),
            20 => atom.value.get_to(&mut self.point[1]),
            30 => atom.value.get_to(&mut self.point[2]),
            11 => atom.value.get_optional_coord_to(0, &mut self.x_axis),
            21 => atom.value.get_optional_coord_to(1, &mut self.x_axis),
            31 => atom.value.get_optional_coord_to(2, &mut self.x_axis),
            40 => atom.value.get_to(&mut self.height),
            41 => atom.value.get_to(&mut self.rectangle_width),
            42 => atom.value.get_to(&mut self.character_width),
            43 => atom.value.get_to(&mut self.character_height),
            50 => atom.value.get_to_option(&mut self.rotation_radian),
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
            44 => atom.value.get_to_option(&mut self.line_spacing_factor),
            90 => {
                self.background_fill_color =
                    match (self.background_fill_color, atom.value.parse::<i32>()) {
                        (Some(MTextBackground::ColorNumber(_)), Ok(1)) => return true,
                        (_, Ok(0)) => None,
                        (_, Ok(1)) => Some(MTextBackground::ColorNumber(0)),
                        (_, Ok(2)) => Some(MTextBackground::WindowColor),
                        _ => return false,
                    };
                true
            }
            63 => {
                if let Ok(color) = atom.value.parse::<i16>() {
                    self.background_fill_color = Some(MTextBackground::ColorNumber(color));
                    true
                } else {
                    false
                }
            }
            45 => atom.value.get_to_option(&mut self.fill_box_scale),
            _ => false,
        }
    }
}

impl SetAtom for Box<Dimension> {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        let value = &atom.value;
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
                let success2 = if let Ok(flags) = atom.value.parse::<i16>() {
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
            41 => atom.value.get_to_option(&mut self.text_line_spacing_factor),
            42 => atom.value.get_to_option(&mut self.actual_measurement),
            1 => atom.value.get_to_option(&mut self.text),
            53 => atom.value.get_to_option(&mut self.text_rotation_angle),
            51 => atom
                .value
                .get_to_option(&mut self.horizontal_direction_angle),

            210 => value.get_optional_coord_to(0, &mut self.extrusion_direction),
            220 => value.get_optional_coord_to(1, &mut self.extrusion_direction),
            230 => value.get_optional_coord_to(2, &mut self.extrusion_direction),

            3 => value.get_to(&mut self.dimension_style),

            13 => value.get_optional_coord_to(0, &mut self.definition_point2),
            23 => value.get_optional_coord_to(1, &mut self.definition_point2),
            33 => value.get_optional_coord_to(2, &mut self.definition_point2),

            14 => value.get_optional_coord_to(0, &mut self.definition_point3),
            24 => value.get_optional_coord_to(1, &mut self.definition_point3),
            34 => value.get_optional_coord_to(2, &mut self.definition_point3),

            15 => value.get_optional_coord_to(0, &mut self.definition_point4),
            25 => value.get_optional_coord_to(1, &mut self.definition_point4),
            35 => value.get_optional_coord_to(2, &mut self.definition_point4),

            12 => value.get_optional_coord_to(0, &mut self.insertion_point),
            22 => value.get_optional_coord_to(1, &mut self.insertion_point),
            32 => value.get_optional_coord_to(2, &mut self.insertion_point),

            16 => value.get_optional_coord_to(0, &mut self.arc_location),
            26 => value.get_optional_coord_to(1, &mut self.arc_location),
            36 => value.get_optional_coord_to(2, &mut self.arc_location),

            50 => value.get_to_option(&mut self.rotation_angle),
            52 => value.get_to_option(&mut self.oblique_angle),
            40 => value.get_to_option(&mut self.leader_length),

            _ => {
                log::info!("unhandled atom: {:?}", atom);
                false
            }
        }
    }
}

impl SetAtom for Point {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        let value = &atom.value;
        match atom.code {
            10 => atom.value.get_to(&mut self.coord[0]),
            20 => atom.value.get_to(&mut self.coord[1]),
            30 => atom.value.get_to(&mut self.coord[2]),
            39 => value.get_to(&mut self.thickness),
            210 => value.get_optional_coord_to(0, &mut self.extrusion_direction),
            220 => value.get_optional_coord_to(1, &mut self.extrusion_direction),
            230 => value.get_optional_coord_to(2, &mut self.extrusion_direction),
            50 => value.get_to_option(&mut self.x_axis_degree),
            _ => false,
        }
    }
}

impl SetAtom for Line {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        let value = &atom.value;
        match atom.code {
            10 => atom.value.get_to(&mut self.p1[0]),
            20 => atom.value.get_to(&mut self.p1[1]),
            30 => atom.value.get_to(&mut self.p1[2]),
            11 => atom.value.get_to(&mut self.p2[0]),
            21 => atom.value.get_to(&mut self.p2[1]),
            31 => atom.value.get_to(&mut self.p2[2]),
            39 => value.get_to(&mut self.thickness),
            210 => value.get_optional_coord_to(0, &mut self.extrusion_direction),
            220 => value.get_optional_coord_to(1, &mut self.extrusion_direction),
            230 => value.get_optional_coord_to(2, &mut self.extrusion_direction),
            _ => false,
        }
    }
}

impl SetAtom for Circle {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        let value = &atom.value;
        match atom.code {
            10 => value.get_to(&mut self.center[0]),
            20 => value.get_to(&mut self.center[1]),
            30 => value.get_to(&mut self.center[2]),
            40 => value.get_to(&mut self.radius),
            39 => value.get_to(&mut self.thickness),
            210 => value.get_optional_coord_to(0, &mut self.extrusion_direction),
            220 => value.get_optional_coord_to(1, &mut self.extrusion_direction),
            230 => value.get_optional_coord_to(2, &mut self.extrusion_direction),
            _ => false,
        }
    }
}

impl SetAtom for Arc {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        if self.circle.set_atom(atom) {
            true
        } else {
            let value = &atom.value;
            match atom.code {
                50 => value.get_to(&mut self.start_degree),
                51 => value.get_to(&mut self.end_degree),
                _ => false,
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct LwPolylineVertexFlags {
    x: bool,
    y: bool,
    start_width: bool,
    end_width: bool,
    bulge: bool,
}

#[derive(Default)]
struct LwPolylineBuilder {
    target: LwPolyline,
    vertex: LwPolylineVertex,
    flags: LwPolylineVertexFlags,
}
impl LwPolylineBuilder {
    fn push_vertex(&mut self) {
        self.target.vertices.push(std::mem::take(&mut self.vertex));
        self.flags = Default::default();
    }
    fn into_entity(mut self) -> Entity {
        if self.flags != Default::default() {
            self.push_vertex();
        }
        Entity::LwPolyline(self.target)
    }
}
impl SetAtom for LwPolylineBuilder {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        let value = &atom.value;
        match atom.code {
            10 if self.flags.x => self.push_vertex(),
            20 if self.flags.y => self.push_vertex(),
            40 if self.flags.start_width => self.push_vertex(),
            41 if self.flags.end_width => self.push_vertex(),
            42 if self.flags.bulge => self.push_vertex(),
            _ => {}
        }
        match atom.code {
            10 => {
                self.flags.x = true;
                value.get_to(&mut self.vertex.coord[0])
            }
            20 => {
                self.flags.y = true;
                value.get_to(&mut self.vertex.coord[1])
            }
            40 => {
                self.flags.start_width = true;
                value.get_to_option(&mut self.vertex.start_width)
            }
            41 => {
                self.flags.end_width = true;
                value.get_to_option(&mut self.vertex.end_width)
            }
            42 => {
                self.flags.bulge = true;
                value.get_to_option(&mut self.vertex.bulge)
            }

            70 => {
                if let Ok(flags) = value.parse::<i16>() {
                    self.target.is_closed = (flags & 0b00000001) != 0;
                    self.target.is_continuous_pattern = (flags & 0b10000000) != 0;
                    true
                } else {
                    false
                }
            }
            43 => value.get_to_option(&mut self.target.constant_width),
            38 => value.get_to_option(&mut self.target.elevation),
            39 => value.get_to_option(&mut self.target.thickness),
            210 => value.get_optional_coord_to(0, &mut self.target.extrusion_direction),
            220 => value.get_optional_coord_to(1, &mut self.target.extrusion_direction),
            230 => value.get_optional_coord_to(2, &mut self.target.extrusion_direction),
            _ => false,
        }
    }
}
