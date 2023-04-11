// Simple bidirectional type checking

#![allow(unused_variables)]

use crate::ast::*;

pub fn check(context: Context, expression: Expression, target: Type) -> Result<(), (&'static str, Context, Type)> {
    match expression {
        Expression::Annotation { expr, kind } => Err(("attempting to typecheck an annotation", context, target)),
        // Bt-CheckInfer
        Expression::Constant { term } => {
            if term.kind == target {
                Ok(())
            } else {
                Err(("constant is of wrong type", context, target))
            }
        },
        // Bt-CheckInfer
        Expression::Variable { id } => {
            // in the future: extend to closures? nah probably not
            match context.get(&id) {
                Some(term) if term.kind == target => Ok(()),
                Some(_) => Err(("variable is of wrong type", context, target)),
                None => Err(("failed to find variable in context", context, target))
            }
        },
        // Bt-Abs
        Expression::Abstraction { param, func } => {
            match target {
                Type::Function { from, to } => {
                    let mut context = context;
                    context.insert(param, Term { val: 0, kind: *from }); // hack
                    return check(context, *func, *to);
                },
                _ => Err(("attempting to check an abstraction with a non-function type", context, target))
            }
        },
        Expression::Application { func, arg } => Err(("attempting to check an application", context, target)),
        // T-If
        Expression::Conditional { if_cond, if_then, if_else } => {
            check(context.clone(), *if_cond, Type::Boolean)?;
            check(context.clone(), *if_then, target.clone())?;
            check(context.clone(), *if_else, target.clone())?;
            return Ok(());
        }
    }
}

// empty's gonna cause problems
pub fn infer(context: Context, expression: Expression) -> Result<Type, (&'static str, Context, Type)> {
    match expression {
        // Bt-Ann
        Expression::Annotation { expr, kind } => check(context, *expr, kind.clone()).map(|x| kind),
        // Bt-True / Bt-False / etc
        Expression::Constant { term } => Ok(term.kind),
        // Bt-Var
        Expression::Variable { id } => {
            match context.get(&id) {
                Some(term) => Ok(term.clone().kind),
                None => Err(("failed to find variable in context", context, Type::Empty))
            }
        },
        // Bt-App
        Expression::Application { func, arg } => {
            let expr = infer(context.clone(), *func)?;
            match expr {
                Type::Function { from, to } => {
                    check(context, *arg, *from).map(|x| *to)
                },
                _ => Err(("application abstraction is not a function type", context, Type::Empty))
            }
        },
        Expression::Abstraction { param, func } => Err(("attempting to infer from an abstraction", context, Type::Empty)),
        // idk
        Expression::Conditional { if_cond, if_then, if_else } => {
            check(context.clone(), *if_cond, Type::Boolean)?;
            let if_then = infer(context.clone(), *if_then)?;
            let if_else = infer(context.clone(), *if_else)?;
            if if_then == if_else {
                Ok(if_then)
            } else {
                Err(("if clauses of different types", context, Type::Empty))
            }
        }
    }
}

/// Evaluates an expression given a context (of variables) to a term.
/// Panics on non-evaluatable code.
pub fn execute(context: Context, expression: Expression) -> Result<Term, (&'static str, Context)> {
    match expression {
        Expression::Annotation { expr, .. } => execute(context, *expr),
        Expression::Constant { term } => Ok(term),
        Expression::Variable { id } => {
            match context.get(&id) {
                Some(term) => Ok(term.clone()),
                None => Err(("no such variable in context", context))
            }
        },
        Expression::Abstraction { .. } => Err(("attempting to execute an abstraction", context)),
        Expression::Application { func, arg } => {
            match *func {
                Expression::Abstraction { param, func } => {
                    let value = execute(context.clone(), *arg)?;
                    let mut context = context;
                    context.insert(param, value);
                    return execute(context, *func);
                },
                _ => Err(("attempting to execute an application to nothing", context))
            }
        },
        Expression::Conditional { if_cond, if_then, if_else } => {
            match execute(context.clone(), *if_cond) {
                Ok(Term { val: 1, .. }) => execute(context, *if_then),
                Ok(Term { val: 0, .. }) => execute(context, *if_else),
                _ => Err(("invalid type for a conditional", context))
            }
        }
    }
}
