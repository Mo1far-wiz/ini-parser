#[cfg(test)]
mod error_tests {
    use ini_parser::{ini_file::IniFile, parser::ParseError};

    #[test]
    fn test_missing_section_brackets() {
        let input = "Section\nkey=value";
        let result = IniFile::from_str(input);
        assert!(result.is_err(), "Expected error when section brackets are missing");
        if let Err(e) = result {
            match e {
                ParseError::PestError(_) => {}, // Expected Pest error
                _ => panic!("Unexpected error type: {:?}", e),
            }
        }
    }

    #[test]
    fn test_missing_equal_sign() {
        let input = "[Section]\nkeyvalue";
        let result = IniFile::from_str(input);
        assert!(result.is_err(), "Expected error when missing equal sign in key-value pair");
        if let Err(e) = result {
            match e {
                ParseError::PestError(_) => {}, // Expected Pest error
                _ => panic!("Unexpected error type: {:?}", e),
            }
        }
    }

    #[test]
    fn test_empty_key() {
        let input = "[Section]\n=value";
        let result = IniFile::from_str(input);
        assert!(result.is_err(), "Expected error when key is empty in key-value pair");
        if let Err(e) = result {
            match e {
                ParseError::PestError(_) => {}, // Expected Pest error
                _ => panic!("Unexpected error type: {:?}", e),
            }
        }
    }

    #[test]
    fn test_no_sections() {
        let input = "key=value\nanother_key=another_value";
        let result = IniFile::from_str(input);
        assert!(result.is_err(), "INI file should have at least one section.");
        if let Err(e) = result {
            match e {
                ParseError::PestError(_) => {}, // Expected Pest error
                _ => panic!("Unexpected error type: {:?}", e),
            }
        }
    }
}
