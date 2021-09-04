use super::ParNode;
use crate::*;

#[derive(Debug, Clone)]
pub struct ParDrawing<'a> {
    pub entities: Vec<ParEntityNode<'a>>,
}
impl<'a> ParDrawing<'a> {
    pub fn parse(nodes: &'a [ParNode<'a>]) -> Self {
        let mut drawing = Self {
            entities: Vec::new(),
        };
        for section in nodes {
            match section.find(2) {
                Some("HEADER") => {}
                Some("CLASSES") => {}
                Some("TABLES") => {}
                Some("BLOCKS") => {}
                Some("ENTITIES") => {
                    drawing.entities = section.nodes.iter().map(ParEntityNode::parse).collect();
                }
                Some("OBJECTS") => {}
                Some(unknown) => {
                    println!("unknown section: {}", unknown);
                }
                None => {
                    println!("section type not found");
                }
            }
        }
        drawing
    }
}
impl<'a> From<ParDrawing<'a>> for Drawing {
    fn from(drawing: ParDrawing<'a>) -> Self {
        Self {
            entities: drawing.entities.into_iter().map(|e| e.target).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SourceAndTarget<'a, T> {
    pub source: &'a ParNode<'a>,
    pub target: T,
}

pub type ParEntityNode<'a> = SourceAndTarget<'a, EntityNode>;
impl<'a> ParEntityNode<'a> {
    pub fn parse(source: &'a ParNode<'a>) -> Self {
        let target = EntityNode {
            header: parse_entity_header(source),
            entity: match source.node_type {
                "LINE" => Entity::Line(Line {
                    p1: source.get_point(0),
                    p2: source.get_point(1),
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
        line_type: LineType::ByLayer,       // 6    String  ByLayer
        color_number: ColorNumber::ByLayer, // 62   i16     ByLayer
        lineweight: None,                   // 370  i16
        line_type_scale: None,              // 48   f64
        is_visible: true,                   // 60   i16     true
        color_rgb: None,                    // 420  i32
        color_name: None,                   // 430  String
        transparency: None,                 // 440  i32
        shadow_mode: None,                  // 284  i16
    };
    for atom in source.atoms {
        match atom.code {
            5 => header.handle = u32::from_str_radix(atom.value, 16).unwrap(),
            67 => {
                header.space = match atom.value {
                    "0" => Space::ModelSpace,
                    "1" => Space::PaperSpace,
                    _ => panic!("unknown space: {:?}", atom),
                };
            }
            8 => header.layer = atom.value.to_owned(),
            6 => {
                header.line_type = match atom.value {
                    "BYLAYER" => LineType::ByLayer,
                    "BYBLOCK" => LineType::ByBlock,
                    "CONTINUOUS" => LineType::Continuous,
                    "DASHED" => LineType::Dashed,
                    _ => LineType::Other(atom.value.to_owned()),
                };
            }
            62 => {
                header.color_number = match atom.value.parse::<i16>().unwrap() {
                    0 => ColorNumber::ByBlock,
                    256 => ColorNumber::ByLayer,
                    257 => ColorNumber::ByEntity,
                    i if i < 0 => ColorNumber::TurnedOff,
                    i if i < 256 => ColorNumber::Number(i as u8),
                    i => panic!("invalid color number: {}", i),
                };
            }
            370 => header.lineweight = atom.value.parse().ok(),
            48 => header.line_type_scale = atom.value.parse().ok(),
            60 => header.is_visible = atom.value == "0",
            420 => {
                header.color_rgb = atom.value.parse::<u32>().ok().map(|bits| Rgb {
                    r: ((bits & 0xff0000) >> 16) as u8,
                    g: ((bits & 0x00ff00) >> 8) as u8,
                    b: (bits & 0x0000ff) as u8,
                });
            }
            430 => header.color_name = Some(atom.value.to_owned()),
            440 => header.transparency = atom.value.parse().ok(),
            284 => {
                header.shadow_mode = atom.value.parse::<i16>().ok().map(|mode| match mode {
                    0 => ShadowMode::CastsAndReceivesShadows,
                    1 => ShadowMode::CastsShadows,
                    2 => ShadowMode::ReceivesShadows,
                    _ => ShadowMode::IgnoresShadows,
                })
            }
            _ => {}
        }
    }
    header
}
