use crate::symbols::chars::DOUBLE_QUOTE;

pub trait Quoter {
    fn dequote(&self) -> String;
    fn enquote(&self) -> String;
}

impl Quoter for String {
    fn dequote(&self) -> String {
        self.trim_start_matches(DOUBLE_QUOTE)
            .trim_end_matches(DOUBLE_QUOTE)
            .to_owned()
    }

    fn enquote(&self) -> String {
        format!("\"{self}\"")
    }
}
