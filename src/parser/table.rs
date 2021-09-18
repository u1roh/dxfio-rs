use super::{FromNode, ParNode};
use crate::*;

impl FromNode for TableNode {
    fn from_node(source: &ParNode) -> Self {
        Self {
            handle: source.atoms.get_or_default(5),
            entries: source.nodes.iter().map(FromNode::from_node).collect(),
        }
    }
}

impl FromNode for TableEntry {
    fn from_node(source: &ParNode) -> Self {
        let handle = source
            .atoms
            .get_or_default(if source.node_type == "DIMSTYLE" {
                105
            } else {
                5
            });
        let name = source.atoms.get_or_default(2);
        let record = match source.node_type {
            // "APPID" => {
            //     unimplemented!()
            // }
            // "BLOCK_RECORD" => {
            //     unimplemented!()
            // }
            "DIMSTYLE" => TableRecord::DimStyle(Box::new(FromNode::from_node(source))),
            "LAYER" => TableRecord::Layer(FromNode::from_node(source)),
            "LTYPE" => TableRecord::LineType(FromNode::from_node(source)),
            // "STYLE" => {
            //     unimplemented!()
            // }
            // "UCS" => {
            //     unimplemented!()
            // }
            // "VIEW" => {
            //     unimplemented!()
            // }
            // "VPORT" => {
            //     unimplemented!()
            // }
            // _ => TableRecord::NotSupported(source.into()),
            _ => unimplemented!(),
        };
        Self {
            handle,
            name,
            record,
        }
    }
}

impl FromNode for DimStyle {
    fn from_node(source: &ParNode) -> Self {
        assert_eq!(source.node_type, "DIMSTYLE");
        let mut dst = DimStyle::default();
        for atom in source.atoms {
            match atom.code {
                3 => atom.get_to(&mut dst.general_dimensioning_suffix),
                4 => atom.get_to(&mut dst.alternate_dimensioning_suffix),
                5 => atom.get_to(&mut dst.arrow_block_name),
                6 => atom.get_to(&mut dst.arrow1_block_name),
                7 => atom.get_to(&mut dst.arrow2_block_name),
                40 => atom.get_to(&mut dst.scale_factor),
                41 => atom.get_to(&mut dst.arrow_size),
                42 => atom.get_to(&mut dst.extension_line_offset),
                43 => atom.get_to(&mut dst.dimension_line_increment),
                44 => atom.get_to(&mut dst.extension_line_extension),
                45 => atom.get_to(&mut dst.rounding_value),
                46 => atom.get_to(&mut dst.dimension_line_extension),
                47 => atom.get_to(&mut dst.plus_tolerance),
                48 => atom.get_to(&mut dst.minus_tolerance),
                140 => atom.get_to(&mut dst.text_height),
                141 => atom.get_to(&mut dst.center_mark_size),
                142 => atom.get_to(&mut dst.tick_size),
                143 => atom.get_to(&mut dst.alternate_unit_scale_factor),
                144 => atom.get_to(&mut dst.linear_measurement_scale_factor),
                145 => atom.get_to(&mut dst.text_vertical_position),
                146 => atom.get_to(&mut dst.tolerance_display_scale_factor),
                147 => atom.get_to(&mut dst.dimension_line_gap),
                148 => atom.get_to(&mut dst.alternate_unit_rounding),
                71 => atom.get_to(&mut dst.tolerance),
                72 => atom.get_to(&mut dst.dimension_limits),
                73 => atom.get_to(&mut dst.text_inside_horizontal),
                74 => atom.get_to(&mut dst.text_outside_horizontal),
                75 => atom.get_bool_to(&mut dst.extension_line1_suppressed),
                76 => atom.get_bool_to(&mut dst.extension_line2_suppressed),
                77 => atom.get_bool_to(&mut dst.text_above_dimension_line),
                _ => {}
            }
        }
        dst
    }
}

impl FromNode for Layer {
    fn from_node(source: &ParNode) -> Self {
        assert_eq!(source.node_type, "LAYER");
        let mut dst = Layer {
            is_plotted: true,
            ..Layer::default()
        };
        for atom in source.atoms {
            match atom.code {
                70 => atom.get_to(&mut dst.flags),
                62 => {
                    // if negative, layer is off
                    dst.color_number = atom.get::<i16>().filter(|&c| c >= 0).map(|c| c as u8)
                }
                6 => dst.line_type = atom.get(),
                290 => dst.is_plotted = atom.get::<i16>().unwrap_or_default() != 0,
                370 => dst.line_weight = atom.get(),
                390 => dst.plot_style_handle = atom.get(),
                347 => dst.material_handle = atom.get(),
                _ => {}
            }
        }
        dst
    }
}

impl FromNode for LineType {
    fn from_node(source: &ParNode) -> Self {
        assert_eq!(source.node_type, "LTYPE");
        let mut dst = LineType::default();
        for atom in source.atoms {
            match atom.code {
                70 => atom.get_to(&mut dst.flags),
                3 => dst.description = atom.value.to_owned(),
                40 => atom.get_to(&mut dst.total_pattern_length),
                49 => {
                    if let Some(len) = atom.get() {
                        dst.pattern_lengths.push(len);
                    }
                }
                _ => {}
            }
        }
        dst
    }
}

// ---------------------------------------------------------
use super::FromNode2;

impl FromNode2 for TableNode {
    fn from_node(source: &Node) -> Self {
        Self {
            handle: source
                .atoms
                .iter()
                .find(|a| a.code == 5)
                .and_then(|a| a.value.get())
                .unwrap_or_default(),
            entries: source.nodes.iter().map(FromNode2::from_node).collect(),
        }
    }
}

