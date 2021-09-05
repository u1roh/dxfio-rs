fn main() {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    println!("args = {:?}", args);
    let bytes = std::fs::read(&args[1]).unwrap();
    let s = dxfio::parser::bytes_to_string(&bytes).unwrap();
    let atoms = dxfio::parser::ParAtom::parse(&s).unwrap();
    let nodes = dxfio::parser::ParNode::parse(&atoms);
    let drawing = dxfio::parser::ParDrawing::parse(&nodes);
    println!("{:?}", drawing);
}
