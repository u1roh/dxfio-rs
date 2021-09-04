#[derive(Debug)]
pub struct Drawing {
    pub entities: Vec<EntityNode>,
}

#[derive(Debug, Default, Clone)]
pub struct EntityHeader {
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

#[derive(Debug, Clone)]
pub struct EntityNode {
    pub header: EntityHeader,
    pub entity: Entity,
}

#[derive(Debug, Clone)]
pub enum Entity {
    Line(Line),
    Unknown {
        node_type: String,
        atoms: Vec<(i16, String)>,
    },
}

#[derive(Debug, Clone)]
pub struct Line {
    pub p1: [f64; 3],
    pub p2: [f64; 3],
}
