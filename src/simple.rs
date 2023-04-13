use crate::ast::*;

/// Evaluates an expression given a context (of variables) to a term, or fails.
pub fn execute(context: &Context, expression: Expression) -> Result<Term, String> {
    match expression {
        Expression::Annotation { expr, .. } => execute(context, *expr),
        Expression::Constant { term } => Ok(term),
        Expression::Variable { id } => match context.get(&id) {
            Some(term) => Ok(term.clone()),
            None => Err(format!("no such variable in context {context:?}"))
        },
        Expression::Abstraction { param, func } =>
            Err(format!("attempting to execute an abstraction ({}){}", param, func)),
        Expression::Application { func, arg } => match *func {
            Expression::Abstraction { param, func } => {
                let value = execute(context, *arg)?;
                let mut context = context.clone();
                context.insert(param, value);
                return execute(&context, *func);
            }
            _ => Err(format!("attempting to execute an application to non-abstraction {}", *func))
        },
        Expression::Conditional { if_cond, if_then, if_else } => {
            match execute(context, *if_cond)? {
                Term::Boolean(true) => execute(context, *if_then),
                Term::Boolean(false) => execute(context, *if_else),
                term => Err(format!("invalid type {} for a conditional", convert(&term)?))
            }
        }
    }
}

