use super::DxfNode;

#[derive(Debug, Clone)]
pub struct Drawing<'a> {
    pub entities: Vec<EntityNode<'a>>,
}
impl<'a> Drawing<'a> {
    pub fn parse(nodes: &'a [DxfNode<'a>]) -> Result<Self, Box<dyn std::error::Error>> {
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
                    drawing.entities = section.nodes.iter().map(EntityNode::parse).collect();
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

#[derive(Debug, Clone)]
pub struct SourceAndTarget<'a, T> {
    pub source: &'a DxfNode<'a>,
    pub target: T,
}

pub type EntityNode<'a> = SourceAndTarget<'a, crate::EntityNode>;
impl<'a> EntityNode<'a> {
    pub fn parse(source: &'a DxfNode<'a>) -> Self {
        let target = crate::EntityNode {
            header: Default::default(),
            entity: match source.node_type {
                "LINE" => crate::Entity::Line(crate::Line {
                    p1: source.get_point(0),
                    p2: source.get_point(1),
                }),
                _ => crate::Entity::Unknown {
                    node_type: source.node_type.to_owned(),
                    atoms: source
                        .atoms
                        .iter()
                        .map(|atom| (atom.code, atom.value.to_owned()))
                        .collect(),
                },
            },
        };
        Self { source, target }
    }
}
