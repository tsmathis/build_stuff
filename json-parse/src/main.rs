use serde_json::{Result, Value};

fn main() -> Result<()> {
    let fp = std::env::args().nth(1).expect("No file path provided");
    let contents =
        std::fs::read_to_string(&fp).unwrap_or_else(|_| panic!("failed to read file: {}", fp));

    let json: Value = serde_json::from_str(&contents)?;

    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    Ok(())
}
