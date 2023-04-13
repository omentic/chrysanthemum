// Simple bidirectional type checking

use crate::ast::*;

/// Checking judgement: takes an expression and a type to check against and calls out to `infer` as needed.
pub fn check(context: &Context, expression: Expression, target: &Type) -> Result<(), String> {
    match expression {
        // fall through to inference mode
        Expression::Annotation { expr, kind } => {
            let result = infer(context, Expression::Annotation { expr, kind })?;
            return match subtype(&result, &target) {
                true => Ok(()),
                false => Err(format!("inferred type {result} does not match target {target}"))
            }
        },
        // Bt-CheckInfer
        Expression::Constant { term } => match subtype(&convert(&term)?, &target) {
            true => Ok(()),
            false => Err(format!("constant is of wrong type, expected {target}"))
            // false => Ok(()) // all our constants are Empty for now
        },
        // Bt-CheckInfer
        Expression::Variable { id } => match context.get(&id) {
            Some(term) if subtype(&convert(term)?, &target) => Ok(()),
            Some(_) => Err(format!("variable {id} is of wrong type")),
            None => Err(format!("failed to find variable {id} in context"))
        },
        // Bt-Abs
        Expression::Abstraction { param, func } => match target {
            Type::Function { from, to } => {
                let mut context = context.clone();
                context.insert(param, default(from)?);
                return check(&context, *func, &to);
            },
            _ => Err(format!("attempting to check an abstraction with a non-function type {target}"))
        },
        // fall through to inference mode
        Expression::Application { func, arg } => {
            let result = &infer(context, Expression::Application { func, arg })?;
            return match subtype(&result, &target) {
                true => Ok(()),
                false => Err(format!("inferred type {result} does not match {target}"))
            }
        },
        // T-If
        Expression::Conditional { if_cond, if_then, if_else } => {
            check(context, *if_cond, &Type::Boolean)?;
            check(context, *if_then, &target)?;
            check(context, *if_else, &target)?;
            return Ok(());
        }
    }
}

/// Inference judgement: takes an expression and attempts to infer the associated type.
pub fn infer(context: &Context, expression: Expression) -> Result<Type, String> {
    match expression {
        // Bt-Ann
        Expression::Annotation { expr, kind } => check(context, *expr, &kind).map(|x| kind),
        // Bt-True / Bt-False / etc
        Expression::Constant { term } => convert(&term),
        // Bt-Var
        Expression::Variable { id } => match context.get(&id) {
            Some(term) => infer(&Context::new(), Expression::Constant { term: term.clone() }),
            None => Err(format!("failed to find variable in context {context:?}"))
        },
        // Bt-App
        Expression::Application { func, arg } => match infer(context, *func)? {
            Type::Function { from, to } => check(context, *arg, &*from).map(|x| *to),
            _ => Err(format!("application abstraction is not a function type"))
        },
        // inference from an abstraction is always an error
        // we could try and infer the func without adding the parameter to scope:
        // but this is overwhelmingly likely to be an error, so just report it now.
        Expression::Abstraction { param, func } =>
            Err(format!("attempting to infer from an abstraction")),
        // idk
        Expression::Conditional { if_cond, if_then, if_else } => {
            check(context, *if_cond, &Type::Boolean)?;
            let if_then = infer(context, *if_then)?;
            let if_else = infer(context, *if_else)?;
            if subtype(&if_then, &if_else) && subtype(&if_else, &if_then) {
                Ok(if_then) // fixme: should be the join
            } else {
                Err(format!("if clauses of different types: {if_then} and {if_else}"))
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
        (Type::Enum(is_variants), Type::Enum(of_variants)) => false, // fixme
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

