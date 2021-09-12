use super::ParNode;
use super::{ParBlockNode, ParEntityNode, ParTableNode};
use crate::*;

#[derive(Debug, Clone)]
pub struct ParDrawing<'a> {
    pub headers: Vec<ParNode<'a>>,
    pub tables: Vec<ParTableNode<'a>>,
    pub blocks: Vec<ParBlockNode<'a>>,
    pub entities: Vec<ParEntityNode<'a>>,
}
impl<'a> ParDrawing<'a> {
    pub fn parse(nodes: &'a [ParNode<'a>]) -> Self {
        let mut drawing = Self {
            headers: Vec::new(),
            tables: Vec::new(),
            blocks: Vec::new(),
            entities: Vec::new(),
        };
        for section in nodes {
            match section.atoms.find(2) {
                Some("HEADER") => {
                    drawing.headers = section.nodes.clone();
                }
                Some("CLASSES") => {}
                Some("TABLES") => {
                    drawing.tables = section.nodes.iter().map(ParTableNode::parse).collect();
                }
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
            headers: drawing.headers.into_iter().map(Into::into).collect(),
            tables: drawing.tables.into_iter().map(|b| b.target).collect(),
            blocks: drawing.blocks.into_iter().map(|b| b.target).collect(),
            entities: drawing.entities.into_iter().map(|e| e.target).collect(),
        }
    }
}
