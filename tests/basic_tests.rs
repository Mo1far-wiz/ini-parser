#[cfg(test)]
mod tests {
    use ini_parser::parser::{IniParser, Rule};
    use pest::Parser;
    use anyhow::{Result, Context};

    #[test]
    fn test_parse_section_name() -> Result<()> {
        let input = "[Section]";
        let parsed = IniParser::parse(Rule::section, input)
            .context("Failed to parse section")?; // Better error message
        assert_eq!(parsed.as_str(), input.trim());
        Ok(())
    }

    #[test]
    fn test_parse_key_value_pair() -> Result<()> {
        let input = "key=value";
        let parsed = IniParser::parse(Rule::key_value, input)
            .context("Failed to parse key-value pair")?;
        let key_value = parsed.into_iter().next().context("No key-value found")?;
        assert_eq!(key_value.as_str(), "key=value");
        Ok(())
    }

    #[test]
    fn test_parse_key() -> Result<()> {
        let input = "username";
        let parsed = IniParser::parse(Rule::key, input)
            .context("Failed to parse key")?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_parse_value() -> Result<()> {
        let input = "admin";
        let parsed = IniParser::parse(Rule::value, input)
            .context("Failed to parse value")?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }
}
