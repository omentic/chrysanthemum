// Simple bidirectional type checking

#![allow(unused_variables)]

use crate::ast::*;

pub fn check(context: Context, expression: Expression, target: Type) -> Result<(), (&'static str, Context, Type)> {
    match expression {
        // fall through to inference mode
        Expression::Annotation { expr, kind } => {
            let result = infer(context.clone(), Expression::Annotation { expr, kind })?;
            return match subtype(&result, &target) {
                true => Ok(()),
                false => Err(("inferred type does not match target", context, target))
            }
        },
        // Bt-CheckInfer
        Expression::Constant { term } => match subtype(&term.kind, &target) {
            true => Ok(()),
            false => Ok(()) // all our constants are Empty for now
            // false => Err(("constant is of wrong type", context, target))
        },
        // Bt-CheckInfer
        Expression::Variable { id } => match context.get(&id) {
            Some(term) if subtype(&term.kind, &target) => Ok(()),
            Some(_) => Err(("variable is of wrong type", context, target)),
            None => Err(("failed to find variable in context", context, target))
        },
        // Bt-Abs
        Expression::Abstraction { param, func } => match target {
            Type::Function { from, to } => {
                let mut context = context;
                context.insert(param, Term { val: 0, kind: *from }); // hack
                return check(context, *func, *to);
            },
            _ => Err(("attempting to check an abstraction with a non-function type", context, target))
        },
        // fall through to inference mode
        Expression::Application { func, arg } => {
            let result = &infer(context.clone(), Expression::Application { func, arg })?;
            return match subtype(&result, &target) {
                true => Ok(()),
                false => Err(("inferred type does not match target", context, target))
            }
        },
        // T-If
        Expression::Conditional { if_cond, if_then, if_else } => {
            check(context.clone(), *if_cond, Type::Boolean)?;
            check(context.clone(), *if_then, target.clone())?;
            check(context.clone(), *if_else, target.clone())?;
            return Ok(());
        }
    }
}

pub fn infer(context: Context, expression: Expression) -> Result<Type, (&'static str, Context, Type)> {
    match expression {
        // Bt-Ann
        Expression::Annotation { expr, kind } => check(context, *expr, kind.clone()).map(|x| kind),
        // Bt-True / Bt-False / etc
        Expression::Constant { term } => Ok(term.kind),
        // Bt-Var
        Expression::Variable { id } => match context.get(&id) {
            Some(term) => Ok(term.clone().kind),
            None => Err(("failed to find variable in context", context, Type::Empty))
        },
        // Bt-App
        Expression::Application { func, arg } => match infer(context.clone(), *func)? {
            Type::Function { from, to } => check(context, *arg, *from).map(|x| *to),
            _ => Err(("application abstraction is not a function type", context, Type::Empty))
        },
        // inference from an abstraction is always an error
        // we could try and infer the func without adding the parameter to scope:
        // but this is overwhelmingly likely to be an error, so just report it now.
        Expression::Abstraction { param, func } =>
            Err(("attempting to infer from an abstraction", context, Type::Empty)),
        // idk
        Expression::Conditional { if_cond, if_then, if_else } => {
            check(context.clone(), *if_cond, Type::Boolean)?;
            let if_then = infer(context.clone(), *if_then)?;
            let if_else = infer(context.clone(), *if_else)?;
            if subtype(&if_then, &if_else) && subtype(&if_else, &if_then) {
                Ok(if_then) // fixme: should be the join
            } else {
                Err(("if clauses of different types", context, Type::Empty))
            }
        }
    }
}

/// Evaluates an expression given a context (of variables) to a term, or fails.
pub fn execute(context: Context, expression: Expression) -> Result<Term, (&'static str, Context)> {
    match expression {
        Expression::Annotation { expr, .. } => execute(context, *expr),
        Expression::Constant { term } => Ok(term),
        Expression::Variable { id } => match context.get(&id) {
            Some(term) => Ok(term.clone()),
            None => Err(("no such variable in context", context))
        },
        Expression::Abstraction { .. } => Err(("attempting to execute an abstraction", context)),
        Expression::Application { func, arg } => match *func {
            Expression::Abstraction { param, func } => {
                let value = execute(context.clone(), *arg)?;
                let mut context = context;
                context.insert(param, value);
                return execute(context, *func);
            }
            _ => Err(("attempting to execute an application to nothing", context))
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

/// The subtyping relation between any two types.
pub fn subtype(is: &Type, of: &Type) -> bool {
    match (is, of) {
        (Type::Record(is_fields), Type::Record(of_fields)) => {
            // width, depth, and permutation
            for (key, of_value) in of_fields {
                match is_fields.get(key) {
                    Some(is_value) => {
                        if !subtype(is_value, of_value) {
                            return false;
                        }
                    }
                    None => return false
                }
            }
            return true;
        },
        (Type::Function { from: is_from, to: is_to },
         Type::Function { from: of_from, to: of_to }) => {
            subtype(of_from, is_from) && subtype(is_to, of_to)
        },
        (Type::Natural, Type::Integer) => true, // obviously not, but let's pretend
        (_, Type::Empty) => true,
        (Type::Error, _) => true,
        (_, _) if is == of => true,
        (_, _) => false
    }
}
