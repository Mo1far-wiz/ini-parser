# ini-parser

This project is a Rust-based utility for parsing `.ini` files, which are commonly used to store configuration settings in a simple key-value pair format. By parsing these files, this tool allows automated processing and extraction of configuration data, useful in various applications, including configuration management and system setup.

## Links

- **Crates.io**: [ini-parser on crates.io](https://crates.io/crates/ini-parser)
- **Documentation**: [ini-parser Documentation](https://docs.rs/ini-parser/latest/ini_parser/index.html)

### Example of an `.ini` File

Here’s an example `.ini` file that can be parsed by this tool:

```ini
# A top-level comment
[General]
app_name = MyApplication
version = 1.2.3
description = "An example INI file for testing"

[Database]
host = localhost
port = 5432
username = admin
password = "p@ssw0rd"

[[Database.Connection]]
user = admin
password = secret

[[Database.Settings]]
timeout = 30
pool_size = 10

[Logging]
level=debug
file_path=/var/log/app.log

[Server]
host=0.0.0.0
port=8080

[Features]
enable_feature_x=true
enable_feature_y=false
```

## Technical Description

### Parsing Process

Using the **Pest** library for Rust, this tool analyzes the structure of `.ini` files, extracting key information such as:

- **Section Name** (e.g., `Database`, `Server`)
- **Key-Value Pairs** (e.g., `host=localhost`, `port=5432`)

The `.ini` files are parsed line-by-line, following a grammar that identifies sections, keys, and values. Pest provides a custom grammar defined in `grammar.pest`, which defines these parsing rules in a concise format.

### Grammar

```pest
ini_file = { SOI ~ (comment | section | NEWLINE)* ~ EOI }

WHITESPACE = _{ " " | "\t" }
NEWLINE = { "\n" }

section = {
    "[" ~ section_name ~ "]" ~
    NEWLINE* ~
    (comment | key_value | nested_section | NEWLINE)*
}
nested_section = {
    "[[" ~ section_name ~ "]]" ~
    NEWLINE* ~
    (comment | key_value | NEWLINE)*
}

key_value = {
    key ~ WHITESPACE? ~ "=" ~ WHITESPACE? ~ (quoted_value | value)? ~ NEWLINE?
}
section_name = { (ASCII_ALPHANUMERIC | "_" | "-" | ".")+ }
key = { (ASCII_ALPHANUMERIC | "_" | "-")+ }
value = { (ASCII_ALPHANUMERIC | PUNCTUATION | " ")+ }
quoted_value = { "\"" ~ (!"\"" ~ (ASCII_ALPHANUMERIC | PUNCTUATION | " "))* ~ "\"" }

comment = { ("#" | ";") ~ (!NEWLINE ~ (ASCII_ALPHANUMERIC | PUNCTUATION | " "))* ~ NEWLINE? }
```

Here's the breakdown of your updated grammar:

### My Grammar rule by rule:

- **Overall Structure**
  The root rule for parsing an INI file is represented as `ini_file`. It allows for multiple sections, comments, and newlines, and must start and end with `SOI` (Start of Input) and `EOI` (End of Input).
  ```pest
  ini_file = { SOI ~ (comment | section | NEWLINE)* ~ EOI }
  ```
  - `SOI`: Start of Input.
  - `EOI`: End of Input.
  - The file can contain sections, comments, and newlines.

- **Whitespace**
  The `WHITESPACE` rule defines space or tab characters as optional spacing in key-value pairs.
  ```pest
  WHITESPACE = _{ " " | "\t" }
  ```
  - `WHITESPACE` can be either a space or a tab.

- **Section**
  A section is defined by a name enclosed in square brackets `[]`, with an optional newline and a combination of comments, key-value pairs, or nested sections.
  ```pest
  section = {
      "[" ~ section_name ~ "]" ~ NEWLINE* ~ (comment | key_value | nested_section | NEWLINE)*
  }
  ```
  - `section_name`: The name of the section.
  - The section can contain comments, key-value pairs, or nested sections.
  - The section can be followed by newlines.

- **Nested Section**
  A nested section is a section enclosed in double square brackets `[[]]`, with similar content as regular sections, including key-value pairs and comments.
  ```pest
  nested_section = {
      "[[" ~ section_name ~ "]]" ~ NEWLINE* ~ (comment | key_value | NEWLINE)*
  }
  ```
  - This allows for nested sections within a section.

- **Key-Value Pair**
  A key-value pair is composed of a key followed by an optional space around the `=` sign, followed by an optional quoted or unquoted value. The key-value pair ends with an optional newline.
  ```pest
  key_value = {
      key ~ WHITESPACE? ~ "=" ~ WHITESPACE? ~ (quoted_value | value)? ~ NEWLINE?
  }
  ```
  - `key`: The key in the key-value pair.
  - `quoted_value | value`: The value of the key, either quoted or unquoted.
  - Newline is optional after the key-value pair.

- **Section Name**
  A section name is a sequence of one or more alphanumeric characters, underscores, hyphens, or dots.
  ```pest
  section_name = { (ASCII_ALPHANUMERIC | "_" | "-" | ".")+ }
  ```

- **Key**
  The key consists of one or more alphanumeric characters, underscores, or hyphens, excluding the equal sign `=`.
  ```pest
  key = { (ASCII_ALPHANUMERIC | "_" | "-")+ }
  ```

- **Value**
  A simple value consists of ASCII printable characters (except control characters, backslashes, and newlines), including punctuation and spaces.
  ```pest
  value = { (ASCII_ALPHANUMERIC | PUNCTUATION | " ")+ }
  ```

- **Quoted Value**
  A quoted value is enclosed in double quotes and may contain spaces, punctuation, and other special characters, except the quote itself.
  ```pest
  quoted_value = { "\"" ~ (!"\"" ~ (ASCII_ALPHANUMERIC | PUNCTUATION | " "))* ~ "\"" }
  ```

- **Comment**
  A comment starts with `#` or `;` and continues until the end of the line, optionally followed by a newline.
  ```pest
  comment = { ("#" | ";") ~ (!NEWLINE ~ (ASCII_ALPHANUMERIC | PUNCTUATION | " "))* ~ NEWLINE? }
  ```

- **Newline**
  The `NEWLINE` rule matches a newline character `\n`.
  ```pest
  NEWLINE = { "\n" }
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

    for (section_name, section) in ini_file.sections {
        println!("[{}]", section_name);
        for (key, value) in &section.key_values {
            println!("{} = {}", key, value);
        }

        for (nested_name, nested_section) in &section.nested_sections {
            println!("[[{}]]", nested_name);
            for (key, value) in &nested_section.key_values {
                println!("{} = {}", key, value);
            }
        }

        println!();
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