use dxfio::parser::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("args = {:?}", args);
    // let s = std::fs::read_to_string(&args[1])?;
    let bytes = std::fs::read(&args[1])?;
    let s = bytes_to_string(&bytes)?;
    // println!("s = {}", s);
    let atoms = ParAtom::parse(&s)?;
    // println!("pairs = {:?}", pairs);
    let nodes = ParNode::parse(&atoms);
    if false {
        let dxfnodes: Vec<dxfio::DxfNode> = nodes.iter().map(Into::into).collect();
        // println!("{}", serde_yaml::to_string(&dxfnodes)?);
        println!("{}", serde_json::to_string_pretty(&dxfnodes)?);
    } else {
        //let drawing = ParDrawing::parse(&nodes);
        let drawing = dxfio::Drawing::parse_nodes(&nodes);
        // println!("{:?}", drawing);
        // println!("{}", serde_yaml::to_string(&drawing)?);
        println!("{}", serde_json::to_string_pretty(&drawing)?);
    }
    Ok(())
}
