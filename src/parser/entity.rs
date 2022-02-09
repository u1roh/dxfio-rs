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
            67 => super::parse_to(&atom.value, &mut self.space),
            8 => super::parse_to(&atom.value, &mut self.layer),
            6 => super::parse_to(&atom.value, &mut self.line_type),
            62 => super::parse_to(&atom.value, &mut self.color_number),
            370 => super::parse_to_option(&atom.value, &mut self.line_weight),
            48 => super::parse_to_option(&atom.value, &mut self.line_type_scale),
            60 => {
                self.is_visible = match atom.value.parse::<i16>() {
                    Ok(0) => true,
                    Ok(1) => false,
                    _ => return false,
                };
                true
            }
            420 => super::parse_to_option(&atom.value, &mut self.color_rgb),
            430 => super::parse_to_option(&atom.value, &mut self.color_name),
            440 => super::parse_to_option(&atom.value, &mut self.transparency),
            284 => super::parse_to_option(&atom.value, &mut self.shadow_mode),
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
            2 => super::parse_to(&atom.value, &mut self.block_name),
            10 => super::parse_to(&atom.value, &mut self.insertion_point[0]),
            20 => super::parse_to(&atom.value, &mut self.insertion_point[1]),
            30 => super::parse_to(&atom.value, &mut self.insertion_point[2]),
            41 => super::parse_to(&atom.value, &mut self.scale_factor[0]),
            42 => super::parse_to(&atom.value, &mut self.scale_factor[1]),
            43 => super::parse_to(&atom.value, &mut self.scale_factor[2]),
            50 => super::parse_to(&atom.value, &mut self.rotation_degree),
            70 => super::parse_to(&atom.value, &mut self.column_count),
            71 => super::parse_to(&atom.value, &mut self.row_count),
            44 => super::parse_to(&atom.value, &mut self.column_spacing),
            45 => super::parse_to(&atom.value, &mut self.row_spacing),
            210 => super::parse_to(&atom.value, &mut self.extrusion_direction[0]),
            220 => super::parse_to(&atom.value, &mut self.extrusion_direction[1]),
            230 => super::parse_to(&atom.value, &mut self.extrusion_direction[2]),
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
            7 => super::parse_to_option(&atom.value, &mut self.style_name),
            10 => super::parse_to(&atom.value, &mut self.point1[0]),
            20 => super::parse_to(&atom.value, &mut self.point1[1]),
            30 => super::parse_to(&atom.value, &mut self.point1[2]),
            11 => super::parse_to(&atom.value, &mut self.point2[0]),
            21 => super::parse_to(&atom.value, &mut self.point2[1]),
            31 => super::parse_to(&atom.value, &mut self.point2[2]),
            39 => super::parse_to_option(&atom.value, &mut self.thickness),
            40 => super::parse_to(&atom.value, &mut self.height),
            41 => super::parse_to_option(&atom.value, &mut self.relative_x_scale_factor),
            50 => super::parse_to_option(&atom.value, &mut self.rotation_degree),
            51 => super::parse_to_option(&atom.value, &mut self.oblique_degree),
            71 => super::parse_to_option(&atom.value, &mut self.mirror_flags),
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
            7 => super::parse_to_option(&atom.value, &mut self.style_name),
            10 => super::parse_to(&atom.value, &mut self.point[0]),
            20 => super::parse_to(&atom.value, &mut self.point[1]),
            30 => super::parse_to(&atom.value, &mut self.point[2]),
            11 => atom.value.get_optional_coord_to(0, &mut self.x_axis),
            21 => atom.value.get_optional_coord_to(1, &mut self.x_axis),
            31 => atom.value.get_optional_coord_to(2, &mut self.x_axis),
            40 => super::parse_to(&atom.value, &mut self.height),
            41 => super::parse_to(&atom.value, &mut self.rectangle_width),
            42 => super::parse_to(&atom.value, &mut self.character_width),
            43 => super::parse_to(&atom.value, &mut self.character_height),
            50 => super::parse_to_option(&atom.value, &mut self.rotation_radian),
            210 => atom
                .value
                .get_optional_coord_to(0, &mut self.extrusion_vector),
            220 => atom
                .value
                .get_optional_coord_to(1, &mut self.extrusion_vector),
            230 => atom
                .value
                .get_optional_coord_to(2, &mut self.extrusion_vector),
            71 => super::parse_to(&atom.value, &mut self.attachment_point),
            72 => super::parse_to(&atom.value, &mut self.drawing_direction),
            73 => super::parse_to(&atom.value, &mut self.line_spacing_style),
            44 => super::parse_to_option(&atom.value, &mut self.line_spacing_factor),
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
            45 => super::parse_to_option(&atom.value, &mut self.fill_box_scale),
            _ => false,
        }
    }
}

