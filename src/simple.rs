// Simple bidirectional type checking

#![allow(unused_variables)]

use crate::ast::*;

pub fn check(context: Context, expression: Expression, target: Type) -> bool {
    match expression {
        // should never happen
        Expression::Annotation { expr, kind } => (kind == target) && check(context, *expr, kind),
        Expression::Constant { term } => term.kind == target,
        Expression::Variable { id } => {
            // in the future: extend to closures? nah probably not
            match context.get(&id) {
                Some(term) if term.kind == target => true,
                _ => false
            }
        },
        Expression::Abstraction { param, func } => {
            match target {
                Type::Function { from, to } => {
                    let mut context = context;
                    context.insert(param, Term { val: 0, kind: *from }); // hack
                    return check(context, *func, *to);
                },
                _ => false
            }
        },
        // should never happen
        Expression::Application { func, arg } => check(context, *func, target),
        Expression::Conditional { if_cond, if_then, if_else } => {
            check(context.clone(), *if_cond, Type::Boolean) &&
            check(context.clone(), *if_then, target.clone()) &&
            check(context.clone(), *if_else, target.clone())
        }
    }
}

// empty's gonna cause problems
pub fn infer(context: Context, expression: Expression) -> Option<Type> {
    match expression {
        Expression::Annotation { expr, kind } => {
            match check(context, *expr, kind.clone()) {
                true => Some(kind),
                false => None
            }
        },
        Expression::Constant { term } => Some(term.kind),
        Expression::Variable { id } => {
            match context.get(&id) {
                Some(term) => Some(term.clone().kind),
                None => None
            }
        },
        Expression::Application { func, arg } => {
            // Bt-App:
            // \Gamma \derives t_1 \infer \tau_1 \to \tau_2, \Gamma \derives t_2 \check \tau_1
            // -------------------------------------------
            // \Gamma \derives t_1 t_2 \infer \tau_2
            match infer(context.clone(), *func) {
                Some(Type::Function { from, to }) => {
                    match check(context, *arg, *from) {
                        true => Some(*to),
                        false => None
                    }
                },
                _ => None
            }
        },
        // should not happen
        Expression::Abstraction { param, func } => infer(context, *func),
        // idk
        Expression::Conditional { if_cond, if_then, if_else } => {
            if check(context.clone(), *if_cond, Type::Boolean) {
                let kind = infer(context.clone(), *if_then);
                if kind == infer(context, *if_else) {
                    return kind;
                }
            }
            return None;
        }
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
                Some(term) => Ok(term.clone()),
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
        }
    }
}
