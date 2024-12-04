use std::{env, fs, path::Path};

fn main() {
    //const XMAS: &str = "XMAS";
    const INPUT_DATA_FOLDER: &str = "src/data/";
    const INPUT_FILE_NAME: &str = "day04_input.txt";

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let input_path = Path::new(&manifest_dir).join(format!("{}{}", INPUT_DATA_FOLDER, INPUT_FILE_NAME));
    let input = fs::read_to_string(input_path);

    println!("Input {:?}", input);
}       
