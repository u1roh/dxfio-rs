use super::{ParNode, ParTableNode, TableNode};
use crate::*;

impl<'a> ParTableNode<'a> {
    pub fn parse(source: &'a ParNode<'a>) -> Self {
        let handle = source.atoms.get_or_default(5);
        let entries = source
            .nodes
            .iter()
            .map(|node| {
                let handle = node
                    .atoms
                    .get_or_default(if source.node_type == "DIMSTYLE" {
                        105
                    } else {
                        5
                    });
                let name = node.atoms.get_or_default(2);
                let record = match node.node_type {
                    // "APPID" => {
                    //     unimplemented!()
                    // }
                    // "BLOCK_RECORD" => {
                    //     unimplemented!()
                    // }
                    "DIMSTYLE" => {
                        let mut dst = DimStyle::default();
                        for atom in node.atoms {
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
                        TableRecord::DimStyle(dst)
                    }
                    "LAYER" => {
                        let mut dst = Layer {
                            is_plotted: true,
                            ..Layer::default()
                        };
                        for atom in node.atoms {
                            match atom.code {
                                70 => atom.get_to(&mut dst.flags),
                                62 => {
                                    // if negative, layer is off
                                    dst.color_number =
                                        atom.get::<i16>().filter(|&c| c >= 0).map(|c| c as u8)
                                }
                                6 => dst.line_type = atom.get(),
                                290 => dst.is_plotted = atom.get::<i16>().unwrap_or_default() != 0,
                                370 => dst.line_weight = atom.get(),
                                390 => dst.plot_style_handle = atom.get(),
                                347 => dst.material_handle = atom.get(),
                                _ => {}
                            }
                        }
                        TableRecord::Layer(dst)
                    }
                    "LTYPE" => {
                        let mut dst = LineType::default();
                        for atom in node.atoms {
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
                        TableRecord::LineType(dst)
                    }
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
                    _ => TableRecord::Unknown(node.into()),
                };
                TableEntry {
                    handle,
                    name,
                    record,
                }
            })
            .collect();
        Self {
            source,
            target: TableNode { handle, entries },
        }
    }
}
