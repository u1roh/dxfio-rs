use bare_dxf::parser::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("args = {:?}", args);
    // let s = std::fs::read_to_string(&args[1])?;
    let bytes = std::fs::read(&args[1])?;
    let (s, _, _) = encoding_rs::SHIFT_JIS.decode(&bytes);
    // println!("s = {}", s);
    let atoms = DxfAtom::parse(&s)?;
    // println!("pairs = {:?}", pairs);
    let nodes = DxfNode::parse(&atoms);
    println!("nodes.len() = {}", nodes.len());
    for node in &nodes {
        node.print(0);
    }
    let drawing = Drawing::parse(&nodes)?;
    println!("{:?}", drawing);
    Ok(())
}

#[derive(Debug)]
struct Drawing<'a> {
    entities: Vec<Entity<'a>>,
}
impl<'a> Drawing<'a> {
    fn parse(nodes: &'a [DxfNode<'a>]) -> Result<Self, Box<dyn std::error::Error>> {
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
                    for entity in &section.nodes {
                        drawing.entities.push(Entity {
                            node: entity,
                            head: Default::default(),
                            body: match entity.node_type {
                                "LINE" => Some(EntityBody::Line(Line {
                                    p1: [
                                        entity.get_or_default(10),
                                        entity.get_or_default(20),
                                        entity.get_or_default(30),
                                    ],
                                    p2: [
                                        entity.get_or_default(11),
                                        entity.get_or_default(21),
                                        entity.get_or_default(31),
                                    ],
                                })),
                                _ => None,
                            },
                        });
                    }
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
        Ok(drawing)
    }
}

#[derive(Debug, Default)]
struct EntityHeader {
    pub handle: u32,
    pub is_in_paper_space: bool,
    pub layer: String,
    pub line_type_name: String,
    pub elevation: f64,
    pub lineweight_enum_value: i16,
    pub line_type_scale: f64,
    pub is_visible: bool,
    pub image_byte_count: i32,
    pub preview_image_data: Vec<Vec<u8>>,
    pub color_24_bit: i32,
    pub color_name: String,
    pub transparency: i32,
}

#[derive(Debug)]
struct Entity<'a> {
    node: &'a DxfNode<'a>,
    head: EntityHeader,
    body: Option<EntityBody>,
}

#[derive(Debug)]
enum EntityBody {
    Line(Line),
}

#[derive(Debug)]
struct Line {
    pub p1: [f64; 3],
    pub p2: [f64; 3],
}
