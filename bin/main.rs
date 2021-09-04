fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("args = {:?}", args);
    // let s = std::fs::read_to_string(&args[1])?;
    let bytes = std::fs::read(&args[1])?;
    let (s, _, _) = encoding_rs::SHIFT_JIS.decode(&bytes);
    // println!("s = {}", s);
    let pairs = CodeValue::parse(&s)?;
    // println!("pairs = {:?}", pairs);
    let nodes = Node::parse(&pairs);
    println!("nodes.len() = {}", nodes.len());
    for node in &nodes {
        node.print(0);
    }
    let drawing = Drawing::parse(&nodes)?;
    println!("{:?}", drawing);
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct CodeValue<'a> {
    pub code: i16,
    pub value: &'a str,
}
impl<'a> CodeValue<'a> {
    fn parse(s: &str) -> Result<Vec<CodeValue>, std::num::ParseIntError> {
        s.lines()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| {
                let value = chunk[1];
                chunk[0]
                    .trim()
                    .parse::<i16>()
                    .map(|code| CodeValue { code, value })
            })
            .collect()
    }
}

#[derive(Debug)]
struct Node<'a> {
    pub node_type: &'a str,
    pub code_values: &'a [CodeValue<'a>],
    pub nodes: Vec<Self>,
}
impl<'a> Node<'a> {
    fn print(&self, indent: usize) {
        for _ in 0..indent {
            print!("  ");
        }
        println!(
            "{}: code_values.len() = {}",
            self.node_type,
            self.code_values.len()
        );
        for node in &self.nodes {
            node.print(indent + 1);
        }
    }
    pub fn find(&self, code: i16) -> Option<&str> {
        self.code_values
            .iter()
            .find(|item| item.code == code)
            .map(|item| item.value)
    }
    pub fn get<T: std::str::FromStr>(&self, code: i16) -> Option<T> {
        self.find(code)?.parse().ok()
    }
    pub fn get_or_default<T: std::str::FromStr + Default>(&self, code: i16) -> T {
        self.get(code).unwrap_or_default()
    }
    pub fn parse(code_values: &'a [CodeValue<'a>]) -> Vec<Self> {
        Self::parse_nodes(code_values, 0).unwrap_or_default().0
    }
    fn parse_node(code_values: &'a [CodeValue<'a>], index: usize) -> Option<(Self, usize)> {
        (index..code_values.len())
            .find(|i| code_values[*i].code == 0)
            .and_then(|start| {
                let node_type = code_values[start].value;
                if Self::CONTAINER_TYPES.contains(&node_type) {
                    Self::parse_nodes(code_values, start + 1).map(|(nodes, end)| {
                        let node = Self {
                            node_type,
                            code_values: &code_values[start + 1..end],
                            nodes,
                        };
                        (node, end)
                    })
                } else {
                    Self::parse_entity(node_type, code_values, start + 1)
                }
            })
    }
    fn parse_nodes(
        code_values: &'a [CodeValue<'a>],
        mut index: usize,
    ) -> Option<(Vec<Self>, usize)> {
        let mut nodes = vec![];
        while let Some((node, i)) = Self::parse_node(code_values, index) {
            if node.node_type.contains("END") {
                return Some((nodes, i));
            }
            index = i;
            nodes.push(node);
        }
        if code_values[index].value == "EOF" {
            Some((nodes, index))
        } else {
            None
        }
    }
    fn parse_entity(
        node_type: &'a str,
        code_values: &'a [CodeValue<'a>],
        start: usize,
    ) -> Option<(Self, usize)> {
        (start..code_values.len())
            .find(|i| code_values[*i].code == 0)
            .map(|end| {
                let entity = Self {
                    node_type,
                    code_values: &code_values[start..end],
                    nodes: vec![],
                };
                (entity, end)
            })
    }
    const CONTAINER_TYPES: &'static [&'static str] = &["SECTION", "BLOCK", "TABLE", "POLYLINE"];
}

#[derive(Debug)]
struct Drawing<'a> {
    entities: Vec<Entity<'a>>,
}
impl<'a> Drawing<'a> {
    fn parse(nodes: &'a [Node<'a>]) -> Result<Self, Box<dyn std::error::Error>> {
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
    node: &'a Node<'a>,
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
