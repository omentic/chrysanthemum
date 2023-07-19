use crate::ast::*;

impl Context {
    /// Evaluates an expression given a context (of variables) to a term, or fails.
    pub fn execute(&self, expression: Expression) -> Result<Term> {
        match expression {
            Expression::Annotation { expr, .. } => self.execute(*expr),
            Expression::Constant { term } => Ok(term),
            Expression::Variable { id } => match self.get(&id) {
                Some(term) => Ok(term.clone()),
                None => Err(format!("no such variable in context {self:?}").into())
            },
            Expression::Abstraction { param, func } =>
                Err(format!("attempting to execute an abstraction ({}){}", param, func).into()),
            Expression::Application { func, arg } => match *func {
                Expression::Abstraction { param, func } => {
                    let value = self.execute(*arg)?;
                    let mut context = self.clone();
                    context.insert(param, value);
                    return context.execute(*func);
                }
                _ => Err(format!("attempting to execute an application to non-abstraction {}", *func).into())
            },
            Expression::Conditional { if_cond, if_then, if_else } => {
                match self.execute(*if_cond)? {
                    Term::Boolean(true) => self.execute(*if_then),
                    Term::Boolean(false) => self.execute(*if_else),
                    term => Err(format!("invalid type {} for a conditional", &term.convert()?).into())
                }
            }
        }
    }
}
