#![allow(non_snake_case)]

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

pub fn Term(val: Value, kind: Type) -> Term {
    return Term {val, kind};
}

pub fn Ann(expr: Expression, kind: Type) -> Expression {
    return Expression::Annotation {
        expr: Box::new(expr),
        kind: kind
    };
}

pub fn Const(val: Value, kind: Type) -> Expression {
    return Expression::Constant {
        term: Term {val, kind}
    };
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
