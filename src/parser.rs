use crate::ast::Term;
use crate::error::ParseError;

pub struct Parser {
    input: Vec<char>,
    pos: usize,
    context: Vec<String>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Parser {
            input: input.chars().collect(),
            pos: 0,
            context: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Term, ParseError> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Result<Term, ParseError> {
        self.parse_abs().or_else(|_| self.parse_app())
    }

    fn parse_abs(&mut self) -> Result<Term, ParseError> {
        if self.consume('λ') || self.consume('\\') {
            let var = self.parse_identifier()?;
            self.expect('.')?;
            self.context.push(var);
            let body = self.parse_term()?;
            self.context.pop();
            Ok(Term::Abs(Box::new(body)))
        } else {
            Err(ParseError::InvalidSyntax("Expected lambda".into()))
        }
    }

    fn parse_app(&mut self) -> Result<Term, ParseError> {
        let mut terms = vec![self.parse_atom()?];

        while let Ok(term) = self.parse_atom() {
            terms.push(term);
        }

        let mut app = terms.remove(0);
        for term in terms {
            app = Term::App(Box::new(app), Box::new(term));
        }
        Ok(app)
    }

    fn parse_atom(&mut self) -> Result<Term, ParseError> {
        self.skip_whitespace();
        if self.consume('(') {
            let term = self.parse_term()?;
            self.expect(')')?;
            Ok(term)
        } else if let Ok(var) = self.parse_var() {
            Ok(var)
        } else {
            Err(ParseError::InvalidSyntax("Expected atom".into()))
        }
    }

    fn parse_var(&mut self) -> Result<Term, ParseError> {
        let name = self.parse_identifier()?;
        self.context
            .iter()
            .rev()
            .position(|v| v == &name)
            .map(|idx| Term::Var(idx))
            .ok_or_else(|| ParseError::UnboundVariable(name))
    }

    // helper methods for parsing
    fn consume(&mut self, expected: char) -> bool {
        self.skip_whitespace();
        if self.pos < self.input.len() && self.input[self.pos] == expected {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, expected: char) -> Result<(), ParseError> {
        if self.consume(expected) {
            Ok(())
        } else {
            Err(ParseError::InvalidSyntax(format!(
                "Expected '{}'",
                expected
            )))
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        self.skip_whitespace();
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_alphabetic() {
            self.pos += 1;
        }
        if start == self.pos {
            Err(ParseError::InvalidSyntax("Expected identifier".into()))
        } else {
            Ok(self.input[start..self.pos].iter().collect())
        }
    }

    pub fn parse_multi_line(&mut self) -> Vec<Result<Term, ParseError>> {
        let input = self.input.iter().collect::<String>();
        let mut results = Vec::new();

        for line in input.lines() {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
                continue;
            }

            let mut temp_parser = Parser::new(trimmed_line);
            match temp_parser.parse() {
                Ok(term) => results.push(Ok(term)),
                Err(e) => results.push(Err(e)),
            }
        }

        results
    }

}
