use crate::*;

use super::FromNode;

impl FromNode for TableNode {
    fn from_node(source: &Node) -> Self {
        Self {
            handle: source
                .atoms
                .iter()
                .find(|a| a.code == 5)
                .and_then(|a| u32::from_str_radix(&a.value, 16).ok())
                .unwrap_or_default(),
            entries: source.nodes.iter().map(FromNode::from_node).collect(),
        }
    }
}

impl FromNode for TableEntry {
    fn from_node(source: &Node) -> Self {
        let handle = {
            let code = if source.node_type == "DIMSTYLE" {
                105
            } else {
                5
            };
            source
                .atoms
                .find(code)
                .and_then(|s| u32::from_str_radix(s, 16).ok())
                .unwrap_or_default()
        };
        let name = source
            .atoms
            .iter()
            .find(|a| a.code == 2)
            .map(|a| a.value.to_string())
            .unwrap_or_default();
        let record = match &*source.node_type {
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
            _ => TableRecord::NotSupported(source.to_owned()),
        };
        Self {
            handle,
            name,
            record,
        }
    }
}

impl FromNode for DimStyle {
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
                3 => super::parse_to(&atom.value, &mut dst.general_dimensioning_suffix),
                4 => super::parse_to(&atom.value, &mut dst.alternate_dimensioning_suffix),
                5 => super::parse_to(&atom.value, &mut dst.arrow_block_name),
                6 => super::parse_to(&atom.value, &mut dst.arrow1_block_name),
                7 => super::parse_to(&atom.value, &mut dst.arrow2_block_name),
                40 => super::parse_to(&atom.value, &mut dst.scale_factor),
                41 => super::parse_to(&atom.value, &mut dst.arrow_size),
                42 => super::parse_to(&atom.value, &mut dst.extension_line_offset),
                43 => super::parse_to(&atom.value, &mut dst.dimension_line_increment),
                44 => super::parse_to(&atom.value, &mut dst.extension_line_extension),
                45 => super::parse_to(&atom.value, &mut dst.rounding_value),
                46 => super::parse_to(&atom.value, &mut dst.dimension_line_extension),
                47 => super::parse_to(&atom.value, &mut dst.plus_tolerance),
                48 => super::parse_to(&atom.value, &mut dst.minus_tolerance),
                140 => super::parse_to(&atom.value, &mut dst.text_height),
                141 => super::parse_to(&atom.value, &mut dst.center_mark_size),
                142 => super::parse_to(&atom.value, &mut dst.tick_size),
                143 => super::parse_to(&atom.value, &mut dst.alternate_unit_scale_factor),
                144 => super::parse_to(&atom.value, &mut dst.linear_measurement_scale_factor),
                145 => super::parse_to(&atom.value, &mut dst.text_vertical_position),
                146 => super::parse_to(&atom.value, &mut dst.tolerance_display_scale_factor),
                147 => super::parse_to(&atom.value, &mut dst.dimension_line_gap),
                148 => super::parse_to(&atom.value, &mut dst.alternate_unit_rounding),
                71 => super::parse_to(&atom.value, &mut dst.tolerance),
                72 => super::parse_to(&atom.value, &mut dst.dimension_limits),
                73 => super::parse_to(&atom.value, &mut dst.text_inside_horizontal),
                74 => super::parse_to(&atom.value, &mut dst.text_outside_horizontal),
                75 => super::parse_and_then_to(
                    &atom.value,
                    &mut dst.extension_line1_suppressed,
                    int2bool,
                ),
                76 => super::parse_and_then_to(
                    &atom.value,
                    &mut dst.extension_line2_suppressed,
                    int2bool,
                ),
                77 => super::parse_and_then_to(
                    &atom.value,
                    &mut dst.text_above_dimension_line,
                    int2bool,
                ),
                _ => false,
            };
        }
        dst
    }
}

impl FromNode for Layer {
    fn from_node(source: &Node) -> Self {
        assert_eq!(source.node_type, "LAYER");
        let mut dst = Layer {
            is_plotted: true,
            ..Layer::default()
        };
        for atom in source.atoms.iter() {
            match atom.code {
                70 => {
                    super::parse_and_then_to(&atom.value, &mut dst.flags, |x: i16| Some(x as _));
                }
                62 => {
                    // if negative, layer is off
                    dst.color_number = atom
                        .value
                        .parse::<i16>()
                        .ok()
                        .filter(|&c| c >= 0)
                        .map(|c| c as u8);
                }
                6 => dst.line_type = atom.value.parse().ok(),
                290 => dst.is_plotted = atom.value.parse::<i16>().unwrap_or_default() != 0,
                370 => dst.line_weight = atom.value.parse::<i16>().ok(),
                390 => dst.plot_style_handle = u32::from_str_radix(&atom.value, 16).ok(),
                347 => dst.material_handle = u32::from_str_radix(&atom.value, 16).ok(),
                _ => {}
            }
        }
        dst
    }
}

impl FromNode for LineType {
    fn from_node(source: &Node) -> Self {
        assert_eq!(source.node_type, "LTYPE");
        let mut dst = LineType::default();
        for atom in source.atoms.iter() {
            let _ = match atom.code {
                70 => {
                    super::parse_and_then_to(&atom.value, &mut dst.flags, |x: i16| Some(x as u16))
                }
                3 => super::parse_to(&atom.value, &mut dst.description),
                40 => super::parse_to(&atom.value, &mut dst.total_pattern_length),
                49 => {
                    if let Ok(len) = atom.value.parse() {
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
