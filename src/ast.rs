// Bidirectional type checking, subtyping, and typeclasses

use core::fmt;
use std::collections::HashMap;

pub type Identifier = String;
pub type Context = HashMap<Identifier, Term>;

// note: built-in functions do NOT go here!
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Annotation{expr: Box<Expression>, kind: Type},
    Constant{term: Term},
    Variable{id: Identifier},
    // note: we keep parameters as an Identifier because we annotate the WHOLE Abstraction
    Abstraction{param: Identifier, func: Box<Expression>},
    Application{func: Box<Expression>, arg: Box<Expression>},
    Conditional{if_cond: Box<Expression>, if_then: Box<Expression>, if_else: Box<Expression>}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Empty,
    Error,
    Unit,
    Boolean,
    Natural,
    Integer,
    Float,
    String,
    Enum(Vec<Type>),
    Record(HashMap<Identifier, Type>),
    Function{from: Box<Type>, to: Box<Type>},
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Unit(),
    Boolean(bool),
    Natural(usize),
    Integer(isize),
    Float(f32),
    String{len: usize, cap: usize, data: Vec<usize>},
    Enum{val: usize, data: Vec<Type>}, // is this right?
    Record(HashMap<Identifier, Term>), // is this right?
    Function(Box<Expression>) // this should allow us to bind functions
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Annotation { expr, kind } => write!(f, "({}: {})", expr, kind),
            Expression::Constant { term } => write!(f, "'{:?}", term),
            Expression::Variable { id } => write!(f, "{}", id),
            Expression::Abstraction { param, func } => write!(f, "(λ{}.{})", param, func),
            Expression::Application { func, arg } => write!(f, "({} {})", func, arg),
            Expression::Conditional { if_cond, if_then, if_else } => write!(f, "(if {} then {} else {})", if_cond, if_then, if_else),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Empty => write!(f, "⊤"),
            Type::Error => write!(f, "⊥"),
            Type::Unit => write!(f, "unit"),
            Type::Boolean => write!(f, "bool"),
            Type::Natural => write!(f, "nat"),
            Type::Integer => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "str"),
            Type::Enum(data) => write!(f, "({:?})", data),
            Type::Record(data) => write!(f, "{{{:?}}}", data),
            Type::Function { from, to } => write!(f, "{}->{}", from, to),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Unit() => write!(f, "∅"),
            Term::Boolean(term) => write!(f, "{}", term),
            Term::Natural(term) => write!(f, "{}", term),
            Term::Integer(term) => write!(f, "{}", term),
            Term::Float(term) => write!(f, "{}", term),
            Term::String { len, cap, data } => write!(f, "\"{:?}\"", data),
            Term::Enum { val, data } => write!(f, "{:?}", data.get(*val)),
            Term::Record(term) => write!(f, "{:?}", term),
            Term::Function(expr) => write!(f, "{}", *expr),
        }
    }
}

// hatehatehate that you can't implement a trait for foreign types
// impl<T> fmt::Display for Vec<T> where T: fmt::Display {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for (i, val) in self.enumerate() {
//             if i == 0 {
//                 write!(f, "{}", val);
//             } else {
//                 write!(f, ",{}", val);
//             }
//         }
//         return Ok(());
//     }
// }

// impl<T, U> fmt::Display for HashMap<T, U> where T: fmt::Display, U: fmt::Display {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for (i, (key, val)) in self.enumerate() {
//             if i == 0 {
//                 write!(f, "{}={}", key, val);
//             } else {
//                 write!(f, ",{}={}", key, val);
//             }
//         }
//         return Ok(());
//     }
// }
