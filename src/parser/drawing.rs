use super::SourceAndTarget2;
use crate::*;

#[derive(Debug, Clone)]
pub struct ParDrawing<'a> {
    pub headers: &'a [Node<'a>],
    pub tables: Vec<SourceAndTarget2<'a, TableNode>>,
    pub blocks: Vec<SourceAndTarget2<'a, BlockNode>>,
    pub entities: Vec<SourceAndTarget2<'a, EntityNode>>,
}
impl<'a> ParDrawing<'a> {
    pub fn parse(nodes: &'a [Node<'a>]) -> Self {
        let mut drawing = Self {
            headers: &[],
            tables: Vec::new(),
            blocks: Vec::new(),
            entities: Vec::new(),
        };
        for section in nodes {
            match section
                .atoms
                .iter()
                .find(|a| a.code == 2)
                .and_then(|a| a.value.get())
            {
                Some("HEADER") => {
                    drawing.headers = &section.nodes;
                }
                Some("CLASSES") => {}
                Some("TABLES") => {
                    drawing.tables = section
                        .nodes
                        .iter()
                        .map(SourceAndTarget2::from_node)
                        .collect();
                }
                Some("BLOCKS") => {
                    drawing.blocks = section
                        .nodes
                        .iter()
                        .map(SourceAndTarget2::from_node)
                        .collect();
                }
                Some("ENTITIES") => {
                    drawing.entities = section
                        .nodes
                        .iter()
                        .map(SourceAndTarget2::from_node)
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
            headers: drawing.headers.into_iter().map(Node::to_owned).collect(),
            tables: drawing.tables.into_iter().map(|b| b.target).collect(),
            blocks: drawing.blocks.into_iter().map(|b| b.target).collect(),
            entities: drawing.entities.into_iter().map(|e| e.target).collect(),
        }
    }
}
