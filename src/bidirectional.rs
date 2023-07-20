use crate::ast::*;

impl Context {
    /// Checking judgement: takes an expression and a type to check against and calls out to `infer` as needed.
    pub fn check(&self, expression: Expression, target: &Type) -> Result<()> {
        match expression {
            // fall through to inference mode
            Expression::Annotation { expr, kind } => {
                let result = self.infer(Expression::Annotation { expr, kind })?;
                return match self.subtype(&result, &target) {
                    true => Ok(()),
                    false => Err(format!("inferred type {result} does not match target {target}").into())
                }
            },
            // Bt-CheckInfer
            Expression::Constant { term } => match self.subtype(&term.convert()?, &target) {
                true => Ok(()),
                false => Err(format!("constant is of wrong type, expected {target}").into())
                // false => Ok(()) // all our constants are Empty for now
            },
            // Bt-CheckInfer
            Expression::Variable { id } => match self.get_term(&id) {
                Some(term) if self.subtype(&term.convert()?, &target) => Ok(()),
                Some(_) => Err(format!("variable {id} is of wrong type").into()),
                None => Err(format!("failed to find variable {id} in context").into())
            },
            // Bt-Abs
            Expression::Abstraction { param, func } => match target {
                Type::Function(from, to) => {
                    let mut context = self.clone();
                    context.insert_term(param, from.default()?);
                    return context.check(*func, &to);
                },
                _ => Err(format!("attempting to check an abstraction with a non-function type {target}").into())
            },
            // fall through to inference mode
            Expression::Application { func, arg } => {
                let result = &self.infer(Expression::Application { func, arg })?;
                return match self.subtype(result, target) {
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
            Expression::Variable { id } => match self.get_term(&id) {
                Some(term) => Context::new().infer(Expression::Constant { term: term.clone() }),
                None => Err(format!("failed to find variable in context {self:?}").into())
            },
            // Bt-App
            Expression::Application { func, arg } => match self.infer(*func)? {
                Type::Function(from, to) => self.check(*arg, &*from).map(|x| *to),
                _ => Err("application abstraction is not a function type".into())
            },
            // inference from an abstraction is always an error
            // we could try and infer the func without adding the parameter to scope:
            // but this is overwhelmingly likely to be an error, so just report it now.
            Expression::Abstraction { param, func } =>
                Err("attempting to infer from an abstraction".into()),
            // idk
            Expression::Conditional { if_cond, if_then, if_else } => {
                self.check(*if_cond, &Type::Boolean)?;
                let if_then = self.infer(*if_then)?;
                let if_else = self.infer(*if_else)?;
                if self.subtype(&if_then, &if_else) && self.subtype(&if_else, &if_then) {
                    Ok(if_then) // fixme: should be the join
                } else {
                    Err(format!("if clauses of different types: {if_then} and {if_else}").into())
                }
            }
        }
    }

    /// The subtyping relation between any two types.
    /// "is" is a subtype of "of", i.e. "is" can be safely used in any context "of" is expected.
    pub fn subtype(&self, is: &Type, of: &Type) -> bool {
        match (is, of) {
            (_, Type::Empty) => true,   // top type: every type is a subtype of the empty type (empty as in structurally empty)
            (Type::Error, _) => true,   // bottom type: no type is a subtype of the error type
            (Type::Natural, Type::Integer) => true, // obviously not, but let's pretend
            (Type::List(is), Type::Slice(of)) | (Type::Array(is, _), Type::Slice(of)) |
            (Type::List(is), Type::List(of)) |  (Type::Slice(is), Type::Slice(of)) => self.subtype(is, of),
            (Type::Array(is, is_size), Type::Array(of, of_size)) => self.subtype(is, of) && is_size == of_size,
            (Type::Tuple(is_data, is_fields), Type::Tuple(of_data, of_fields)) => {
                // length, order, and subtype
                if is_data.len() != of_data.len() || is_fields.len() != of_fields.len() {
                    return false;
                }
                for (is, of) in std::iter::zip(is_data, of_data) {
                    if !self.subtype(is, of) {
                        return false;
                    }
                }
                for (is, of) in std::iter::zip(is_fields, of_fields) {
                    if is != of {
                        return false;
                    }
                }
                true
            },
            (Type::Struct(is), Type::Struct(of)) => {
                // width, depth, and permutation
                for (key, of_value) in of {
                    match is.get(key) {
                        Some(is_value) => {
                            if !self.subtype(is_value, of_value) {
                                return false;
                            }
                        }
                        None => return false
                    }
                }
                true
            },
            (Type::Union(is), Type::Union(of)) => {
                // a union type is a subtype of another if the latter has *more* fields (opposite structs!)
                for data in of {
                    if !is.contains(data) {
                        return false;
                    }
                }
                true
            },
            (Type::Function(is_from, is_to), Type::Function(of_from, of_to)) => {
                self.subtype(of_from, is_from) && self.subtype(is_to, of_to)
            },
            (is, Type::Interface(signatures, associated)) => {
                if let Some(of) = associated && !self.subtype(is, of) {
                    return false;
                }
                for sig in signatures.clone() {
                    let signature = Signature {
                        name: sig.name,
                        from: sig.from.deselfify(is),
                        to: sig.to.deselfify(is)
                    };
                    if !(self.contains_sig(&signature)) { // we need context for interfaces...
                        return false;
                    }
                }
                true
            },
            (is, Type::Generic(Some(data))) => data.contains(is),
            (_, Type::Generic(None)) => true,
            (_, _) => is == of
        }
    }
}

impl Type {
    /// Replace explicit Oneself types with a replacement type. For interfaces.
    fn deselfify(self, replacement: &Type) -> Self {
        match self {
            Type::Oneself => replacement.clone(),
            Type::Empty | Type::Error | Type::Unit | Type::Boolean |
            Type::Natural | Type::Integer | Type::Float | Type::String => self,
            Type::List(data) => Type::List(Box::new(data.deselfify(replacement))),
            Type::Array(data, len) => Type::Array(Box::new(data.deselfify(replacement)), len),
            Type::Slice(data) => Type::Slice(Box::new(data.deselfify(replacement))),
            Type::Union(data) => Type::Union(
                data.iter().map(|x| x.clone().deselfify(replacement)).collect()),
            Type::Struct(data) => Type::Struct(
                data.iter().map(|(k, v)| (k.clone(), v.clone().deselfify(replacement))).collect()),
            Type::Tuple(data, idents) => Type::Tuple(
                data.iter().map(|x| x.clone().deselfify(replacement)).collect(), idents),
            Type::Function(from, to) => Type::Function(
                Box::new(from.deselfify(replacement)), Box::new(to.deselfify(replacement))),
            Type::Interface(signatures, associated) => Type::Interface(signatures,
                associated.map(|x| Box::new(x.deselfify(replacement)))),
            Type::Generic(Some(data)) => Type::Generic(
                Some(data.iter().map(|x| x.clone().deselfify(replacement)).collect())),
            Type::Generic(None) => Type::Generic(None),
        }
    }
}