impl SetAtom for Box<Dimension> {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        let value = &atom.value;
        match atom.code {
            280 => super::parse_to(&atom.value, &mut self.version),
            2 => super::parse_to(&atom.value, &mut self.block_name),

            10 => super::parse_to(&atom.value, &mut self.definition_point[0]),
            20 => super::parse_to(&atom.value, &mut self.definition_point[1]),
            30 => super::parse_to(&atom.value, &mut self.definition_point[2]),

            11 => super::parse_to(&atom.value, &mut self.text_mid_point[0]),
            21 => super::parse_to(&atom.value, &mut self.text_mid_point[1]),
            31 => super::parse_to(&atom.value, &mut self.text_mid_point[2]),

            70 => {
                let success1 = super::parse_to(&atom.value, &mut self.dimension_type);
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
            71 => super::parse_to(&atom.value, &mut self.attachment_point),
            72 => super::parse_to(&atom.value, &mut self.text_line_spacing_style),
            41 => super::parse_to_option(&atom.value, &mut self.text_line_spacing_factor),
            42 => super::parse_to_option(&atom.value, &mut self.actual_measurement),
            1 => super::parse_to_option(&atom.value, &mut self.text),
            53 => super::parse_to_option(&atom.value, &mut self.text_rotation_angle),
            51 => super::parse_to_option(&atom.value, &mut self.horizontal_direction_angle),

            210 => value.get_optional_coord_to(0, &mut self.extrusion_direction),
            220 => value.get_optional_coord_to(1, &mut self.extrusion_direction),
            230 => value.get_optional_coord_to(2, &mut self.extrusion_direction),

            3 => super::parse_to(value, &mut self.dimension_style),

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

            50 => super::parse_to_option(value, &mut self.rotation_angle),
            52 => super::parse_to_option(value, &mut self.oblique_angle),
            40 => super::parse_to_option(value, &mut self.leader_length),

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
            10 => super::parse_to(&atom.value, &mut self.coord[0]),
            20 => super::parse_to(&atom.value, &mut self.coord[1]),
            30 => super::parse_to(&atom.value, &mut self.coord[2]),
            39 => super::parse_to(value, &mut self.thickness),
            210 => value.get_optional_coord_to(0, &mut self.extrusion_direction),
            220 => value.get_optional_coord_to(1, &mut self.extrusion_direction),
            230 => value.get_optional_coord_to(2, &mut self.extrusion_direction),
            50 => super::parse_to_option(value, &mut self.x_axis_degree),
            _ => false,
        }
    }
}

impl SetAtom for Line {
    fn set_atom(&mut self, atom: &Atom) -> bool {
        let value = &atom.value;
        match atom.code {
            10 => super::parse_to(&atom.value, &mut self.p1[0]),
            20 => super::parse_to(&atom.value, &mut self.p1[1]),
            30 => super::parse_to(&atom.value, &mut self.p1[2]),
            11 => super::parse_to(&atom.value, &mut self.p2[0]),
            21 => super::parse_to(&atom.value, &mut self.p2[1]),
            31 => super::parse_to(&atom.value, &mut self.p2[2]),
            39 => super::parse_to(value, &mut self.thickness),
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
            10 => super::parse_to(value, &mut self.center[0]),
            20 => super::parse_to(value, &mut self.center[1]),
            30 => super::parse_to(value, &mut self.center[2]),
            40 => super::parse_to(value, &mut self.radius),
            39 => super::parse_to(value, &mut self.thickness),
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
                50 => super::parse_to(value, &mut self.start_degree),
                51 => super::parse_to(value, &mut self.end_degree),
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
                super::parse_to(value, &mut self.vertex.coord[0])
            }
            20 => {
                self.flags.y = true;
                super::parse_to(value, &mut self.vertex.coord[1])
            }
            40 => {
                self.flags.start_width = true;
                super::parse_to_option(value, &mut self.vertex.start_width)
            }
            41 => {
                self.flags.end_width = true;
                super::parse_to_option(value, &mut self.vertex.end_width)
            }
            42 => {
                self.flags.bulge = true;
                super::parse_to_option(value, &mut self.vertex.bulge)
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
            43 => super::parse_to_option(value, &mut self.target.constant_width),
            38 => super::parse_to_option(value, &mut self.target.elevation),
            39 => super::parse_to_option(value, &mut self.target.thickness),
            210 => value.get_optional_coord_to(0, &mut self.target.extrusion_direction),
            220 => value.get_optional_coord_to(1, &mut self.target.extrusion_direction),
            230 => value.get_optional_coord_to(2, &mut self.target.extrusion_direction),
            _ => false,
        }
    }
}
