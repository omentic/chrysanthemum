// Simple bidirectional type checking

use crate::ast::*;

impl Context {
    /// Checking judgement: takes an expression and a type to check against and calls out to `infer` as needed.
    pub fn check(&self, expression: Expression, target: &Type) -> Result<()> {
        match expression {
            // fall through to inference mode
            Expression::Annotation { expr, kind } => {
                let result = self.infer(Expression::Annotation { expr, kind })?;
                return match result.subtype(&target) {
                    true => Ok(()),
                    false => Err(format!("inferred type {result} does not match target {target}").into())
                }
            },
            // Bt-CheckInfer
            Expression::Constant { term } => match &term.convert()?.subtype(&target) {
                true => Ok(()),
                false => Err(format!("constant is of wrong type, expected {target}").into())
                // false => Ok(()) // all our constants are Empty for now
            },
            // Bt-CheckInfer
            Expression::Variable { id } => match self.get(&id) {
                Some(term) if term.convert()?.subtype(&target) => Ok(()),
                Some(_) => Err(format!("variable {id} is of wrong type").into()),
                None => Err(format!("failed to find variable {id} in context").into())
            },
            // Bt-Abs
            Expression::Abstraction { param, func } => match target {
                Type::Function { from, to } => {
                    let mut context = self.clone();
                    context.insert(param, from.default()?);
                    return context.check(*func, &to);
                },
                _ => Err(format!("attempting to check an abstraction with a non-function type {target}").into())
            },
            // fall through to inference mode
            Expression::Application { func, arg } => {
                let result = &self.infer(Expression::Application { func, arg })?;
                return match result.subtype(&target) {
                    true => Ok(()),
                    false => Err(format!("inferred type {result} does not match {target}").into())
                }
            },
            // T-If
            Expression::Conditional { if_cond, if_then, if_else } => {
                self.check(*if_cond, &Type::Boolean)?;
                self.check(*if_then, &target)?;
                self.check(*if_else, &target)?;
                return Ok(());
            }
        }
    }

    /// Inference judgement: takes an expression and attempts to infer the associated type.
    pub fn infer(&self, expression: Expression) -> Result<Type> {
        match expression {
            // Bt-Ann
            Expression::Annotation { expr, kind } => self.check(*expr, &kind).map(|x| kind),
            // Bt-True / Bt-False / etc
            Expression::Constant { term } => term.convert(),
            // Bt-Var
            Expression::Variable { id } => match self.get(&id) {
                Some(term) => Context::new().infer(Expression::Constant { term: term.clone() }),
                None => Err(format!("failed to find variable in context {self:?}").into())
            },
            // Bt-App
            Expression::Application { func, arg } => match self.infer(*func)? {
                Type::Function { from, to } => self.check(*arg, &*from).map(|x| *to),
                _ => Err(format!("application abstraction is not a function type").into())
            },
            // inference from an abstraction is always an error
            // we could try and infer the func without adding the parameter to scope:
            // but this is overwhelmingly likely to be an error, so just report it now.
            Expression::Abstraction { param, func } =>
                Err(format!("attempting to infer from an abstraction").into()),
            // idk
            Expression::Conditional { if_cond, if_then, if_else } => {
                self.check(*if_cond, &Type::Boolean)?;
                let if_then = self.infer(*if_then)?;
                let if_else = self.infer(*if_else)?;
                if if_then.subtype(&if_else) && if_else.subtype(&if_then) {
                    Ok(if_then) // fixme: should be the join
                } else {
                    Err(format!("if clauses of different types: {if_then} and {if_else}").into())
                }
            }
        }
    }
}

impl Type {
    /// The subtyping relation between any two types.
    pub fn subtype(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Struct(is_fields), Type::Struct(of_fields)) => {
                // width, depth, and permutation
                for (key, of_value) in of_fields {
                    match is_fields.get(key) {
                        Some(is_value) => {
                            if !is_value.subtype(of_value) {
                                return false;
                            }
                        }
                        None => return false
                    }
                }
                return true;
            },
            (Type::Union(is_variants), Type::Union(of_variants)) => false, // fixme
            (Type::Function { from: is_from, to: is_to },
             Type::Function { from: of_from, to: of_to }) => {
                of_from.subtype(is_from) && is_to.subtype(of_to)
            },
            (Type::Natural, Type::Integer) => true, // obviously not, but let's pretend
            (_, Type::Empty) => true,
            (Type::Error, _) => true,
            (_, _) if self == other => true,
            (_, _) => false
        }
    }
}
