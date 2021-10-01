use crate::{MTextAlignment, MTextCommand, MTextNode};

impl crate::MTextFormatString {
    pub fn parse_and_build_nodes(&mut self) {
        self.raw = parse_control_codes(&self.raw);
        self.nodes = parse_to_nodes(&self.raw);
    }
}

fn parse_to_nodes(s: &str) -> Vec<MTextNode> {
    chars_to_nodes(&mut s.chars(), &[]).0
}

fn chars_to_nodes(
    chars: &mut impl Iterator<Item = char>,
    ends: &[char],
) -> (Vec<MTextNode>, Option<char>) {
    enum Event {
        Exit,
        Node(MTextNode),
        Char(char),
    }
    let mut dst = vec![];
    let mut backslash = false;
    let mut text = String::default();
    let mut on_event = |event| match event {
        Event::Char(ch) => text.push(ch),
        Event::Node(node) => {
            if !text.is_empty() {
                dst.push(MTextNode::Text(std::mem::take(&mut text)));
            }
            dst.push(node);
        }
        Event::Exit => {
            if !text.is_empty() {
                dst.push(MTextNode::Text(std::mem::take(&mut text)));
            }
        }
    };
    while let Some(ch) = chars.next() {
        if backslash {
            backslash = false;
            if ['\\', '{', '}'].contains(&ch) {
                on_event(Event::Char(ch));
            } else if ch == 'S' {
                let (nodes1, sep) = chars_to_nodes(chars, &['^', '#', '/']);
                let (nodes2, _) = chars_to_nodes(chars, &[';']);
                let stack_type = match sep {
                    Some('/') => crate::MTextStackType::Slash,
                    Some('#') => crate::MTextStackType::Number,
                    Some('^') => crate::MTextStackType::Hat,
                    None => crate::MTextStackType::Hat,
                    _ => unreachable!(),
                };
                on_event(Event::Node(MTextNode::Stacked(nodes1, nodes2, stack_type)));
            } else if let Some(cmd) = parse_command(ch, chars) {
                on_event(Event::Node(MTextNode::Command(cmd)));
            }
        } else if ends.contains(&ch) {
            on_event(Event::Exit);
            return (dst, Some(ch));
        } else if ch == '\\' {
            backslash = true;
        } else if ch == '{' {
            let (nodes, _) = chars_to_nodes(chars, &['}']);
            on_event(Event::Node(MTextNode::Block(nodes)));
        } else {
            on_event(Event::Char(ch));
        }
    }
    on_event(Event::Exit);
    (dst, None)
}

#[test]
fn test_parse_to_nodes() {
    assert_eq!(parse_to_nodes("abc"), &[MTextNode::Text("abc".to_owned())]);
    assert_eq!(
        parse_to_nodes("a\\\\bc"),
        &[MTextNode::Text("a\\bc".to_owned())]
    );
    assert_eq!(
        parse_to_nodes("a\\{b\\}c"),
        &[MTextNode::Text("a{b}c".to_owned())]
    );
    assert_eq!(
        parse_to_nodes("\\W12.34;abc"),
        &[
            MTextNode::Command(MTextCommand::W(12.34)),
            MTextNode::Text("abc".to_owned())
        ]
    );
    assert_eq!(
        parse_to_nodes("a{b}c"),
        &[
            MTextNode::Text("a".to_owned()),
            MTextNode::Block(vec![MTextNode::Text("b".to_owned())]),
            MTextNode::Text("c".to_owned())
        ]
    );
    assert_eq!(
        parse_to_nodes("{\\H1.8;abc}"),
        &[MTextNode::Block(vec![
            MTextNode::Command(MTextCommand::H(1.8)),
            MTextNode::Text("abc".to_owned())
        ]),]
    );
}

fn parse_command(ch: char, chars: &mut impl Iterator<Item = char>) -> Option<MTextCommand> {
    use MTextCommand::*;
    fn read_to_semicolon(chars: &mut impl Iterator<Item = char>) -> String {
        chars.take_while(|&c| c != ';').collect()
    }
    match ch {
        'O' => Some(OStart),
        'o' => Some(OEnd),
        'L' => Some(LStart),
        'l' => Some(LEnd),
        'K' => Some(KStart),
        'k' => Some(KEnd),
        'P' => Some(P),
        'H' => {
            let s = read_to_semicolon(chars);
            if s.ends_with('x') {
                s[..s.len() - 1].parse().ok().map(Hx)
            } else {
                s.parse().ok().map(H)
            }
        }
        'C' => read_to_semicolon(chars).parse().ok().map(C),
        'T' => read_to_semicolon(chars).parse().ok().map(T),
        'Q' => read_to_semicolon(chars).parse().ok().map(Q),
        'W' => read_to_semicolon(chars).parse().ok().map(W),
        'F' | 'f' => Some(F(read_to_semicolon(chars))),
        'A' => read_to_semicolon(chars).parse().ok().and_then(|n| {
            Some(A(match n {
                0 => MTextAlignment::Bottom,
                1 => MTextAlignment::Center,
                2 => MTextAlignment::Top,
                _ => return None,
            }))
        }),
        code => {
            log::error!("unknown format code: '{}'", code);
            None
        }
    }
}

// https://knowledge.autodesk.com/ja/support/autocad-lt/learn-explore/caas/CloudHelp/cloudhelp/2020/JPN/AutoCAD-LT/files/GUID-968CBC1D-BA99-4519-ABDD-88419EB2BF92-htm.html
pub(super) fn parse_control_codes(mut src: &str) -> String {
    let mut dst = String::default();
    let mut overlined = false;
    let mut underlined = false;
    let mut struck = false;
    while let Some(k) = src.find("%%") {
        dst += &src[..k];
        src = &src[k + 2..];
        // 仕様が理解できず。"%% nnn" と書かれているが、3桁固定…？ Unicodeは十進数3桁では収まらないが…。
        if let Some(ch) = src.get(..3).and_then(|s| s.parse().ok()).and_then(char::from_u32) {
            src = &src[3..];
            dst.push(ch);
        } else {
            match &src[..1] {
                "c" | "C" => dst.push('⌀'),
                "d" | "D" => dst.push('°'),
                "p" | "P" => dst.push('±'),
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
