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
    RegApp(RegApp),       // APPID
    Block(Block),         // BLOCK_RECORD
    DimStyle(DimStyle),   // DIMSTYLE
    Layer(Layer),         // LAYER
    LineType(LineType),   // LTYPE
    TextStyle(TextStyle), // STYLE
    Ucs(Ucs),             // UCS
    View(View),           // VIEW
    Viewport(Viewport),   // VPORT
    Unknown(crate::DxfNode),
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
    pub arrow_block_name_1st: String,          // 6 DIMBLK1 (obsolete, now object ID)
    pub arrow_block_name_2nd: String,          // 7 DIMBLK2 (obsolete, now object ID)
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
                                               // pub _: f64,    // 141 DIMCEN
                                               // pub _: f64,    // 142 DIMTSZ
                                               // pub _: f64,    // 143 DIMALTF
                                               // pub _: f64,    // 144 DIMLFAC
                                               // pub _: f64,    // 145 DIMTVP
                                               // pub _: f64,    // 146 DIMTFAC
                                               // pub _: f64,    // 147 DIMGAP
                                               // pub _: f64,    // 148 DIMALTRND
                                               // pub _: i16,    // 71 DIMTOL
                                               // pub _: i16,    // 72 DIMLIM
                                               // pub _: i16,    // 73 DIMTIH
                                               // pub _: i16,    // 74 DIMTOH
                                               // pub _: i16,    // 75 DIMSE1
                                               // pub _: i16,    // 76 DIMSE2
                                               // pub _: i16,    // 77 DIMTAD
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
