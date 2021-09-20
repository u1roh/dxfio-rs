use crate::{MTextAlignment, MTextCommand, MTextNode};

impl crate::MTextFormatString {
    pub fn parse_and_build_nodes(&mut self) {
        self.raw = parse_control_codes(&self.raw);
        self.nodes = parse_and_build_nodes(&self.raw);
    }
}

fn parse_and_build_nodes(src: &str) -> Vec<MTextNode> {
    // src.find(&['{'])
    let mut dst = vec![];
    for (i, ch) in src.chars().enumerate() {
        // match dst.last_mut() {
        //     Some(crate::MTextNode::Text(range)) => match ch {
        //         '\\' => {
        //             range.end = i;
        //         }
        //         _ => {}
        //     },
        //     _ => {}
        // }
    }
    dst
}

fn parse_texts_and_commands(src: &str, mut range: std::ops::Range<usize>) -> Vec<crate::MTextNode> {
    let mut dst = vec![];
    while let Some(len) = src[range.clone()].find('\\') {
        if len > 0 {
            dst.push(MTextNode::Text(range.start..range.start + len));
            range.start += len;
        }
        if let Some(cmd) = parse_command(src, &mut range) {
            dst.push(MTextNode::Command(cmd));
        }
    }
    dst.push(MTextNode::Text(range.clone()));
    dst
}

fn parse_command(src: &str, range: &mut std::ops::Range<usize>) -> Option<MTextCommand> {
    assert_eq!(src.chars().next(), Some('\\'));
    use MTextCommand::*;
    range.start += 2;
    src.get(range.start - 1..range.start)
        .and_then(|ch| match ch {
            "O" => Some(OStart),
            "o" => Some(OEnd),
            "L" => Some(LStart),
            "l" => Some(LEnd),
            "K" => Some(KStart),
            "k" => Some(KEnd),
            "P" => Some(P),
            "C" => src[range.clone()].find(';').and_then(|k| {
                range.start += k;
                src[range.start - k..range.start].parse().ok().map(C)
            }),
            "F" => src[range.clone()].find(';').map(|k| {
                range.start += k;
                F(src[range.start - k..range.start].to_owned())
            }),
            "H" => src[range.clone()].find(';').and_then(|k| {
                range.start += k;
                let s = &src[range.start - k..range.start];
                if s.ends_with('x') {
                    src[range.start - k..range.start - 1].parse().ok().map(Hx)
                } else {
                    src[range.start - k..range.start].parse().ok().map(H)
                }
            }),
            "T" => src[range.clone()].find(';').and_then(|k| {
                range.start += k;
                src[range.start - k..range.start].parse().ok().map(T)
            }),
            "Q" => src[range.clone()].find(';').and_then(|k| {
                range.start += k;
                src[range.start - k..range.start].parse().ok().map(Q)
            }),
            "W" => src[range.clone()].find(';').and_then(|k| {
                range.start += k;
                src[range.start - k..range.start].parse().ok().map(W)
            }),
            "A" => src[range.clone()].find(';').and_then(|k| {
                range.start += k;
                src[range.start - k..range.start]
                    .parse()
                    .ok()
                    .and_then(|n| {
                        Some(A(match n {
                            0 => MTextAlignment::Bottom,
                            1 => MTextAlignment::Center,
                            2 => MTextAlignment::Top,
                            _ => return None,
                        }))
                    })
            }),
            code => {
                log::error!("unknown format code: '{}'", code);
                None
            }
        })
}

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
