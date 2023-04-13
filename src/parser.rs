use crate::ast::*;
use multipeek::multipeek;

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

const operators: [char; 17] =
    ['=', '+', '-', '*', '/', '<', '>', '@', '$', '~', '&', '%', '|', '!', '?', '^', '\\'];
const brackets: [char; 6] = ['(', ')', '{', '}', '[', ']'];
const special: [char; 7] = ['.', ',', ':', ';', '`', '\'', '"'];
const keywords: [&'static str; 3] = ["if", "else", "func"];

pub enum Token {
    Operator(String),
    Keyword(String),
    Separator(String),
    Identifier(String),
    Value(String),
    Char(char),
    String(String),
    Comment(String),
    Token(String), // catch-all
    ScopeBegin, // {
    ScopeEnd,   // }
    ExprEnd,    // ;
}

/// Properly lexes a whitespace-oriented language into a series of tokens.
pub fn lex(input: &str) -> Result<Vec<Token>, &'static str> {
    enum State {
        Default,
        Char,
        String,
        MultiLineString,
        Comment,
    }
    struct Indentation {
        blank: bool,    // is the line entirely whitespace so far?
        level: usize,   // current indentation level
        count: usize,   // current whitespace count
    }

    let mut state = State::Default;
    let mut indent = Indentation { blank: true, level: 0, count: 0 };
    let mut buffer = String::new();
    let mut result = Vec::new();

    let mut input = multipeek(input.chars()); // multipeek my beloved
    while let Some(c) = input.next() {
        match state {
            State::Default => match c {
                ' ' if indent.blank => indent.count += 1,
                ' ' if buffer.len() > 0 => {
                    result.push(parse_token(&buffer)?);
                    buffer.clear();
                },
                ' ' => todo!(),
                '\n' => todo!(),
                '\t' => return Err("Tabs are not supported!"),
                '\'' => {
                    result.push(parse_token(&buffer)?);
                    buffer.clear();
                    if input.peek_nth(0) == Some(&'\\') || input.peek_nth(1) == Some(&'\'') {
                        state = State::Char;
                    } else {
                        result.push(Token::Separator("'".to_string()));
                    }
                },
                '"' => {
                    if input.peek_nth(0) == Some(&'\"') && input.peek_nth(1) == Some(&'\"') {
                        state = State::MultiLineString;
                        input.next();
                        input.next();
                    } else {
                        state = State::String;
                    }
                },
                '#' => {
                    state = State::Comment;
                    result.push(parse_token(&buffer)?);
                    buffer.clear();
                },
                _ if brackets.contains(&c) || special.contains(&c) => {
                    if buffer.len() > 0 {
                        result.push(parse_token(&buffer)?);
                        buffer.clear();
                    }
                    result.push(Token::Separator(c.to_string()));
                    if indent.blank {
                        indent.blank = false;
                    }
                }
                _ if indent.blank => {
                    indent.blank = false;
                    // indentation check
                    todo!();
                    buffer.push(c);
                }
                _ => buffer.push(c)
            },
            State::Char => match c {
                '\\' => {
                    match input.next() {
                        Some('\\') => result.push(Token::Char('\\')),
                        Some('0') => result.push(Token::Char('\0')),
                        Some('n') => result.push(Token::Char('\n')),
                        Some('r') => result.push(Token::Char('\r')),
                        Some('t') => result.push(Token::Char('\t')),
                        Some('\"') => result.push(Token::Char('\"')),
                        Some('\'') => result.push(Token::Char('\'')),
                        _ => return Err("Invalid string escape sequence!"),
                    }
                    state = State::Default;
                    if input.next() != Some('\'') {
                        return Err("Invalid character sequence!")
                    }
                },
                '\'' => {
                    result.push(Token::Char('\0'));
                    state = State::Default;
                }
                _ => {
                    result.push(Token::Char(c));
                    state = State::Default;
                    if input.next() != Some('\'') {
                        return Err("Invalid character sequence!")
                    }
                }
            },
            State::String => match c {
                '\\' => match input.next() {
                    Some('\\') => buffer.push('\\'),
                    Some('0') => buffer.push('\0'),
                    Some('n') => buffer.push('\n'),
                    Some('r') => buffer.push('\r'),
                    Some('t') => buffer.push('\t'),
                    Some('\"') => buffer.push('\"'),
                    Some('\'') => buffer.push('\''),
                    _ => return Err("Invalid string escape sequence!"),
                },
                '\"' => {
                    state = State::Default;
                    result.push(Token::String(buffer.to_string()));
                    buffer.clear();
                }
                _ => buffer.push(c)
            },
            State::MultiLineString => match c {
                '\"' if input.peek_nth(0) == Some(&'"') && input.peek_nth(1) == Some(&'"') => {
                    state = State::Default;
                    result.push(Token::String(buffer.to_string()));
                    buffer.clear();
                    input.next();
                    input.next();
                },
                _ => buffer.push(c)
            },
            State::Comment => match c {
                '\n' => {
                    state = State::Default;
                    result.push(Token::Comment(buffer.to_string()));
                },
                _ => buffer.push(c)
            },
        }
    }
    return Ok(result);
}

fn parse_token(token: &str) -> Result<Token, &'static str> {
    if keywords.contains(&token) {
        Ok(Token::Keyword(token.to_string()))
    } else if is_operator(token) {
        Ok(Token::Operator(token.to_string()))
    } else if is_value(token) {
        Ok(Token::Value(token.to_string()))
    } else if is_identifier(token) {
        Ok(Token::Identifier(token.to_string()))
    } else {
        Err("Could not parse token!")
    }
}

fn is_operator(token: &str) -> bool {
    for c in token.chars() {
        if !operators.contains(&c) {
            return false;
        }
    }
    return true;
}

fn is_value(token: &str) -> bool {
    if token == "true" || token == "false" {
        return true;
    }
    // fixme: hex literals etc
    for c in token.chars() {
        // note size annotations are separately lexed
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

fn is_identifier(token: &str) -> bool {
    if let Some(c) = token.chars().nth(0) {
        if c.is_numeric() || c == '_' {
            return false;
        }
    }
    for c in token.chars() {
        if !c.is_alphanumeric() && c != '_' {
            return false;
        }
    }
    return true;
}
