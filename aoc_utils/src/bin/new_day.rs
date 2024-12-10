use std::env;
use aoc_utils::create_day_files;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: cargo new-day <year> <day_number>");
        std::process::exit(1);
    }

    let year = &args[1];
    let day = args[2].parse::<u32>().unwrap_or_else(|_| {
        eprintln!("Day must be a number");
        std::process::exit(1);
    });

    if let Err(e) = create_day_files(year, day) {
        eprintln!("Error creating day files: {}", e);
        std::process::exit(1);
    }

    let package_name = format!("year_{}", year);
    let day_padded = format!("{:02}", day);

    println!("\nDay {} created!", day_padded);
    println!("cargo run --package {} --bin day{}", package_name, day_padded);
}
