use std::{
    fmt::{self, Display, Formatter},
    writeln,
};

#[derive(Debug)]
pub enum ErrorKind {
    // HtmlError,
    // ParsingError,
    Alerterror,
}

pub struct Errors<'a> {
    kind: ErrorKind,
    message: &'a str,
}

impl<'a> Display for Errors<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind {
            // ErrorKind::HtmlError => writeln!(f, "Invalid Link, {}", self.message),
            // ErrorKind::ParsingError => {
            // writeln!(f, "No css element {} exists in the dom", self.message)
            // }
            ErrorKind::Alerterror => writeln!(f, "Error Sending Email: {}", self.message),
        }
    }
}

impl<'a> Errors<'a> {
    pub fn new(kind: ErrorKind, message: &'a str) -> Errors {
        Errors { kind, message }
    }
}
