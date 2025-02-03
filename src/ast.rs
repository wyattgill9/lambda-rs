#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Var(usize),
    Abs(Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Term::Var(i) => write!(f, "{}", i),
            Term::Abs(body) => write!(f, "λ.{}", body),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
        }
    }
}
