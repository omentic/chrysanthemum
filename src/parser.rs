use crate::ast::*;

// (λx:T.y): T z
pub fn parse(input: &str) -> Expression {
    match parse_lambda(input) {
        Ok(expr) => return expr,
        Err(e) => println!("invalid expression! {:?}", e)
    }
    return Expression::Constant { term: Term::Unit() };
}

/// Parses a Nim-like language into an AST.
pub fn parse_file(path: &str) -> Vec<Expression> {
    match std::fs::read_to_string(path) {
        Ok(file) => match lex(&file) {
            Ok(input) => match parse_lang(&input) {
                Ok(expr) => return expr,
                Err(e) => println!("failed to parse file! {:?}", e),
            },
            Err(e) => println!("failed to lex file! {:?}", e),
        },
        Err(e) => println!("failed to read file! {:?}", e),
    }
    return Vec::new();
}

/// Parses a lambda-calculus-like language into an AST.
pub fn parse_lambda(input: &str) -> Result<Expression, peg::error::ParseError<peg::str::LineCol>> {
    // this is kinda awful, i miss my simple nim pegs
    peg::parser! {
        grammar lambda() for str {
            rule ident() -> String
            = i:['a'..='z' | 'A'..='Z' | '0'..='9']+ {
                i.iter().collect::<String>()
            }
            rule cons() -> Expression
            = p:"-"? c:['0'..='9']+ {
                let value = c.iter().collect::<String>().parse::<usize>().unwrap();
                Expression::Constant {
                    term: if let Some(_) = p {
                        Term::Integer(-1 * isize::try_from(value).unwrap())
                    } else {
                        Term::Natural(value)
                    }
                }
            }
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
            // pub rule ast() -> Vec<Expression>
            // = expr() ** ("\n"+)
        }
    }
    return lambda::expr(input.trim());
}

/// Converts a whitespace-indented language into a regular bracketed language for matching with PEGs
/// Then, tokens are known to be separated by [\n ]+ (except strings. problem for later.)
pub fn lex(input: &str) -> Result<String, &'static str> {
    #[derive(Eq, PartialEq)]
    enum Previous {
        Start,
        Block,
        Line,
    }
    struct State {
        blank: bool,    // is the line entirely whitespace so far?
        level: usize,   // current indentation level
        count: usize,   // current whitespace count
        previous: Previous,
        comment: bool   // is the current line a comment?
    }
    let indent_size: usize = 2;

    let mut state = State { blank: true, level: 0, count: 0, previous: Previous::Start, comment: false };
    let mut buffer = String::new();
    let mut result = String::new();

    // wow lexers are hard
    for c in input.chars() {
        match c {
            '\n' => {
                if !buffer.is_empty() {
                    if state.count == state.level {
                        if state.previous != Previous::Start {
                            result.push(';');
                            result.push('\n');
                        }
                        state.previous = Previous::Line;
                    } else if state.level + indent_size == state.count {
                        result.push(' ');
                        result.push('{');
                        result.push('\n');
                        state.level = state.count;
                        state.previous = Previous::Line;
                    } else if state.count > state.level + indent_size {
                        return Err("invalid jump in indentation");
                    } else if state.count % indent_size != 0 {
                        return Err("incorrect indentation offset, must be a multiple of indent_size");
                    } else if state.level > state.count {
                        while state.level > state.count {
                            if state.previous == Previous::Line {
                                result.push(';');
                            }
                            state.level -= indent_size;
                            result.push('\n');
                            result.push_str(&" ".repeat(state.level));
                            result.push('}');
                            result.push(';');
                            state.previous = Previous::Block;
                        }
                        result.push('\n');
                    } else {
                        return Err("unknown indentation error");
                    }

                    result.push_str(&" ".repeat(state.count));
                    result.push_str(&buffer);

                    state.count = 0;
                    state.comment = false;
                    buffer.clear();
                }
                state.blank = true;
            },
            ' ' if state.blank => {
                state.count += 1;
            },
            '#' => {
                state.blank = false;
                state.comment = true;
            },
            _ => {
                if state.blank {
                    state.blank = false;
                }
                if !state.comment {
                    buffer.push(c);
                }
            },
        }
    }
    if state.previous == Previous::Line {
        result.push(';');
    }
    while state.level != 0 {
        state.level -= 2;
        result.push('\n');
        result.push_str(&" ".repeat(state.level));
        result.push('}');
        result.push(';');
    }
    return Ok(result);
}

/// Parses a simple language with bracket-based indentation and end-of-term semicolons.
/// The lex() function can turn an indentation-based language into a language recognizable by this.
#[allow(unused_variables)]
pub fn parse_lang(input: &str) -> Result<Vec<Expression>, peg::error::ParseError<peg::str::LineCol>> {
    peg::parser! {
        grammar puck() for str {
            // whitespace
            rule w() = ("\n" / " ")+
            // identifiers
            rule ident() -> String = i:['a'..='z' | 'A'..='Z' | '0'..='9']+ {
                i.iter().collect::<String>()
            }
            // constants
            rule cons() -> Expression = p:"-"? c:['0'..='9']+ {
                let value = c.iter().collect::<String>().parse::<usize>().unwrap();
                Expression::Constant {
                    term: if let Some(_) = p {
                        Term::Integer(-1 * isize::try_from(value).unwrap())
                    } else {
                        Term::Natural(value)
                    }
                }
            }
            // types
            rule primitive() -> Type = k:$("empty" / "unit" / "bool" / "nat" / "int") {
                match k {
                    "empty" => Type::Empty,
                    "unit" => Type::Unit,
                    "bool" => Type::Boolean,
                    "nat" => Type::Natural,
                    "int" => Type::Integer,
                    _ => Type::Empty // never happens
                }
            }
            // fixme: parenthesis necessary, left-recursion issue
            rule function() -> Type = "(" w()? f:kind() w()? "->" w()? t:kind() w()? ")" {
                Type::Function { from: Box::new(f), to: Box::new(t) }
            }
            // todo: records, etc
            rule kind() -> Type
             = k:(function() / primitive()) {
                k
            }
            // fixme: cannot say e:(expr()), left-recursion issue
            rule ann() -> Expression
            = e:(cond() / abs() / app() / cons() / var()) w()? ":" w() k:kind() {
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
            // todo: multiple parameters pls
            rule abs() -> Expression
            = "func" w() n:ident() w()? "(" p:ident() ")" w()? ":" w()? k:function() w() "=" w() "{" w() f:expr() w() "}" {
                Expression::Annotation {
                    expr: Box::new(Expression::Abstraction { param: p, func: Box::new(f) }),
                    kind: k
                }
            }
            // fixme: this requires, uh, refactoring the ast...
            rule app() -> Expression
            = f:ident() "(" a:expr() ")" {
                Expression::Application {
                    func: Box::new(Expression::Variable { id: f }),
                    arg: Box::new(a)
                }
            }
            rule cond() -> Expression
            = "if" w() c:expr() w() "=" w() "{" w() t:expr() w() "};" w() "else" w() "=" w() "{" w() e:expr() w() "}" {
                Expression::Conditional {
                    if_cond: Box::new(c),
                    if_then: Box::new(t),
                    if_else: Box::new(e)
                }
            }
            pub rule expr() -> Expression
            = e:(ann() / cond() / abs() / app() / cons() / var()) ";" {
                e
            }
            pub rule file() -> Vec<Expression>
            = expr() ++ "\n"
        }
    }
    return puck::file(input);
}
