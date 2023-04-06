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
pub fn execute(context: Context, expression: Expression) -> Term {
    match expression {
        Expression::Annotation { expr, .. } => return execute(context, *expr),
        Expression::Constant { term } => return term,
        Expression::Variable { id } => return context[&id],
        Expression::Abstraction { .. } => panic!("attempting to execute an abstraction"),
        Expression::Application { func, arg } => {
            match *func {
                Expression::Abstraction { param, func } => {
                    let mut context = context;
                    context.insert(param, execute(context.clone(), *arg));
                    return execute(context, *func);
                },
                _ => panic!("attempting to execute an application to nothing")
            }
        },
        Expression::Conditional { if_cond, if_then, if_else } => {
            match execute(context.clone(), *if_cond).val {
                1 => execute(context, *if_then),
                0 => execute(context, *if_else),
                _ => panic!("invalid type for a conditional")
            }
        },
    }
}
