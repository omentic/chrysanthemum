// Bidirectional type checking, simple types for effects (or perhaps subtyping?) and typeclasses

use core::fmt;
use std::collections::HashMap;

pub type Identifier = String;
pub type Context = HashMap<Identifier, Term>;

// note: when comes the time, we'll put effects in here (i think)
#[derive(Clone, PartialEq, Eq)]
pub enum Expression {
    Annotation{expr: Box<Expression>, kind: Type},
    Constant{term: Term},
    Variable{id: Identifier},
    Abstraction{param: Identifier, func: Box<Expression>},
    Application{func: Box<Expression>, arg: Box<Expression>},
    Conditional{if_cond: Box<Expression>, if_then: Box<Expression>, if_else: Box<Expression>}
}

// _every_ type in our language is represented as this and interpreted as a type.
// how to store more data than fits... hmm
pub type Value = u64;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Type {
    Empty,
    Unit,
    Bool,
    Natural,
    Integer,
    // Float,
    // String,
    // Enum(Vec<Type>),
    // Record(Vec<Type>),
    // Function{from: Box<Type>, to: Box<Type>},
}

// this means that functions cannot have types? unless we put them as empty values ig
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Term {
    pub val: Value,
    pub kind: Type, // currently useless / redundant: will be useful for casting
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Annotation { expr, kind } => write!(f, "({:?}: {:?})", expr, kind),
            Expression::Constant { term } => write!(f, "'{}", term.val),
            Expression::Variable { id } => write!(f, "{}", id),
            Expression::Abstraction { param, func } => write!(f, "(λ{}.{:?})", param, func),
            Expression::Application { func, arg } => write!(f, "({:?} {:?})", func, arg),
            Expression::Conditional { if_cond, if_then, if_else } => write!(f, "(if {:?} then {:?} else {:?})", if_cond, if_then, if_else),
        }
    }
}
