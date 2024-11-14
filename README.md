# ini-parser

This project is a Rust-based utility for parsing `.ini` files, which are commonly used to store configuration settings in a simple key-value pair format. By parsing these files, this tool allows automated processing and extraction of configuration data, useful in various applications, including configuration management and system setup.

## Links

- **Crates.io**: [ini-parser on crates.io](https://crates.io/crates/ini-parser)
- **Documentation**: [ini-parser Documentation](https://docs.rs/ini-parser/latest/ini_parser/index.html)

### Example of an `.ini` File

Here’s an example `.ini` file that can be parsed by this tool:

```ini
[Database]
host=localhost
port=5432

[Server]
address=127.0.0.1
port=8080
```

## Technical Description

### Parsing Process

Using the **Pest** library for Rust, this tool analyzes the structure of `.ini` files, extracting key information such as:

- **Section Name** (e.g., `Database`, `Server`)
- **Key-Value Pairs** (e.g., `host=localhost`, `port=5432`)

The `.ini` files are parsed line-by-line, following a grammar that identifies sections, keys, and values. Pest provides a custom grammar defined in `grammar.pest`, which defines these parsing rules in a concise format.

### Grammar

```pest
ini_file = { SOI ~ (section ~ NEWLINE*)* ~ EOI }

section = {
    "[" ~ section_name ~ "]" ~
    NEWLINE* ~
    key_value* ~
    NEWLINE*
}

section_name = { (!"]" ~ ANY)+ }

key_value = {
    key ~ "=" ~ value ~
    NEWLINE?
}

key = { (!(NEWLINE | "=") ~ ANY)+ }
value = { (!NEWLINE ~ ANY)+ }

NEWLINE = { "\n" }
```

### Let's break down the grammar rule by rule:

- **Overall Structure**
The `.ini` file is represented as an `ini_file`, which can contain multiple `section` blocks.
  ```pest
  ini_file = { SOI ~ (section)* ~ EOI }
  ```
  - `SOI`: Start of input.
  - `EOI`: End of input.
  - The file can contain multiple sections.

- **Section**
Each section is defined as follows:
  ```pest
  section = {
      "[" ~ section_name ~ "]" ~ NEWLINE ~ key_value*
  }
  ```
  - `section_name`: Defines the name of the section (e.g., `Database`).
  - `key_value`: One or more key-value pairs within the section.

- **Section Name**
The name of the section is represented by any characters except `]`.
  ```pest
  section_name = { (!"]" ~ ANY)+ }
  ```

- **Key-Value Pair**
A key-value pair is defined as a `key` followed by an `=` sign and then the `value`.
  ```pest
  key_value = { key ~ "=" ~ value ~ NEWLINE }
  ```

- **Key**
The key is any sequence of characters that does not contain `=`.
  ```pest
  key = { (!"=" ~ ANY)+ }
  ```

- **Value**  
The value can be any sequence of characters that does not contain a newline.
  ```pest
  value = { (!NEWLINE ~ ANY)+ }
  ```

- **Newline**
Newlines are used to separate lines in the file.
  ```pest
  NEWLINE = _{ "\n" }
  ```

### Parsing Outcome and Usage

Once parsed, the extracted data is structured into an `IniFile` object. This object contains a list of `Section` objects, each with a name and key-value pairs. You can access and manipulate the configuration data programmatically.

### Example Usage

After parsing an `.ini` file, you can use the data in your application. Here's how you would parse an INI file and access its data:

```rust
use ini_parser::parser::parse;

fn main() {
    let input = std::fs::read_to_string("config.ini")
        .expect("Error reading INI file");

    let ini_file = parse(&input).expect("Failed to parse INI file");

    for section in &ini_file.sections {
        println!("[{}]", section.name);
        for (key, value) in &section.key_values {
            println!("{} = {}", key, value);
        }
    }
}
```

### Features
- **INI File Parsing**: Supports parsing sections and key-value pairs.
- **Error Handling**: Gracefully handles errors during parsing and provides useful error messages.
- **Extensible**: The grammar is defined in a modular way, making it easy to extend the parser for other configurations if needed.

### Installation

To use this library in your project, add it to your `Cargo.toml` file:

```toml
[dependencies]
ini_parser = "0.1"
```

Then, you can include the library in your code:

```rust
use ini_parser::parser::parse;
```

### Running Tests

To run tests for this project, use the following command:

```bash
cargo test
```

---

###### Thanks for your attention!