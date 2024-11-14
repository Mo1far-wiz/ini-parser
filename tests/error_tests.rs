#[cfg(test)]
mod error_tests {
    use ini_parser::parser::{IniParser, Rule};
    use pest::Parser;

    #[test]
    fn test_missing_section_brackets() {
        let input = "Section\nkey=value";
        let result = IniParser::parse(Rule::ini_file, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_equal_sign() {
        let input = "[Section]\nkeyvalue";
        let result = IniParser::parse(Rule::ini_file, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_key() {
        let input = "[Section]\n=value";
        let result = IniParser::parse(Rule::ini_file, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_no_sections() {
        let input = "key=value\nanother_key=another_value";
        let result = IniParser::parse(Rule::ini_file, input);
        assert!(result.is_err(), "INI file should have at least one section");
    }
}
