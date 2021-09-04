use bare_dxf::parser::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("args = {:?}", args);
    // let s = std::fs::read_to_string(&args[1])?;
    let bytes = std::fs::read(&args[1])?;
    let (s, _, _) = encoding_rs::SHIFT_JIS.decode(&bytes);
    // println!("s = {}", s);
    let atoms = DxfAtom::parse(&s)?;
    // println!("pairs = {:?}", pairs);
    let nodes = DxfNode::parse(&atoms);
    println!("nodes.len() = {}", nodes.len());
    for node in &nodes {
        node.print(0);
    }
    let drawing = ParDrawing::parse(&nodes);
    println!("{:?}", drawing);
    Ok(())
}
