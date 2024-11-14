use std::collections::HashMap;

#[derive(Debug)]
pub struct IniFile {
    pub sections: HashMap<String, Section>,
}

#[derive(Debug)]
pub struct Section {
    pub name: String,
    pub key_values: HashMap<String, String>,
}
