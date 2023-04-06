// Simple bidirectional type checking

#![allow(unused_variables)]

use crate::ast::*;

pub fn check(context: Context, expression: Expression, target: Type) -> bool {
    match expression {
        Expression::Annotation { expr, kind } => kind == target,
        Expression::Constant { term } => term.kind == target,
        Expression::Variable { id } => {
            match context.get(&id) {
                Some(term) => term.kind == target,
                None => false,
            }
        },
        Expression::Abstraction { param, func } => todo!(),
        Expression::Application { func, arg } => todo!(),
        Expression::Conditional { if_cond, if_then, if_else } => todo!(),
    }
}

// empty's gonna cause problems
pub fn infer(context: Context, expression: Expression) -> Option<Type> {
    match expression {
        Expression::Annotation { expr, kind } =>  Some(kind),
        Expression::Constant { term } => Some(term.kind),
        Expression::Variable { id } => {
            match context.get(&id) {
                Some(term) => Some(term.kind),
                None => None
            }
        },
        Expression::Abstraction { param, func } => return None,
        Expression::Application { func, arg } => return None,
        Expression::Conditional { if_cond, if_then, if_else } => {
            if infer(context.clone(), *if_cond) == Some(Type::Boolean) {
                let kind = infer(context.clone(), *if_then);
                if infer(context, *if_else) == kind {
                    return kind;
                }
            }
            return None;
        },
    }
}

/// Evaluates an expression given a context (of variables) to a term.
/// Panics on non-evaluatable code.
pub fn execute(context: Context, expression: Expression) -> Result<Term, &'static str> {
    match expression {
        Expression::Annotation { expr, .. } => execute(context, *expr),
        Expression::Constant { term } => Ok(term),
        Expression::Variable { id } => {
            match context.get(&id) {
                Some(term) => Ok(*term),
                None => Err("no such variable in context")
            }
        },
        Expression::Abstraction { .. } => Err("attempting to execute an abstraction"),
        Expression::Application { func, arg } => {
            match *func {
                Expression::Abstraction { param, func } => {
                    let result = execute(context.clone(), *arg);
                    match result {
                        Ok(value) => {
                            let mut context = context;
                            context.insert(param, value);
                            return execute(context, *func);
                        },
                        Err(e) => Err(e)
                    }
                },
                _ => Err("attempting to execute an application to nothing")
            }
        },
        Expression::Conditional { if_cond, if_then, if_else } => {
            match execute(context.clone(), *if_cond) {
                Ok(Term { val: 1, .. }) => execute(context, *if_then),
                Ok(Term { val: 0, .. }) => execute(context, *if_else),
                _ => Err("invalid type for a conditional")
            }
        },
    }
}
