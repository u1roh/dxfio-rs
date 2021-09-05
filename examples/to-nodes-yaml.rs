fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nodes = dxfio::DxfNode::open(&args[1]).unwrap();
    println!("{}", serde_yaml::to_string(&nodes).unwrap());
}