#![allow(non_snake_case, non_upper_case_globals)]

use crate::ast::*;

// intentionally small: i want to run into errors
/// assumption: the count is instantiated to zero
pub fn unique_ident(count: &mut u8) -> String {
    *count += 1;
    if *count == 0 {
        panic!("we've overflowed!")
    } else {
        format!("{:X}", count)
    }
}

pub fn Ann(expr: Expression, kind: Type) -> Expression {
    Expression::Annotation { expr: Box::new(expr), kind }
}

pub fn Const(term: Term) -> Expression {
    Expression::Constant { term }
}

pub fn Var(id: &str) -> Expression {
    Expression::Variable { id: String::from(id) }
}

pub fn Abs(param: &str, func: Expression) -> Expression {
    Expression::Abstraction {
        param: String::from(param),
        func: Box::new(func)
    }
}

pub fn App(func: Expression, arg: Expression) -> Expression {
    Expression::Application {
        func: Box::new(func),
        arg: Box::new(arg)
    }
}

pub fn Cond(if_cond: Expression, if_then: Expression, if_else: Expression) -> Expression {
    Expression::Conditional {
        if_cond: Box::new(if_cond),
        if_then: Box::new(if_then),
        if_else: Box::new(if_else)
    }
}

pub fn Func(from: Type, to: Type) -> Type {
    Type::Function(Box::new(from), Box::new(to))
}

pub const Empty: Type = Type::Empty;
pub const Error: Type = Type::Empty;
pub const Unit: Type = Type::Unit;
pub const Bool: Type = Type::Boolean;
pub const Nat: Type = Type::Natural;
pub const Int: Type = Type::Integer;

pub fn Float(term: f32) -> Term {
    Term::Float(term)
}

pub fn Str(data: &str) -> Term {
    Term::String(data.into())
}

pub fn Union(data: Term) -> Term {
    Term::Union(Box::new(data))
}
