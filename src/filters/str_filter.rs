use {
    bet::BeTree,
    lazy_regex::regex::{self, Regex},
    thiserror::Error,
};

#[derive(Debug, Error)]
pub enum ParseStrFilterError {

    #[error("invalid pattern {0:?} : {1}")]
    InvalidPattern(String, String),

    #[error("invalid regex {0:?}")]
    InvalidRegex(#[from] regex::Error),
}

/// Query operators.
/// `And` and `Or` are binary while `Not` is unary.
#[derive(Debug, Clone, Copy, PartialEq)]
enum BoolOperator {
    And,
    Or,
    Not,
}
impl BoolOperator {
    fn eval(self, a: bool, b: Option<bool>) -> bool {
        match (self, b) {
            (Self::And, Some(b)) => a & b,
            (Self::Or, Some(b)) => a | b,
            (Self::Not, None) => !a,
            _ => {
                panic!("unexpected operator or operands"); // parsing failed
            }
        }
    }
    /// tell whether we can skip evaluating the second operand
    fn short_circuit(self, a: bool) -> bool {
        match (self, a) {
            (Self::And, false) => true,
            (Self::Or, true) => true,
            _ => false,
        }
    }
}

/// a filter for strings
#[derive(Debug)]
pub struct StrFilter {
    expr: BeTree<BoolOperator, Regex>,
}

fn invalid(pattern: &str, reason: &str) -> Result<StrFilter, ParseStrFilterError> {
    Err(ParseStrFilterError::InvalidPattern(pattern.to_owned(), reason.to_owned()))
}

impl StrFilter {
    pub fn new(pattern: &str) -> Result<Self, ParseStrFilterError> {
        if pattern.contains(',') {
            Self::with_comma_syntax(pattern)
        } else {
            Self::with_be_syntax(pattern)
        }
    }
    /// parse a filter defined with a rich binary expression syntax (parentheses, &, |, etc.)
    /// a sequence of patterns, each one with an optional NOT before.
    ///
    /// Example: ̀ dystroy & !miaou`
    pub fn with_be_syntax(pattern: &str) -> Result<Self, ParseStrFilterError> {
        let mut expr = BeTree::new();
        let chars: Vec<char> = pattern.chars().collect();
        for i in 0..chars.len() {
            match chars[i] {
                '(' if chars.get(i + 1) == Some(&' ') => {
                    if expr.accept_opening_par() {
                        expr.open_par();
                    } else {
                        return invalid(pattern, "unexpected opening parenthesis");
                    }
                }
                ')' if i > 0 && chars.get(i - 1) == Some(&' ') => {
                    if expr.accept_closing_par() {
                        expr.close_par();
                    } else {
                        println!("expr: {:#?}", &expr);
                        return invalid(pattern, "unexpected closing parenthesis");
                    }
                }
                '&' if chars.get(i + 1) == Some(&' ') => {
                    if expr.accept_binary_operator() {
                        expr.push_operator(BoolOperator::And);
                    } else {
                        return invalid(pattern, "unexpected '&'");
                    }
                }
                '|' if chars.get(i + 1) == Some(&' ') => {
                    if expr.accept_binary_operator() {
                        expr.push_operator(BoolOperator::Or);
                    } else {
                        return invalid(pattern, "unexpected '|'");
                    }
                }
                '!' if expr.accept_unary_operator() => {
                    expr.push_operator(BoolOperator::Not);
                }
                ' ' => {}
                c => {
                    expr.mutate_or_create_atom(String::new).push(c);
                }
            }
        }
        let expr = expr.try_map_atoms(|s| Regex::new(s))?;
        Ok(Self { expr })
    }
    /// parse a filter defined with the comma syntax, ie a AND on
    /// a sequence of patterns, each one with an optional NOT before.
    ///
    /// Example: ̀ dystroy,!miaou`
    pub fn with_comma_syntax(pattern: &str) -> Result<Self, ParseStrFilterError> {
        let mut expr = BeTree::new();
        let atoms = pattern.split(',').map(|s| s.trim());
        for atom in atoms {
            if atom.is_empty() {
                return invalid(pattern, "empty token");
            }
            if !expr.is_empty() {
                expr.push_operator(BoolOperator::And);
            }
            if let Some(atom) = atom.strip_prefix('!') {
                expr.push_operator(BoolOperator::Not);
                expr.push_atom(Regex::new(atom)?);
            } else {
                expr.push_atom(Regex::new(atom)?);
            }
        }
        Ok(Self { expr })
    }
    pub fn accepts(&self, candidate: &str) -> bool {
        self.expr
            .eval(
                |r| r.is_match(candidate),
                |op, a, b| op.eval(a, b),
                |op, &a| op.short_circuit(a),
            )
            .unwrap_or_else(|| {
                println!("unexpected lack of expr result on {:?}", candidate);
                false
            })
    }
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod str_filter_tests {

    use super::*;

    #[test]
    fn test_comma() {
        let f = StrFilter::new("dystroy,!miaou").unwrap();
        assert_eq!(f.accepts("a/dystroy/b"), true);
        assert_eq!(f.accepts("a/miaou/b"), false);
        assert_eq!(f.accepts("a/miaou/dystroy"), false);
    }

    #[test]
    fn test_comma_regex() {
        let f = StrFilter::new(r"dystroy,!m\w{3}u").unwrap();
        assert_eq!(f.accepts("a/dystroy/b"), true);
        assert_eq!(f.accepts("a/miaou/b"), false);
        assert_eq!(f.accepts("a/miaou/dystroy"), false);
    }

    #[test]
    fn test_be() {
        let f = StrFilter::new("dystroy & !( miaou | blog )").unwrap();
        assert_eq!(f.accepts("a/dystroy/b"), true);
        assert_eq!(f.accepts("dystroy/miaou/b"), false);
        assert_eq!(f.accepts("a/blog/dystroy"), false);
        assert_eq!(f.accepts("a/blog/"), false);
    }

    #[test]
    fn test_be_regex() {
        let f = StrFilter::new(r"^/dystroy & !( m\w{3}u | blog )").unwrap();
        assert_eq!(f.accepts("/a/dystroy/b"), false);
        assert_eq!(f.accepts("/dystroy/b"), true);
        assert_eq!(f.accepts("/dystroy/mieou/b"), false);
        assert_eq!(f.accepts("/dystroy/mieaou/b"), true);
        assert_eq!(f.accepts("/z/dystroy/mieaou/b"), false);
        assert_eq!(f.accepts("/a/blog/dystroy"), false);
        assert_eq!(f.accepts("/a/blog/"), false);
    }

}
