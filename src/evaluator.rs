use crate::ast::Term;

pub fn evaluate(term: Term) -> Term {
    let mut current = term;
    loop {
        let next = beta_reduce(&current);
        if next == current {
            return current;
        }
        current = next;
    }
}

fn beta_reduce(term: &Term) -> Term {
    match term {
        Term::App(t1, t2) => match beta_reduce(t1) {
            Term::Abs(body) => substitute(&body, &beta_reduce(t2)),
            reduced_t1 => Term::App(Box::new(reduced_t1), Box::new(beta_reduce(t2))),
        },
        Term::Abs(body) => Term::Abs(Box::new(beta_reduce(body))),
        Term::Var(_) => term.clone(),
    }
}

fn substitute(body: &Term, replacement: &Term) -> Term {
    substitute_helper(body, replacement, 0)
}

fn substitute_helper(term: &Term, replacement: &Term, depth: usize) -> Term {
    match term {
        Term::Var(i) => {
            if *i == depth {
                shift(replacement, depth, 0)
            } else {
                Term::Var(*i)
            }
        }
        Term::Abs(body) => Term::Abs(Box::new(substitute_helper(body, replacement, depth + 1))),
        Term::App(t1, t2) => Term::App(
            Box::new(substitute_helper(t1, replacement, depth)),
            Box::new(substitute_helper(t2, replacement, depth)),
        ),
    }
}

fn shift(term: &Term, cutoff: usize, amount: isize) -> Term {
    match term {
        Term::Var(i) => {
            if *i >= cutoff {
                Term::Var((*i as isize + amount) as usize)
            } else {
                Term::Var(*i)
            }
        }
        Term::Abs(body) => Term::Abs(Box::new(shift(body, cutoff + 1, amount))),
        Term::App(t1, t2) => Term::App(
            Box::new(shift(t1, cutoff, amount)),
            Box::new(shift(t2, cutoff, amount)),
        ),
    }
}
