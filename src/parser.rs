use crate::ast::*;

/// Parses a lambda-calculus-like language into an AST.
pub fn parse_lambda(input: &str) -> Result<Expression, peg::error::ParseError<peg::str::LineCol>> {
    // this is kinda awful, i miss my simple nim pegs
    peg::parser! {
        grammar lambda() for str {
            rule ident() -> String = i:['a'..='z' | 'A'..='Z' | '0'..='9']+ {
                i.iter().collect::<String>()
            }
            rule bool() -> Expression = b:$("true" / "false") {
                match b {
                    "true" => Expression::Constant { term: Term::Boolean(true) },
                    "false" => Expression::Constant { term: Term::Boolean(false) },
                    _ => Expression::Constant { term: Term::Unit() }
                }
            }
            rule num() -> Expression = p:"-"? c:['0'..='9']+ {
                let value = c.iter().collect::<String>().parse::<usize>().unwrap();
                Expression::Constant {
                    term: if let Some(_) = p {
                        Term::Integer(-1 * isize::try_from(value).unwrap())
                    } else {
                        Term::Natural(value)
                    }
                }
            }
            rule cons() -> Expression = c:(bool() / num())
            rule primitive() -> Type
            = k:$("empty" / "unit" / "bool" / "nat" / "int") {
                match k {
                    "empty" => Type::Empty,
                    "unit" => Type::Unit,
                    "bool" => Type::Boolean,
                    "nat" => Type::Natural,
                    "int" => Type::Integer,
                    _ => Type::Empty
                }
            }
            // fixme: brackets are necessary here
            rule function() -> Type = "(" f:kind() " "* "->" " "* t:kind() ")" {
                Type::Function { from: Box::new(f), to: Box::new(t) }
            }
            rule kind() -> Type
             = k:(function() / primitive()) {
                k
            }
            rule ann() -> Expression
            = e:(bracketed() / (cond() / abs() / app() / cons() / var())) " "* ":" " "* k:kind() {
                Expression::Annotation {
                    expr: Box::new(e),
                    kind: k
                }
            }
            rule var() -> Expression
            = v:ident() {
                Expression::Variable {
                    id: v
                }
            }
            rule abs() -> Expression
            = ("λ" / "lambda ") " "* p:ident() " "* "." " "* f:expr() {
                Expression::Abstraction {
                    param: p,
                    func: Box::new(f)
                }
            }
            // fixme: more cases should parse, but how?
            rule app() -> Expression
            = "(" f:expr() ")" " "* a:expr() {
                Expression::Application {
                    func: Box::new(f),
                    arg: Box::new(a)
                }
            }
            rule cond() -> Expression
            = "if" " "+ c:expr() " "+ "then" " "+ t:expr() " "+ "else" " "+ e:expr() {
                Expression::Conditional {
                    if_cond: Box::new(c),
                    if_then: Box::new(t),
                    if_else: Box::new(e)
                }
            }
            rule unbracketed() -> Expression
            = e:(cond() / ann() / abs() / app() / cons() / var()) {
                e
            }
            rule bracketed() -> Expression
            = "(" " "* e:(cond() / ann() / abs() / app() / cons() / var()) " "* ")" {
                e
            }
            pub rule expr() -> Expression
            // what the fuck
            // why doesn't = " "* e:(unbracketed() / bracketed()) " "* work
            = e:(unbracketed() / bracketed()) {
                e
            }
        }
    }
    return lambda::expr(input.trim());
}

