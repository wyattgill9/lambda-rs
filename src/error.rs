#[derive(Debug)]
pub enum ParseError {
    InvalidSyntax(String),
    UnboundVariable(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidSyntax(msg) => write!(f, "Syntax error: {}", msg),
            ParseError::UnboundVariable(var) => write!(f, "Unbound variable: {}", var),
        }
    }
}

impl std::error::Error for ParseError {}