impl FromNode2 for TableEntry {
    fn from_node(source: &Node) -> Self {
        let handle = {
            let code = if source.node_type == "DIMSTYLE" {
                105
            } else {
                5
            };
            source
                .atoms
                .iter()
                .find(|a| a.code == code)
                .and_then(|a| a.value.get())
                .unwrap_or_default()
        };
        let name = source
            .atoms
            .iter()
            .find(|a| a.code == 2)
            .and_then(|a| a.value.get())
            .unwrap_or_default();
        let record = match &*source.node_type {
            // "APPID" => {
            //     unimplemented!()
            // }
            // "BLOCK_RECORD" => {
            //     unimplemented!()
            // }
            "DIMSTYLE" => TableRecord::DimStyle(Box::new(FromNode2::from_node(source))),
            "LAYER" => TableRecord::Layer(FromNode2::from_node(source)),
            "LTYPE" => TableRecord::LineType(FromNode2::from_node(source)),
            // "STYLE" => {
            //     unimplemented!()
            // }
            // "UCS" => {
            //     unimplemented!()
            // }
            // "VIEW" => {
            //     unimplemented!()
            // }
            // "VPORT" => {
            //     unimplemented!()
            // }
            _ => TableRecord::NotSupported(source.to_owned()),
        };
        Self {
            handle,
            name,
            record,
        }
    }
}

impl FromNode2 for DimStyle {
    fn from_node(source: &Node) -> Self {
        assert_eq!(source.node_type, "DIMSTYLE");
        let mut dst = DimStyle::default();
        let int2bool = |x: i16| match x {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        };
        for atom in source.atoms.iter() {
            let _ = match atom.code {
                3 => atom.value.get_to(&mut dst.general_dimensioning_suffix),
                4 => atom.value.get_to(&mut dst.alternate_dimensioning_suffix),
                5 => atom.value.get_to(&mut dst.arrow_block_name),
                6 => atom.value.get_to(&mut dst.arrow1_block_name),
                7 => atom.value.get_to(&mut dst.arrow2_block_name),
                40 => atom.value.get_to(&mut dst.scale_factor),
                41 => atom.value.get_to(&mut dst.arrow_size),
                42 => atom.value.get_to(&mut dst.extension_line_offset),
                43 => atom.value.get_to(&mut dst.dimension_line_increment),
                44 => atom.value.get_to(&mut dst.extension_line_extension),
                45 => atom.value.get_to(&mut dst.rounding_value),
                46 => atom.value.get_to(&mut dst.dimension_line_extension),
                47 => atom.value.get_to(&mut dst.plus_tolerance),
                48 => atom.value.get_to(&mut dst.minus_tolerance),
                140 => atom.value.get_to(&mut dst.text_height),
                141 => atom.value.get_to(&mut dst.center_mark_size),
                142 => atom.value.get_to(&mut dst.tick_size),
                143 => atom.value.get_to(&mut dst.alternate_unit_scale_factor),
                144 => atom.value.get_to(&mut dst.linear_measurement_scale_factor),
                145 => atom.value.get_to(&mut dst.text_vertical_position),
                146 => atom.value.get_to(&mut dst.tolerance_display_scale_factor),
                147 => atom.value.get_to(&mut dst.dimension_line_gap),
                148 => atom.value.get_to(&mut dst.alternate_unit_rounding),
                71 => atom.value.get_to(&mut dst.tolerance),
                72 => atom.value.get_to(&mut dst.dimension_limits),
                73 => atom.value.get_to(&mut dst.text_inside_horizontal),
                74 => atom.value.get_to(&mut dst.text_outside_horizontal),
                75 => atom
                    .value
                    .and_then_to(&mut dst.extension_line1_suppressed, int2bool),
                76 => atom
                    .value
                    .and_then_to(&mut dst.extension_line2_suppressed, int2bool),
                77 => atom
                    .value
                    .and_then_to(&mut dst.text_above_dimension_line, int2bool),
                _ => false,
            };
        }
        dst
    }
}

impl FromNode2 for Layer {
    fn from_node(source: &Node) -> Self {
        assert_eq!(source.node_type, "LAYER");
        let mut dst = Layer {
            is_plotted: true,
            ..Layer::default()
        };
        for atom in source.atoms.iter() {
            match atom.code {
                70 => {
                    atom.value
                        .and_then_to(&mut dst.flags, |x: i16| Some(x as _));
                }
                62 => {
                    // if negative, layer is off
                    dst.color_number = atom.value.get::<i16>().filter(|&c| c >= 0).map(|c| c as u8);
                }
                6 => dst.line_type = atom.value.get(),
                290 => dst.is_plotted = atom.value.get::<i16>().unwrap_or_default() != 0,
                370 => dst.line_weight = atom.value.get::<i16>().map(|x| x as u16),
                390 => dst.plot_style_handle = atom.value.get(),
                347 => dst.material_handle = atom.value.get(),
                _ => {}
            }
        }
        dst
    }
}

impl FromNode2 for LineType {
    fn from_node(source: &Node) -> Self {
        assert_eq!(source.node_type, "LTYPE");
        let mut dst = LineType::default();
        for atom in source.atoms.iter() {
            let _ = match atom.code {
                70 => atom
                    .value
                    .and_then_to(&mut dst.flags, |x: i16| Some(x as u16)),
                3 => atom.value.get_to(&mut dst.description),
                40 => atom.value.get_to(&mut dst.total_pattern_length),
                49 => {
                    if let Some(len) = atom.value.get() {
                        dst.pattern_lengths.push(len);
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            };
        }
        dst
    }
}
