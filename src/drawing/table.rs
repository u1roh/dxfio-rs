#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableNode {
    pub handle: u32,
    pub entries: Vec<TableEntry>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableEntry {
    pub handle: u32,
    pub name: String,
    pub record: TableRecord,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TableRecord {
    RegApp(RegApp),          // APPID
    Block(Block),            // BLOCK_RECORD
    DimStyle(Box<DimStyle>), // DIMSTYLE
    Layer(Layer),            // LAYER
    LineType(LineType),      // LTYPE
    TextStyle(TextStyle),    // STYLE
    Ucs(Ucs),                // UCS
    View(View),              // VIEW
    Viewport(Viewport),      // VPORT
    NotSupported(crate::DxfNode),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegApp {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Block {}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DimStyle {
    pub general_dimensioning_suffix: String,   // 3 DIMPOST
    pub alternate_dimensioning_suffix: String, // 4 DIMAPOST
    pub arrow_block_name: String,              // 5 DIMBLK (obsolete, now object ID)
    pub arrow1_block_name: String,             // 6 DIMBLK1 (obsolete, now object ID)
    pub arrow2_block_name: String,             // 7 DIMBLK2 (obsolete, now object ID)
    pub scale_factor: f64,                     // 40 DIMSCALE
    pub arrow_size: f64,                       // 41 DIMASZ
    pub extension_line_offset: f64,            // 42 DIMEXO
    pub dimension_line_increment: f64,         // 43 DIMDLI
    pub extension_line_extension: f64,         // 44 DIMEXE
    pub rounding_value: f64,                   // 45 DIMRND
    pub dimension_line_extension: f64,         // 46 DIMDLE
    pub plus_tolerance: f64,                   // 47 DIMTP
    pub minus_tolerance: f64,                  // 48 DIMTM
    pub text_height: f64,                      // 140 DIMTXT
    pub center_mark_size: f64,                 // 141 DIMCEN
    pub tick_size: f64,                        // 142 DIMTSZ
    pub alternate_unit_scale_factor: f64,      // 143 DIMALTF
    pub linear_measurement_scale_factor: f64,  // 144 DIMLFAC
    pub text_vertical_position: f64,           // 145 DIMTVP
    pub tolerance_display_scale_factor: f64,   // 146 DIMTFAC
    pub dimension_line_gap: f64,               // 147 DIMGAP
    pub alternate_unit_rounding: f64,          // 148 DIMALTRND
    pub tolerance: i16,                        // 71 DIMTOL
    pub dimension_limits: i16,                 // 72 DIMLIM
    pub text_inside_horizontal: i16,           // 73 DIMTIH
    pub text_outside_horizontal: i16,          // 74 DIMTOH
    pub extension_line1_suppressed: bool,      // 75 DIMSE1
    pub extension_line2_suppressed: bool,      // 76 DIMSE2
    pub text_above_dimension_line: bool,       // 77 DIMTAD
                                               // pub _: i16,    // 78 DIMZIN
                                               // pub _: i16,    // 79 DIMAZIN
                                               // pub _: i16,    // 170 DIMALT
                                               // pub _: i16,    // 171 DIMALTD
                                               // pub _: i16,    // 172 DIMTOFL
                                               // pub _: i16,    // 173 DIMSAH
                                               // pub _: i16,    // 174 DIMTIX
                                               // pub _: i16,    // 175 DIMSOXD
                                               // pub _: i16,    // 176 DIMCLRD
                                               // pub _: i16,    // 177 DIMCLRE
                                               // pub _: i16,    // 178 DIMCLRT
                                               // pub _: i16,    // 179 DIMADEC
                                               // pub _: i16,       // 270 DIMUNIT (obsolete, now use DIMLUNIT AND DIMFRAC)
                                               // pub _: i16,       // 271 DIMDEC
                                               // pub _: i16,       // 272 DIMTDEC
                                               // pub _: i16,       // 273 DIMALTU
                                               // pub _: i16,       // 274 DIMALTTD
                                               // pub _: i16,       // 275 DIMAUNIT
                                               // pub _: i16,       // 276 DIMFRAC
                                               // pub _: i16,       // 277 DIMLUNIT
                                               // pub _: i16,       // 278 DIMDSEP
                                               // pub _: i16,       // 279 DIMTMOVE
                                               // pub _: i16,       // 280 DIMJUST
                                               // pub _: i16,       // 281 DIMSD1
                                               // pub _: i16,       // 282 DIMSD2
                                               // pub _: i16,       // 283 DIMTOLJ
                                               // pub _: i16,       // 284 DIMTZIN
                                               // pub _: i16,       // 285 DIMALTZ
                                               // pub _: i16,       // 286 DIMALTTZ
                                               // pub _: i16,       // 287 DIMFIT (obsolete, now use DIMATFIT and DIMTMOVE)
                                               // pub _: i16,       // 288 DIMUPT
                                               // pub _: i16,       // 289 DIMATFIT
                                               // pub _handle: u32, // 340 DIMTXSTY (handle of referenced STYLE)
                                               // pub _handle: u32, // 341 DIMLDRBLK (handle of referenced BLOCK)
                                               // pub _handle: u32, // 342 DIMBLK (handle of referenced BLOCK)
                                               // pub _handle: u32, // 343 DIMBLK1 (handle of referenced BLOCK)
                                               // pub _handle: u32, // 344 DIMBLK2 (handle of referenced BLOCK)
                                               // pub _: i16,       // 371 DIMLWD (lineweight enum value)
                                               // pub _: i16,       // 372 DIMLWE (lineweight enum value)
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Layer {
    pub is_plotted: bool,
    pub flags: u16,
    pub color_number: Option<u8>, // None means turned-off
    pub line_type: Option<String>,
    pub line_weight: Option<u16>,
    pub plot_style_handle: Option<u32>,
    pub material_handle: Option<u32>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LineType {
    pub flags: u16,
    pub description: String,
    pub total_pattern_length: f64,
    pub pattern_lengths: Vec<f64>,
}
impl LineType {
    pub fn is_continuous(&self) -> bool {
        self.pattern_lengths.is_empty()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextStyle {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ucs {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct View {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Viewport {}
