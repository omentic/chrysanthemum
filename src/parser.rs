use crate::ast::*;

// (λx:T.y): T z
pub fn parse(input: &str) -> Expression {
    return parse_str(input).expect("invalid expression");
}

/// Parses a lambda-calculus-like language into an AST.
pub fn parse_str(input: &str) -> Result<Expression, peg::error::ParseError<peg::str::LineCol>> {
    // this is kinda awful
    // i miss my nim pegs
    peg::parser!{
        grammar lambda() for str {
            rule identifier() -> String
            = i:['a'..='z' | 'A'..='Z' | '0'..='9']+ {
                i.iter().collect::<String>()
            }
            rule constant() -> Expression
            = p:"-"? c:['0'..='9']+ {
                let value = c.iter().collect::<String>().parse::<Value>().unwrap();
                Expression::Constant {
                    term: Term {
                        val: if let Some(_) = p {
                            value.wrapping_neg()
                        } else {
                            value
                        },
                        kind: Type::Empty
                    }
                }
            }
            // fucking awful but i don't know another way
            // k:("empty" / "unit" / etc) returns ()
            // and i can't seem to match and raise a parse error
            // so ¯\_(ツ)_/¯
            rule empty() -> Type = k:"empty" {Type::Empty}
            rule unit() -> Type = k:"unit" {Type::Unit}
            rule boolean() -> Type = k:"bool" {Type::Boolean}
            rule natural() -> Type = k:"nat" {Type::Natural}
            rule integer() -> Type = k:"int" {Type::Integer}
            rule kind() -> Type
             = k:(empty() / unit() / boolean() / natural() / integer()) {
                k
            }
            rule annotation() -> Expression
            = e:(conditional() / abstraction() / application() / constant() / variable()) " "* ":" " "* k:kind() {
                Expression::Annotation {
                    expr: Box::new(e),
                    kind: k
                }
            }
            rule variable() -> Expression
            = v:identifier() {
                Expression::Variable {
                    id: v
                }
            }
            rule abstraction() -> Expression
            = ("λ" / "lambda ") " "* p:identifier() " "* "." " "* f:expression() {
                Expression::Abstraction {
                    param: p,
                    func: Box::new(f)
                }
            }
            // fixme: more cases should parse, but how?
            rule application() -> Expression
            = "(" f:(annotation() / abstraction()) ")" " "* a:expression() {
                Expression::Application {
                    func: Box::new(f),
                    arg: Box::new(a)
                }
            }
            rule conditional() -> Expression
            = "if" " "+ c:expression() " "+ "then" " "+ t:expression() " "+ "else" " "+ e:expression() {
                Expression::Conditional {
                    if_cond: Box::new(c),
                    if_then: Box::new(t),
                    if_else: Box::new(e)
                }
            }
            pub rule expression() -> Expression
            = e:(conditional() / annotation() / abstraction() / application() / constant() / variable()) {
                e
            }
            pub rule ast() -> Vec<Expression>
            = expression() ** ("\n"+)
        }
    }
    // assert_eq!(lambda::expression("(λx:bool.x)").unwrap(), lambda::expression("(λx: bool . x)").unwrap());

    return lambda::expression(input.trim());
}

/// Parses a Nim-like language into an AST.
pub fn parse_file(path: &str) -> Vec<Expression> {
    todo!();
}
