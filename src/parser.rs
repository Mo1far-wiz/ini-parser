use pest::error::Error as PestError;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use thiserror::Error;

use crate::ini_file::{IniFile, Section};

/// The parser for the INI file format using Pest.
///
/// This parser is responsible for parsing the entire INI file, including sections,
/// key-value pairs, and nested sections. It uses Pest's PEG grammar to process the
#[derive(Parser)]
#[grammar = "ini_grammar.pest"]
pub struct IniParser;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Parsing error: {0}")]
    PestError(#[from] Box<PestError<Rule>>),

    #[error("Key-value pair found outside of any section")]
    KeyValueOutsideSection,

    #[error("Unexpected rule: {0:?}")]
    UnexpectedRule(Rule),
}

impl IniFile {
    /// Parses a string representation of an INI file and returns an `IniFile` struct.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the contents of an INI file.
    ///
    /// # Returns
    ///
    /// Returns a `Result` that contains the `IniFile` on success or a `ParseError` on failure.
    pub fn from_str(input: &str) -> Result<Self, ParseError> {
        let pairs = IniParser::parse(Rule::ini_file, input.trim_start())
            .map_err(|e| ParseError::PestError(Box::new(e)))?;

        let mut sections = HashMap::new();
        let mut current_section: Option<Section> = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::ini_file => {
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::section => {
                                if let Some(section) = current_section.take() {
                                    sections.insert(section.name.clone(), section);
                                }

                                current_section = Some(Section::parse(inner_pair)?);
                            }
                            Rule::nested_section => {
                                if let Some(parent_section) = current_section.as_mut() {
                                    let nested_section = Section::parse(inner_pair)?;
                                    parent_section
                                    .nested_sections
                                    .insert(nested_section.name.clone(), nested_section);

                                } else {
                                    return Err(ParseError::KeyValueOutsideSection);
                                }
                            }
                            Rule::key_value => {
                                if let Some(section) = current_section.as_mut() {
                                    let (key, value) = parse_key_value(inner_pair)?;
                                    section.key_values.insert(key, value);
                                } else {
                                    return Err(ParseError::KeyValueOutsideSection);
                                }
                            }
                            Rule::comment | Rule::NEWLINE => {
                                // Ignore comments and newlines
                            }
                            Rule::EOI => {
                                // End of input, ignore it
                            }
                            _ => return Err(ParseError::UnexpectedRule(inner_pair.as_rule())),
                        }
                    }
                }
                _ => return Err(ParseError::UnexpectedRule(pair.as_rule())),
            }
        }
        if let Some(section) = current_section {
            sections.insert(section.name.clone(), section);
        }
        Ok(IniFile { sections })
    }
}

impl Section {
    /// Parses a section from a given pair.
    ///
    /// # Arguments
    ///
    /// * `pair` - The pair to parse, typically representing a section from the INI input.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with a `Section` on success, or a `ParseError` on failure.
    fn parse(pair: Pair<Rule>) -> Result<Self, ParseError> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or_else(|| ParseError::UnexpectedRule(Rule::section_name))?
            .as_str()
            .to_string();

        let mut key_values = HashMap::new();
        let mut nested_sections = HashMap::new();

        for inner_pair in inner {
            match inner_pair.as_rule() {
                Rule::key_value => {
                    let (key, value) = parse_key_value(inner_pair)?;
                    key_values.insert(key, value);
                }
                Rule::nested_section => {
                    let nested_section = Section::parse(inner_pair)?;
                    nested_sections.insert(nested_section.name.clone(), nested_section);
                }
                _ => {}
            }
        }

        Ok(Section {
            name,
            key_values,
            nested_sections,
        })
    }
}


/// Parse a key-value pair into a tuple of key and value.
fn parse_key_value(pair: Pair<Rule>) -> Result<(String, String), ParseError> {
    let mut inner = pair.into_inner();
    let key = inner
        .next()
        .ok_or_else(|| ParseError::UnexpectedRule(Rule::key))?
        .as_str()
        .to_string();
    let value = inner.next().map_or_else(
        || "".to_string(), // Handle cases with no value
        |v| v.as_str().to_string(),
    );
    Ok((key, value))
}

pub fn parse(input: &str) -> Result<IniFile, ParseError> {
    IniFile::from_str(input)
}