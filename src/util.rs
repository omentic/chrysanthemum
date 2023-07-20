#![allow(non_snake_case, non_upper_case_globals)]

use crate::ast::*;

// intentionally small: i want to run into errors
/// assumption: the count is instantiated to zero
pub fn unique_ident(count: &mut u8) -> String {
    *count += 1;
    if *count == 0 {
        panic!("we've overflowed!");
    } else {
        return String::from(format!("{:X}", count));
    }
}

pub fn Ann(expr: Expression, kind: Type) -> Expression {
    return Expression::Annotation {
        expr: Box::new(expr),
        kind: kind
    };
}

pub fn Const(term: Term) -> Expression {
    return Expression::Constant { term };
}

pub fn Var(id: &str) -> Expression {
    return Expression::Variable {
        id: String::from(id)
    };
}

pub fn Abs(param: &str, func: Expression) -> Expression {
    return Expression::Abstraction {
        param: String::from(param),
        func: Box::new(func),
    };
}

pub fn App(func: Expression, arg: Expression) -> Expression {
    return Expression::Application {
        func: Box::new(func),
        arg: Box::new(arg)
    };
}

pub fn Cond(if_cond: Expression, if_then: Expression, if_else: Expression) -> Expression {
    return Expression::Conditional {
        if_cond: Box::new(if_cond),
        if_then: Box::new(if_then),
        if_else: Box::new(if_else)
    };
}

pub fn Func(from: Type, to: Type) -> Type {
    return Type::Function(Box::new(from), Box::new(to))
}

pub const Empty: Type = Type::Empty;
pub const Error: Type = Type::Empty;
pub const Unit: Type = Type::Unit;
pub const Bool: Type = Type::Boolean;
pub const Nat: Type = Type::Natural;
pub const Int: Type = Type::Integer;

pub fn Float(term: f32) -> Term {
    return Term::Float(term)
}

pub fn Str(data: &str) -> Term {
    return Term::String(data.into())
}

pub fn Union(data: Term) -> Term {
    return Term::Union(Box::new(data))
}
