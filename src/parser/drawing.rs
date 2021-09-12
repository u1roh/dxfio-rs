use super::{ParNode, SourceAndTarget};
use crate::*;

#[derive(Debug, Clone)]
pub struct ParDrawing<'a> {
    pub headers: &'a [ParNode<'a>],
    pub tables: Vec<SourceAndTarget<'a, TableNode>>,
    pub blocks: Vec<SourceAndTarget<'a, BlockNode>>,
    pub entities: Vec<SourceAndTarget<'a, EntityNode>>,
}
impl<'a> ParDrawing<'a> {
    pub fn parse(nodes: &'a [ParNode<'a>]) -> Self {
        let mut drawing = Self {
            headers: &[],
            tables: Vec::new(),
            blocks: Vec::new(),
            entities: Vec::new(),
        };
        for section in nodes {
            match section.atoms.find(2) {
                Some("HEADER") => {
                    drawing.headers = &section.nodes;
                }
                Some("CLASSES") => {}
                Some("TABLES") => {
                    drawing.tables = section
                        .nodes
                        .iter()
                        .map(SourceAndTarget::from_node)
                        .collect();
                }
                Some("BLOCKS") => {
                    drawing.blocks = section
                        .nodes
                        .iter()
                        .map(SourceAndTarget::from_node)
                        .collect();
                }
                Some("ENTITIES") => {
                    drawing.entities = section
                        .nodes
                        .iter()
                        .map(SourceAndTarget::from_node)
                        .collect();
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
            headers: drawing.headers.iter().map(Into::into).collect(),
            tables: drawing.tables.into_iter().map(|b| b.target).collect(),
            blocks: drawing.blocks.into_iter().map(|b| b.target).collect(),
            entities: drawing.entities.into_iter().map(|e| e.target).collect(),
        }
    }
}
