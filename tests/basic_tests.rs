#[cfg(test)]
mod tests {
    use ini_parser::parser::{IniParser, Rule};
    use pest::Parser;

    #[test]
    fn test_parse_section_name() {
        let input = "[Section]";
        let parsed = IniParser::parse(Rule::section, input).unwrap();
        assert_eq!(parsed.as_str(), input.trim());
    }

    #[test]
    fn test_parse_key_value_pair() {
        let input = "key=value";
        let parsed = IniParser::parse(Rule::key_value, input).unwrap();
        let key_value = parsed.into_iter().next().unwrap();
        assert_eq!(key_value.as_str(), "key=value");
    }

    #[test]
    fn test_parse_key() {
        let input = "username";
        let parsed = IniParser::parse(Rule::key, input).unwrap();
        assert_eq!(parsed.as_str(), input);
    }

    #[test]
    fn test_parse_value() {
        let input = "admin";
        let parsed = IniParser::parse(Rule::value, input).unwrap();
        assert_eq!(parsed.as_str(), input);
    }
}
