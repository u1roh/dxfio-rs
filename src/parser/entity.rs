use super::{EntityNode, ParEntityNode, ParNode};
use crate::*;

impl<'a> ParEntityNode<'a> {
    pub(super) fn parse(source: &'a ParNode<'a>) -> Self {
        let target = EntityNode {
            header: parse_entity_header(source),
            entity: match source.node_type {
                "LINE" => Entity::Line(Line {
                    p1: source.atoms.get_point(0),
                    p2: source.atoms.get_point(1),
                }),
                "INSERT" => Entity::Insert({
                    let mut insert = Insert::new(source.atoms.get_or_default(2));
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
                }),
                _ => Entity::Unknown(source.into()),
            },
        };
        Self { source, target }
    }
}

fn parse_entity_header(source: &ParNode) -> EntityHeader {
    let mut header = EntityHeader {
        handle: 0,                          // 5    String
        space: Space::ModelSpace,           // 67   i16     ModelSpace
        layer: String::default(),           // 8    String
        line_type: LineTypeName::ByLayer,   // 6    String  ByLayer
        color_number: ColorNumber::ByLayer, // 62   i16     ByLayer
        line_weight: None,                  // 370  i16
        line_type_scale: None,              // 48   f64
        is_visible: true,                   // 60   i16     true
        color_rgb: None,                    // 420  i32
        color_name: None,                   // 430  String
        transparency: None,                 // 440  i32
        shadow_mode: None,                  // 284  i16
        extras: vec![],
    };
    // read atoms until subclass marker (group code 100)
    for atom in source.atoms.iter().take_while(|a| a.code != 100) {
        match atom.code {
            5 => header.handle = u32::from_str_radix(atom.value, 16).unwrap_or_default(),
            67 => {
                header.space = match atom.value {
                    "0" => Space::ModelSpace,
                    "1" => Space::PaperSpace,
                    _ => {
                        log::error!("unknown space: {:?}", atom);
                        Space::ModelSpace // fallback
                    }
                };
            }
            8 => header.layer = atom.value.to_owned(),
            6 => {
                header.line_type = match atom.value {
                    "BYLAYER" => LineTypeName::ByLayer,
                    "BYBLOCK" => LineTypeName::ByBlock,
                    "CONTINUOUS" => LineTypeName::Continuous,
                    "DASHED" => LineTypeName::Dashed,
                    _ => LineTypeName::Other(atom.value.to_owned()),
                };
            }
            62 => {
                header.color_number = match atom.get().unwrap_or_default() {
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
            370 => header.line_weight = atom.get(),
            48 => header.line_type_scale = atom.get(),
            60 => header.is_visible = atom.value == "0",
            420 => {
                header.color_rgb = atom.get().map(|bits: u32| Rgb {
                    r: ((bits & 0xff0000) >> 16) as u8,
                    g: ((bits & 0x00ff00) >> 8) as u8,
                    b: (bits & 0x0000ff) as u8,
                });
            }
            430 => header.color_name = Some(atom.value.to_owned()),
            440 => header.transparency = atom.get(),
            284 => {
                header.shadow_mode = atom.get().map(|mode: i16| match mode {
                    0 => ShadowMode::CastsAndReceivesShadows,
                    1 => ShadowMode::CastsShadows,
                    2 => ShadowMode::ReceivesShadows,
                    _ => ShadowMode::IgnoresShadows,
                })
            }
            _ => header.extras.push((*atom).into()),
        }
    }
    header
}
