fn main() {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    let drawing = dxfio::Document::open(&args[1]).unwrap();
    println!("{}", serde_yaml::to_string(&drawing).unwrap());
}
