fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("args = {:?}", args);
    // let s = std::fs::read_to_string(&args[1])?;
    let bytes = std::fs::read(&args[1])?;
    let (s, _, _) = encoding_rs::SHIFT_JIS.decode(&bytes);
    // println!("s = {}", s);
    let pairs = code_value_pairs(&s)?;
    println!("pairs = {:?}", pairs);
    Ok(())
}

fn code_value_pairs(s: &str) -> Result<Vec<(i16, &str)>, std::num::ParseIntError> {
    s.lines()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| chunk[0].trim().parse::<i16>().map(|code| (code, chunk[1])))
        .collect()
}
