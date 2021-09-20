// https://knowledge.autodesk.com/ja/support/autocad-lt/learn-explore/caas/CloudHelp/cloudhelp/2020/JPN/AutoCAD-LT/files/GUID-968CBC1D-BA99-4519-ABDD-88419EB2BF92-htm.html
pub(super) fn parse_control_codes(mut src: &str) -> String {
    let mut dst = String::default();
    let mut overlined = false;
    let mut underlined = false;
    let mut struck = false;
    while let Some(k) = src.find("%%") {
        eprintln!("k = {}", k);
        dst += &src[..k];
        src = &src[k + 2..];
        // 仕様が理解できず。"%% nnn" と書かれているが、3桁固定…？ Unicodeは十進数3桁では収まらないが…。
        if let Some(ch) = src[..3].parse().ok().and_then(char::from_u32) {
            src = &src[3..];
            dst.push(ch);
        } else {
            match &src[..1] {
                "c" => dst.push('⌀'),
                "d" => dst.push('°'),
                "p" => dst.push('±'),
                "%" => dst.push('%'),
                "o" => {
                    dst += if overlined { "\\O" } else { "\\o" };
                    overlined = !overlined;
                }
                "u" => {
                    dst += if underlined { "\\L" } else { "\\l" };
                    underlined = !underlined;
                }
                "k" => {
                    dst += if struck { "\\K" } else { "\\k" };
                    struck = !struck;
                }
                _ => {}
            }
            src = &src[1..];
        }
    }
    dst += src;
    dst
}

#[test]
fn test_parse_control_codes() {
    assert_eq!(parse_control_codes("abc%%c12mm"), "abc⌀12mm");
    assert_eq!(parse_control_codes("abc%%c12%%p0.1mm"), "abc⌀12±0.1mm");
}
