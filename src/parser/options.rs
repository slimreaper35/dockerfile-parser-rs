use crate::symbols::chars::EQUALS;
use crate::symbols::strings::HYPHEN_MINUS;

use std::collections::HashMap;

pub fn get_options_from(arguments: Vec<String>) -> (HashMap<String, String>, Vec<String>) {
    let mut options = HashMap::new();
    let mut remaining = Vec::new();

    for arg in &arguments {
        if let Some(stripped) = arg.strip_prefix(HYPHEN_MINUS) {
            if let Some((key, value)) = stripped.split_once(EQUALS) {
                options.insert(key.to_string(), value.to_string());
                continue;
            }
        }
        break;
    }

    remaining.extend(arguments.iter().skip(options.len()).cloned());

    (options, remaining)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_options() {
        let arguments = vec![
            String::from("--option1=value1"),
            String::from("--option2=value2"),
            String::from("arg1"),
            String::from("arg2"),
        ];
        let (options, remaining) = get_options_from(arguments);
        assert_eq!(
            options.get("option1"),
            Some(String::from("value1")).as_ref()
        );
        assert_eq!(
            options.get("option2"),
            Some(String::from("value2")).as_ref()
        );
        assert_eq!(remaining, vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_parse_options_no_arguments() {
        let arguments = vec![
            String::from("--option1=value1"),
            String::from("--option2=value2"),
        ];
        let (options, remaining) = get_options_from(arguments);
        assert_eq!(
            options.get("option1"),
            Some(String::from("value1")).as_ref()
        );
        assert_eq!(
            options.get("option2"),
            Some(String::from("value2")).as_ref()
        );
        assert!(remaining.is_empty());
    }

    #[test]
    fn test_parse_options_no_options() {
        let arguments = vec![String::from("arg1"), String::from("arg2")];
        let (options, remaining) = get_options_from(arguments);
        assert!(options.is_empty());
        assert_eq!(remaining, vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_parse_options_no_equals() {
        let arguments = vec![
            String::from("--option1"),
            String::from("--option2"),
            String::from("arg1"),
            String::from("arg2"),
        ];
        let (options, remaining) = get_options_from(arguments);
        assert_eq!(options.get("option1"), None);
        assert_eq!(options.get("option2"), None);
        assert_eq!(remaining, vec!["--option1", "--option2", "arg1", "arg2"]);
    }
}
