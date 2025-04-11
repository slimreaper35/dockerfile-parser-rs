use std::collections::HashMap;

use crate::symbols::chars::DOUBLE_QUOTE;
use crate::symbols::chars::EQUALS;
use crate::symbols::chars::LEFT_BRACKET;
use crate::symbols::chars::RIGHT_BRACKET;

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

pub fn process_key_value_pairs(arguments: &[String]) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let mut last_key: Option<String> = None;

    for arg in arguments {
        let (key, value) = match arg.split_once(EQUALS) {
            Some((key, value)) => (key.to_string(), value.to_string()),
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

        result.insert(key.to_string(), value.to_string());
        last_key = Some(key);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_exec_form_pass() {
        assert!(is_exec_form(&[String::from("[\"/usr/bin/executable\"]")]));
        assert!(is_exec_form(&[String::from(
            "[\"/usr/bin/executable\", \"arg1\"]"
        )]));
        assert!(is_exec_form(&[String::from(
            "[\"/usr/bin/executable\", \"arg1\", \"arg2\"]"
        )]));
        assert!(!is_exec_form(&[String::from("/usr/bin/executable")]));
        assert!(!is_exec_form(&[String::from("/usr/bin/executable arg1")]));
        assert!(!is_exec_form(&[String::from(
            "/usr/bin/executable arg1 arg2"
        )]));
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
