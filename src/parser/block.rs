use crate::*;

impl super::FromNode for BlockNode {
    fn from_node(source: &Node) -> Self {
        let mut target = Self {
            handle: 0,
            layer: String::default(),
            block_name: String::default(),
            block_flags: BlockFlags::default(),
            base_point: [0.0, 0.0, 0.0],
            xref_path_name: String::default(),
            description: String::default(),
            entities: Vec::new(),
        };
        for atom in source.atoms.iter() {
            let _ = match atom.code {
                8 => super::parse_to(&atom.value, &mut target.layer),
                2 | 3 => super::parse_to(&atom.value, &mut target.block_name),
                70 => super::parse_to(&atom.value, &mut target.block_flags),
                10 => super::parse_to(&atom.value, &mut target.base_point[0]),
                20 => super::parse_to(&atom.value, &mut target.base_point[1]),
                30 => super::parse_to(&atom.value, &mut target.base_point[2]),
                1 => super::parse_to(&atom.value, &mut target.xref_path_name),
                4 => super::parse_to(&atom.value, &mut target.description),
                _ => false,
            };
        }
        target.entities = source
            .nodes
            .iter()
            .map(super::FromNode::from_node)
            .collect();
        target
    }
}

impl std::str::FromStr for BlockFlags {
    type Err = <i16 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i16>().map(|flags| Self {
            is_anonymous: (flags & 0b0000_0001) != 0,
            has_non_constant_attribute_definitions: (flags & 0b0000_0010) != 0,
            is_xref: (flags & 0b0000_0100) != 0,
            is_xref_overlay: (flags & 0b0000_1000) != 0,
            is_externally_dependent: (flags & 0b0001_0000) != 0,
            is_resolved_xref_or_dependent_of_xref: (flags & 0b0010_0000) != 0,
            is_referenced_xref: (flags & 0b0100_0000) != 0,
        })
    }
}
