use crate::symbols::chars::EQ;
use crate::symbols::chars::SPACE;
use crate::symbols::strings::HYPHEN_MINUS;

use std::collections::HashMap;

pub fn parse_options(args: &str) -> (HashMap<String, String>, &str) {
    let mut options = HashMap::new();
    let mut remaining = args;

    while let Some(option_start_position) = remaining.find(HYPHEN_MINUS) {
        // split at the start of the option
        let after_option = remaining.split_at(option_start_position).1;

        let option_end_position = after_option.find(SPACE).unwrap_or(after_option.len());
        let new_option = &after_option[..option_end_position];
        // not a real Docker option
        if !new_option.contains(EQ) {
            break;
        }

        if let Some((key, value)) = new_option[HYPHEN_MINUS.len()..].split_once(EQ) {
            options.insert(key.to_string(), value.to_string());
        }

        // go to the next option
        remaining = after_option[option_end_position..].trim_start();
    }

    (options, remaining.trim())
}
