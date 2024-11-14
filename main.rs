use clap::{Arg, Command};
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::process;

fn parse_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    for result in rdr.records() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => eprintln!("Error reading record: {}", err),
        }
    }
    Ok(())
}

fn show_credits() {
    println!("CSV Parser CLI\nCreated by: Your Name\nVersion: 1.0");
}

fn main() {
    let matches = Command::new("csv-parser")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("CLI tool to parse CSV files")
        .subcommand(
            Command::new("parse").about("Parse a CSV file").arg(
                Arg::new("file")
                    .required(true)
                    .help("The path to the CSV file"),
            ),
        )
        .subcommand(Command::new("credits").about("Show credits information"))
        .get_matches();

    match matches.subcommand() {
        Some(("parse", sub_m)) => {
            let file_path = sub_m.get_one::<String>("file").expect("file is required");
            if let Err(e) = parse_csv(file_path) {
                eprintln!("Error parsing CSV: {}", e);
                process::exit(1);
            }
        }
        Some(("credits", _)) => show_credits(),
        _ => {
            println!("Usage:");
            println!("  csv-parser <command> [options]");
            println!("\nAvailable commands:");
            println!("  parse <file_path>    Parse the given CSV file");
            println!("  credits              Show credits information");
        }
    }
}
