fn main() {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    println!("args = {:?}", args);
    let bytes = std::fs::read(&args[1]).unwrap();
    let s = dxfio::parser::bytes_to_string(&bytes).unwrap();
    let atoms = dxfio::Atom::parse_str(&s).unwrap();
    let nodes = dxfio::Node::parse_atoms(&atoms);
    let drawing = dxfio::Drawing::parse_nodes(&nodes);
    println!("{:?}", drawing);
}
