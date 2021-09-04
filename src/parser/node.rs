use super::DxfAtom;

#[derive(Debug)]
pub struct DxfNode<'a> {
    pub node_type: &'a str,
    pub atoms: &'a [DxfAtom<'a>],
    pub nodes: Vec<Self>,
}
impl<'a> DxfNode<'a> {
    pub fn print(&self, indent: usize) {
        for _ in 0..indent {
            print!("  ");
        }
        println!("{}: atoms.len() = {}", self.node_type, self.atoms.len());
        for node in &self.nodes {
            node.print(indent + 1);
        }
    }
    pub fn find(&self, code: i16) -> Option<&str> {
        self.atoms
            .iter()
            .find(|item| item.code == code)
            .map(|item| item.value)
    }
    pub fn get<T: std::str::FromStr>(&self, code: i16) -> Option<T> {
        self.find(code)?.parse().ok()
    }
    pub fn get_or_default<T: std::str::FromStr + Default>(&self, code: i16) -> T {
        self.get(code).unwrap_or_default()
    }
    pub fn parse(atoms: &'a [DxfAtom<'a>]) -> Vec<Self> {
        NodeParser { atoms }.parse_nodes(0).unwrap_or_default().0
    }
}

struct NodeParser<'a> {
    atoms: &'a [DxfAtom<'a>],
}
impl<'a> NodeParser<'a> {
    fn parse_nodes(&self, mut start: usize) -> Option<(Vec<DxfNode<'a>>, usize)> {
        let mut nodes = vec![];
        while let Some((node, end)) = self.parse_node(start) {
            if node.node_type.contains("END") {
                return Some((nodes, end));
            }
            start = end;
            nodes.push(node);
        }
        if self.atoms[start].value == "EOF" {
            Some((nodes, start))
        } else {
            None
        }
    }
    fn parse_node(&self, start: usize) -> Option<(DxfNode<'a>, usize)> {
        const CONTAINER_TYPES: &[&str] = &["SECTION", "BLOCK", "TABLE", "POLYLINE"];
        (start..self.atoms.len())
            .find(|i| self.atoms[*i].code == 0)
            .and_then(|start| {
                let node_type = self.atoms[start].value;
                if CONTAINER_TYPES.contains(&node_type) {
                    self.parse_container(node_type, start + 1)
                } else {
                    self.parse_element(node_type, start + 1)
                }
            })
    }
    fn parse_container(&self, node_type: &'a str, start: usize) -> Option<(DxfNode<'a>, usize)> {
        self.parse_nodes(start).map(|(nodes, end)| {
            let node = DxfNode {
                node_type,
                atoms: &self.atoms[start..end],
                nodes,
            };
            (node, end)
        })
    }
    fn parse_element(&self, node_type: &'a str, start: usize) -> Option<(DxfNode<'a>, usize)> {
        (start..self.atoms.len())
            .find(|i| self.atoms[*i].code == 0)
            .map(|end| {
                let entity = DxfNode {
                    node_type,
                    atoms: &self.atoms[start..end],
                    nodes: vec![],
                };
                (entity, end)
            })
    }
}
