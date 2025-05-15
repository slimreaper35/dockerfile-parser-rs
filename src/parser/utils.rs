use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::symbols::chars::COMMA;
use crate::symbols::chars::DOUBLE_QUOTE;
use crate::symbols::chars::EQUALS;
use crate::symbols::chars::LEFT_BRACKET;
use crate::symbols::chars::RIGHT_BRACKET;
use crate::symbols::strings::EMPTY;
use crate::symbols::strings::HYPHEN_MINUS;

pub fn is_exec_form(arguments: &[String]) -> bool {
    matches!(
        (arguments.first(), arguments.last()),
        (Some(first), Some(last)) if first.starts_with(LEFT_BRACKET) && last.ends_with(RIGHT_BRACKET)
    )
}

pub fn clean_shell_form(arguments: &[String]) -> Vec<String> {
    arguments
        .iter()
        .map(|arg| arg.replace(DOUBLE_QUOTE, EMPTY))
        .filter(|arg| !arg.is_empty())
        .collect()
}

pub fn clean_exec_form(arguments: &[String]) -> Vec<String> {
    arguments
        .iter()
        .map(|arg| {
            arg.trim_start_matches(LEFT_BRACKET)
                .trim_start_matches(COMMA)
                .trim_end_matches(RIGHT_BRACKET)
                .trim_end_matches(COMMA)
                .replace(DOUBLE_QUOTE, EMPTY)
        })
        .filter(|arg| !arg.is_empty())
        .collect()
}

pub fn get_options_from(arguments: &[String]) -> (HashMap<String, String>, Vec<String>) {
    let mut options = HashMap::new();
    let mut remaining = Vec::new();

    // some options can be passed multiple times
    let mut options_counter = 0;

    for arg in arguments {
        if let Some(stripped) = arg.strip_prefix(HYPHEN_MINUS) {
            options_counter += 1;

            if let Some((key, value)) = stripped.split_once(EQUALS) {
                options.insert(key.to_owned(), value.to_owned());
            } else {
                // some options can have default values
                options.insert(stripped.to_owned(), EMPTY.to_owned());
            }
        } else {
            break;
        }
    }

    remaining.extend(arguments.iter().skip(options_counter).cloned());

    (options, remaining)
}

pub fn process_key_value_pairs(arguments: &[String]) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();
    let mut last_key: Option<String> = None;

    for arg in arguments {
        let (key, value) = if let Some((key, value)) = arg.split_once(EQUALS) {
            (key.to_owned(), value.to_owned())
        } else {
            // try to append the value to the last key
            if last_key.is_none() {
                continue;
            }

            let last_key = last_key.unwrap();
            let last_value = &result[&last_key];

            let new_value = format!("{last_value} {arg}");

            (last_key, new_value)
        };

        result.insert(
            key.clone(),
            value
                .trim_start_matches(DOUBLE_QUOTE)
                .trim_end_matches(DOUBLE_QUOTE)
                .to_owned(),
        );
        last_key = Some(key);
    }

    result
}

pub fn process_optional_key_value_pairs(arguments: &[String]) -> BTreeMap<String, Option<String>> {
    let mut result = BTreeMap::new();

    for arg in arguments {
        let (key, value) = match arg.split_once(EQUALS) {
            Some((key, value)) => (
                key.to_owned(),
                Some(
                    value
                        .trim_start_matches(DOUBLE_QUOTE)
                        .trim_end_matches(DOUBLE_QUOTE)
                        .to_owned(),
                ),
            ),
            // ignore multi-word default values
            None => (arg.to_owned(), None),
        };
        result.insert(key, value);
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
    fn test_clean_shell_form() {
        let shell_form = String::from("echo \"Hello, World!\"");
        let arguments: Vec<String> = shell_form.split_whitespace().map(String::from).collect();
        let cleaned = clean_shell_form(&arguments);

        assert_eq!(
            cleaned,
            vec![
                String::from("echo"),
                String::from("Hello,"),
                String::from("World!")
            ]
        );
    }

    #[test]
    fn test_clean_exec_form() {
        let exec_form = String::from("[\"/usr/bin/executable\", \"arg1\", \"arg2\"]");
        let arguments: Vec<String> = exec_form.split_whitespace().map(String::from).collect();
        let cleaned = clean_exec_form(&arguments);

        assert_eq!(
            cleaned,
            vec![
                String::from("/usr/bin/executable"),
                String::from("arg1"),
                String::from("arg2"),
            ]
        );
    }

    #[test]
    fn test_clean_exec_form_with_spaces() {
        let exec_form = String::from("[\"/usr/bin/executable\", \"arg1 with spaces\", \"arg2\"]");
        let arguments: Vec<String> = exec_form.split_whitespace().map(String::from).collect();
        let cleaned = clean_exec_form(&arguments);

        assert_eq!(
            cleaned,
            vec![
                String::from("/usr/bin/executable"),
                String::from("arg1"),
                String::from("with"),
                String::from("spaces"),
                String::from("arg2"),
            ]
        );
    }

    #[test]
    fn test_get_options_from_arguments() {
        let arguments = vec![
            String::from("--option1=value1"),
            String::from("--option2=value2"),
            String::from("arg1"),
            String::from("arg2"),
        ];
        let (options, remaining) = get_options_from(&arguments);

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
        let (options, remaining) = get_options_from(&arguments);

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
        let (options, remaining) = get_options_from(&arguments);

        assert!(options.is_empty());
        assert_eq!(remaining, vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_get_options_from_no_equals() {
        let arguments = vec![
            String::from("--option1"),
            String::from("--option2"),
            String::from("arg1"),
            String::from("arg2"),
        ];
        let (options, remaining) = get_options_from(&arguments);

        assert_eq!(options.get("option1"), Some(String::from("")).as_ref());
        assert_eq!(options.get("option2"), Some(String::from("")).as_ref());
        assert_eq!(remaining, vec!["arg1", "arg2"]);
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

    #[test]
    fn test_process_optional_key_value_pairs_without_quotes() {
        let arguments = vec![String::from("key1=value1"), String::from("key2")];
        let result = process_optional_key_value_pairs(&arguments);

        assert_eq!(
            result.get("key1"),
            Some(Some(String::from("value1"))).as_ref()
        );
        assert_eq!(result.get("key2"), Some(None).as_ref());
    }

    #[test]
    fn test_process_optional_key_value_pairs_with_quotes() {
        let arguments = vec![String::from("key1=\"value1\""), String::from("key2")];
        let result = process_optional_key_value_pairs(&arguments);

        assert_eq!(
            result.get("key1"),
            Some(Some(String::from("value1"))).as_ref()
        );
        assert_eq!(result.get("key2"), Some(None).as_ref());
    }
}
