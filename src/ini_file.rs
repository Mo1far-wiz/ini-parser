use std::collections::HashMap;

#[derive(Debug)]
/// Represents an INI file with sections and key-value pairs.
pub struct IniFile {
    /// The sections of the INI file, each containing key-value pairs.
    pub sections: HashMap<String, Section>,
}

#[derive(Debug)]
/// Represents a section in an INI file.
///
/// A section contains key-value pairs and can optionally have nested sections.
pub struct Section {
    pub name: String,
    pub key_values: HashMap<String, String>,
    pub nested_sections: HashMap<String, Section>, // Add nested sections
}
