use crate::symbols::chars::DOUBLE_QUOTE;
use crate::symbols::strings::EMPTY;

pub trait Quoter {
    fn dequote(&self) -> String;
    fn enquote(&self) -> String;
}

impl Quoter for String {
    fn dequote(&self) -> String {
        self.replace(DOUBLE_QUOTE, EMPTY)
    }

    fn enquote(&self) -> String {
        format!("\"{self}\"")
    }
}
