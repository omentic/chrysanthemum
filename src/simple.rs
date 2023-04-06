// Simple bidirectional type checking

use crate::ast::*;

pub fn infer(context: Context, expression: Expression) {
    todo!();
}

pub fn check(context: Context, expression: Expression) {
    todo!();
}

/// Evaluates an expression given a context (of variables) to a term.
/// Panics on non-evaluatable code.
pub fn execute(context: Context, expression: Expression) -> Result<Term, &'static str> {
    match expression {
        Expression::Annotation { expr, .. } => execute(context, *expr),
        Expression::Constant { term } => Ok(term),
        Expression::Variable { id } => context.get(&id).ok_or("no such variable in context").map(|x| *x),
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
