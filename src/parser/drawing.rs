use super::DxfNode;
use crate::*;

#[derive(Debug, Clone)]
pub struct ParDrawing<'a> {
    pub entities: Vec<ParEntityNode<'a>>,
}
impl<'a> ParDrawing<'a> {
    pub fn parse(nodes: &'a [DxfNode<'a>]) -> Self {
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
            entities: drawing.entities.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SourceAndTarget<'a, T> {
    pub source: &'a DxfNode<'a>,
    pub target: T,
}

pub type ParEntityNode<'a> = SourceAndTarget<'a, EntityNode>;
impl<'a> From<ParEntityNode<'a>> for EntityNode {
    fn from(x: ParEntityNode<'a>) -> Self {
        x.target
    }
}
impl<'a> ParEntityNode<'a> {
    pub fn parse(source: &'a DxfNode<'a>) -> Self {
        let target = EntityNode {
            header: Default::default(),
            entity: match source.node_type {
                "LINE" => Entity::Line(Line {
                    p1: source.get_point(0),
                    p2: source.get_point(1),
                }),
                _ => Entity::Unknown {
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
