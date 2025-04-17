use std::collections::HashMap;

use crate::symbols::chars::DOUBLE_QUOTE;
use crate::symbols::chars::EQUALS;
use crate::symbols::chars::LEFT_BRACKET;
use crate::symbols::chars::RIGHT_BRACKET;
use crate::symbols::strings::HYPHEN_MINUS;

pub fn is_exec_form(arguments: &[String]) -> bool {
    arguments
        .first()
        .map(|s| s.starts_with(LEFT_BRACKET))
        .unwrap_or(false)
        && arguments
            .last()
            .map(|s| s.ends_with(RIGHT_BRACKET))
            .unwrap_or(false)
}

pub fn get_options_from(arguments: Vec<String>) -> (HashMap<String, String>, Vec<String>) {
    let mut options = HashMap::new();
    let mut remaining = Vec::new();

    for arg in &arguments {
        if let Some(stripped) = arg.strip_prefix(HYPHEN_MINUS) {
            if let Some((key, value)) = stripped.split_once(EQUALS) {
                options.insert(key.to_owned(), value.to_owned());
                continue;
            }
        }
        break;
    }

    remaining.extend(arguments.iter().skip(options.len()).cloned());

    (options, remaining)
}

pub fn process_key_value_pairs(arguments: &[String]) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let mut last_key: Option<String> = None;

    for arg in arguments {
        let (key, value) = match arg.split_once(EQUALS) {
            Some((key, value)) => (key.to_owned(), value.to_owned()),
            None => {
                // try to append the value to the last key
                if last_key.is_none() {
                    continue;
                }

                let last_key = last_key.unwrap();
                let last_value = result.get(&last_key).unwrap();

                let new_value = format!("{} {}", last_value, arg);

                (last_key, new_value)
            }
        };

        let value = value
            .trim_start_matches(DOUBLE_QUOTE)
            .trim_end_matches(DOUBLE_QUOTE);

        result.insert(key.to_owned(), value.to_owned());
        last_key = Some(key);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_exec_form() {
        assert!(is_exec_form(&[String::from("[\"/usr/bin/executable\"]")]));
        assert!(is_exec_form(&[String::from(
            "[\"/usr/bin/executable\", \"arg1\"]"
        )]));
        assert!(is_exec_form(&[String::from(
            "[\"/usr/bin/executable\", \"arg1\", \"arg2\"]"
        )]));
    }

    #[test]
    fn test_is_not_exec_form() {
        assert!(!is_exec_form(&[String::from("/usr/bin/executable")]));
        assert!(!is_exec_form(&[String::from("/usr/bin/executable arg1")]));
        assert!(!is_exec_form(&[String::from(
            "/usr/bin/executable arg1 arg2"
        )]));
    }

    #[test]
    fn test_get_options_from_arguments() {
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
    fn test_get_options_from_no_arguments() {
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
    fn test_get_options_from_no_options() {
        let arguments = vec![String::from("arg1"), String::from("arg2")];
        let (options, remaining) = get_options_from(arguments);

        assert!(options.is_empty());
        assert_eq!(remaining, vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_get_options_from_flags() {
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

    #[test]
    fn test_process_key_value_pairs_without_spaces() {
        let arguments = vec![
            String::from("key1=\"value1\""),
            String::from("key2=\"value2\""),
        ];
        let result = process_key_value_pairs(&arguments);

        assert_eq!(result.get("key1"), Some(String::from("value1")).as_ref());
        assert_eq!(result.get("key2"), Some(String::from("value2")).as_ref());
    }

    #[test]
    fn test_process_key_value_pairs_without_spaces_and_quotes() {
        let arguments = vec![String::from("key1=value1"), String::from("key2=value2")];
        let result = process_key_value_pairs(&arguments);

        assert_eq!(result.get("key1"), Some(String::from("value1")).as_ref());
        assert_eq!(result.get("key2"), Some(String::from("value2")).as_ref());
    }

    #[test]
    fn test_process_key_value_pairs_with_spaces_and_quotes() {
        let arguments = vec![
            String::from("key1=\"value1"),
            String::from("with"),
            String::from("spaces\""),
            String::from("key2=\"value2"),
            String::from("with"),
            String::from("spaces\""),
        ];
        let result = process_key_value_pairs(&arguments);

        assert_eq!(
            result.get("key1"),
            Some(String::from("value1 with spaces")).as_ref()
        );
        assert_eq!(
            result.get("key2"),
            Some(String::from("value2 with spaces")).as_ref()
        );
    }
}
