use clap::{Parser, Subcommand};
use std::fs;
use std::process::exit;

use ini_parser::parser::parse;

#[derive(Parser, Debug)]
#[command(
    author = "Oleksandr Prokhorov <oleksandr.prokhorov@ukma.edu.ua>",
    version = "1.0",
    about = "INI File Parser",
    long_about = "This CLI parses INI files and displays their contents in a human-readable format.",
    disable_help_flag = true,
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Parse {
        #[arg(short, long)]
        file: String,
    },
    Credits,
    Help,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { file } => {
            let ini_content = fs::read_to_string(file).unwrap_or_else(|err| {
                eprintln!("Error reading file '{}': {}", file, err);
                exit(1);
            });

            let ini_file = match parse(&ini_content) {
                Ok(ini_file) => ini_file,
                Err(e) => {
                    eprintln!("Failed to parse INI file: {}", e);
                    exit(1)
                }
            };

            println!("Parsed INI File:");
            for (section_name, section) in &ini_file.sections {
                println!("[{}]", section_name);
                for (key, value) in &section.key_values {
                    println!("{} = {}", key, value);
                }
                println!();
            }
        }
        Commands::Credits => {
            println!("INI File Parser CLI");
            println!("Version: 1.0");
            println!("Author: Oleksandr Prokhorov <oleksandr.prokhorov@ukma.edu.ua>");
            println!("This tool was developed as a part of the Rust course at National University of \"Kyiv-Mohyla Academy\".");
        }
        Commands::Help => {
            println!("INI File Parser CLI");
            println!();
            println!("USAGE:");
            println!("\tini_file_parser <COMMAND>");
            println!();
            println!("COMMANDS:");
            println!("\tparse   Parses an INI file and displays its contents.");
            println!("\t\tOptions:");
            println!("\t\t\t--file <FILE>     Specify the path to the .ini file to parse.");
            println!();
            println!("\tcredits Shows credits and authorship information.");
            println!();
            println!("\thelp    Displays this help information.");
            println!();
        }
    }
}
