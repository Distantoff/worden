use std::{fmt, error};

#[derive(Debug)]
pub struct ParseOrderError {
    pub incorrect_order_expr: String
}

#[derive(Debug)]
enum ModelErrorKind {
    IncorrectOrderString
}

impl error::Error for ParseOrderError { }

impl fmt::Display for ParseOrderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "it is not possible to convert a string {} to an enumeration",
            self.incorrect_order_expr)
    }
}
