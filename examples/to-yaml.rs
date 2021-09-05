fn main() {
    let args: Vec<String> = std::env::args().collect();
    let drawing = dxfio::Drawing::open(&args[1]).unwrap();
    println!("{}", serde_yaml::to_string(&drawing).unwrap());
}
