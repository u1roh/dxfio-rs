use super::ParNode;
use crate::*;

impl super::ParseFromNode for BlockNode {
    fn parse_from_node(source: &ParNode) -> Self {
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
            .map(super::SourceAndTarget::parse_from_node)
            .map(|e| e.target)
            .collect();
        target
    }
}