use super::ParNode;
use crate::*;

#[derive(Debug, Clone)]
pub struct ParDrawing<'a> {
    pub blocks: Vec<ParBlockNode<'a>>,
    pub entities: Vec<ParEntityNode<'a>>,
}
impl<'a> ParDrawing<'a> {
    pub fn parse(nodes: &'a [ParNode<'a>]) -> Self {
        let mut drawing = Self {
            blocks: Vec::new(),
            entities: Vec::new(),
        };
        for section in nodes {
            match section.find(2) {
                Some("HEADER") => {}
                Some("CLASSES") => {}
                Some("TABLES") => {}
                Some("BLOCKS") => {
                    drawing.blocks = section.nodes.iter().map(ParBlockNode::parse).collect();
                }
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
            blocks: drawing.blocks.into_iter().map(|b| b.target).collect(),
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

pub type ParBlockNode<'a> = SourceAndTarget<'a, BlockNode>;
impl<'a> ParBlockNode<'a> {
    pub fn parse(source: &'a ParNode<'a>) -> Self {
        let mut target = BlockNode {
            handle: 0,
            layer: String::default(),
            block_name: String::default(),
            block_flags: BlockFlags::default(),
            base_point: [0.0, 0.0, 0.0],
            xref_path_name: String::default(),
            description: String::default(),
            entities: Vec::new(),
        };
        for atom in source.atoms {
            match atom.code {
                8 => target.layer = atom.value.to_owned(),
                2 | 3 => target.block_name = atom.value.to_owned(),
                70 => {
                    if let Some(flags) = atom.get::<u8>() {
                        target.block_flags = BlockFlags {
                            is_anonymous: (flags & 0b0000_0001) != 0,
                            has_non_constant_attribute_definitions: (flags & 0b0000_0010) != 0,
                            is_xref: (flags & 0b0000_0100) != 0,
                            is_xref_overlay: (flags & 0b0000_1000) != 0,
                            is_externally_dependent: (flags & 0b0001_0000) != 0,
                            is_resolved_xref_or_dependent_of_xref: (flags & 0b0010_0000) != 0,
                            is_referenced_xref: (flags & 0b0100_0000) != 0,
                        };
                    }
                }
                10 => atom.get_to(&mut target.base_point[0]),
                20 => atom.get_to(&mut target.base_point[1]),
                30 => atom.get_to(&mut target.base_point[2]),
                1 => target.xref_path_name = atom.value.to_owned(),
                4 => target.description = atom.value.to_owned(),
                _ => {}
            }
        }
        target.entities = source
            .nodes
            .iter()
            .map(ParEntityNode::parse)
            .map(|e| e.target)
            .collect();
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
                header.color_number = match atom.get().unwrap_or_default() {
                    0 => ColorNumber::ByBlock,
                    256 => ColorNumber::ByLayer,
                    257 => ColorNumber::ByEntity,
                    i if i < 0 => ColorNumber::TurnedOff,
                    i if i < 256 => ColorNumber::Number(i as u8),
                    i => panic!("invalid color number: {}", i),
                };
            }
            370 => header.lineweight = atom.get(),
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
