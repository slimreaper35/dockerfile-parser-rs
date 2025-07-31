pub mod chars {
    pub const BACKSLASH: char = '\\';
    pub const COLON: char = ':';
    pub const COMMA: char = ',';
    pub const SINGLE_QUOTE: char = '\'';
    pub const DOUBLE_QUOTE: char = '"';
    pub const EQUALS: char = '=';
    pub const HASHTAG: char = '#';
    pub const LEFT_BRACKET: char = '[';
    pub const RIGHT_BRACKET: char = ']';
    pub const SPACE: char = ' ';
}

pub mod strings {
    pub const EMPTY: &str = "";
    pub const HEREDOC_END: &str = "EOF";
    pub const HEREDOC_NEWLINE: &str = "--NEWLINE--";
    pub const HEREDOC_START: &str = "<<EOF";
    pub const HYPHEN_MINUS: &str = "--";
}
