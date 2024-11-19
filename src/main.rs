use clap::{Parser, Subcommand};
use ini_parser::ini_file::{IniFile, Section};
use ini_parser::parser::ParseError;
use std::fs;
use std::process::exit;

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
    /// Parses an INI file and displays its content
    Parse {
        /// Path to the INI file
        #[arg(short, long)]
        file: String,
    },
    /// Displays credits
    Credits,
    /// Displays help information
    Help,
}

fn print_section(section: &Section, indent: usize) {
    let indent_str = " ".repeat(indent);

    // Print key-value pairs
    for (key, value) in &section.key_values {
        println!("{}{} = {}", indent_str, key, value);
    }

    // Recursively print nested sections
    for (nested_name, nested_section) in &section.nested_sections {
        println!("\n{}[{}]", indent_str, nested_name);
        print_section(nested_section, indent + 2);
    }
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { file } => {
            // Read the file content
            let ini_content = fs::read_to_string(file).unwrap_or_else(|err| {
                eprintln!("Error reading file '{}': {}", file, err);
                exit(1);
            });

            // Parse the content
            let ini_file = IniFile::from_str(&ini_content).unwrap_or_else(|err| {
                eprintln!("Failed to parse INI file: {}", format_parse_error(err));
                exit(1);
            });

            // Display the parsed content
            println!("Parsed INI File:");
            for (section_name, section) in ini_file.sections {
                println!("[{}]", section_name);
                print_section(&section, 2);
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

/// Formats a `ParseError` into a user-friendly message.
fn format_parse_error(error: ParseError) -> String {
    match error {
        ParseError::PestError(e) => format!("Parsing error: {}", e),
        ParseError::KeyValueOutsideSection => "Key-value pair found outside of any section".to_string(),
        ParseError::UnexpectedRule(rule) => format!("Unexpected rule: {:?}", rule),
    }
}
