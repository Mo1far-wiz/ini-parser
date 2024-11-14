use pest::error::Error as PestError;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use thiserror::Error;

use crate::ini_file::{IniFile, Section};

#[derive(Parser)]
#[grammar = "ini_grammar.pest"] // Replace with the actual path to your .pest file
pub struct IniParser;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Parsing error: {0}")]
    PestError(#[from] Box<PestError<Rule>>),
}

impl Section {
    fn parse(pair: Pair<Rule>) -> Result<Self, ParseError> {
        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().as_str().to_string();
        let mut key_values = HashMap::new();

        for item in inner {
            if item.as_rule() == Rule::key_value {
                let mut kv = item.into_inner();
                let key = kv.next().unwrap().as_str().to_string();
                let value = kv.next().unwrap().as_str().to_string();
                key_values.insert(key, value);
            }
        }

        Ok(Section { name, key_values })
    }
}

impl IniFile {
    pub fn parse(pair: Pair<Rule>) -> Result<Self, ParseError> {
        let mut sections = HashMap::new();

        for section_pair in pair
            .into_inner()
            .filter(|item| item.as_rule() == Rule::section)
        {
            let section = Section::parse(section_pair)?;
            sections.insert(section.name.clone(), section);
        }

        Ok(IniFile { sections })
    }
}

pub fn parse(input: &str) -> Result<IniFile, ParseError> {
    let file = IniParser::parse(Rule::ini_file, input.trim_start())
        .expect("unable to parse")
        .next()
        .unwrap();

    IniFile::parse(file)
}
