fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("args = {:?}", args);
    // let s = std::fs::read_to_string(&args[1])?;
    let bytes = std::fs::read(&args[1])?;
    let (s, _, _) = encoding_rs::SHIFT_JIS.decode(&bytes);
    // println!("s = {}", s);
    let pairs = code_value_pairs(&s)?;
    // println!("pairs = {:?}", pairs);
    let nodes = Node::parse(&pairs);
    println!("nodes.len() = {}", nodes.len());
    for node in &nodes {
        node.print(0);
    }
    Ok(())
}

type CodeValue<'a> = (i16, &'a str);

fn code_value_pairs(s: &str) -> Result<Vec<CodeValue>, std::num::ParseIntError> {
    s.lines()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| chunk[0].trim().parse::<i16>().map(|code| (code, chunk[1])))
        .collect()
}

#[derive(Debug)]
struct Node<'a> {
    pub entity_type: &'a str,
    pub code_values: &'a [CodeValue<'a>],
    pub nodes: Vec<Self>,
}
impl<'a> Node<'a> {
    fn print(&self, indent: usize) {
        for _ in 0..indent {
            print!("  ");
        }
        println!(
            "{}: code_values.len() = {}",
            self.entity_type,
            self.code_values.len()
        );
        for node in &self.nodes {
            node.print(indent + 1);
        }
    }
    fn parse(code_values: &'a [CodeValue<'a>]) -> Vec<Self> {
        Self::parse_nodes(code_values, 0).unwrap_or_default().0
    }
    fn parse_node(code_values: &'a [CodeValue<'a>], index: usize) -> Option<(Self, usize)> {
        (index..code_values.len())
            .find(|i| code_values[*i].0 == 0)
            .and_then(|start| {
                let entity_type = code_values[start].1;
                if Self::CONTAINER_TYPES.contains(&entity_type) {
                    Self::parse_nodes(code_values, start + 1).map(|(nodes, end)| {
                        let node = Self {
                            entity_type,
                            code_values: &code_values[start + 1..end],
                            nodes,
                        };
                        (node, end)
                    })
                } else {
                    Self::parse_entity(entity_type, code_values, start + 1)
                }
            })
    }
    fn parse_nodes(
        code_values: &'a [CodeValue<'a>],
        mut index: usize,
    ) -> Option<(Vec<Self>, usize)> {
        let mut nodes = vec![];
        while let Some((node, i)) = Self::parse_node(code_values, index) {
            if node.entity_type.contains("END") {
                return Some((nodes, i));
            }
            index = i;
            nodes.push(node);
        }
        if code_values[index] == (0, "EOF") {
            Some((nodes, index))
        } else {
            None
        }
    }
    fn parse_entity(
        entity_type: &'a str,
        code_values: &'a [CodeValue<'a>],
        start: usize,
    ) -> Option<(Self, usize)> {
        (start..code_values.len())
            .find(|i| code_values[*i].0 == 0)
            .map(|end| {
                let entity = Self {
                    entity_type,
                    code_values: &code_values[start..end],
                    nodes: vec![],
                };
                (entity, end)
            })
    }
    const CONTAINER_TYPES: &'static [&'static str] = &["SECTION", "BLOCK", "TABLE", "POLYLINE"];
}
