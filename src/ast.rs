use std::collections::HashMap;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;
pub type Identifier = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Context(HashMap<Identifier, Term>);

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

/// All supported types.
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
    List(Box<Type>),
    Array(Box<Type>, usize), // todo: replace with dependent types
    Slice(Box<Type>), // potentially fucky lifetime stuff too
    Union(Vec<Type>), // unordered
    Struct(HashMap<Identifier, Type>), // unordered
    Tuple(Vec<Type>, Vec<Option<Identifier>>), // ordered with labels
    Function{from: Box<Type>, to: Box<Type>},
}

/// Data associated with a type.
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Unit(),
    Boolean(bool),
    Natural(usize),
    Integer(isize),
    Float(f32),
    String(String),
    List(Vec<Term>),
    Array(Vec<Term>),
    Union(Box<Term>),
    Struct(HashMap<Identifier, Term>),
    Tuple(Vec<Term>, Vec<Option<Identifier>>),
    Function(Box<Expression>) // this should allow us to bind functions
}

impl Term {
    /// Convert a term into its corresponding type.
    pub fn convert(&self) -> Result<Type> {
        match self {
            Term::Unit() => Ok(Type::Unit),
            Term::Boolean(_) => Ok(Type::Boolean),
            Term::Natural(_) => Ok(Type::Natural),
            Term::Integer(_) => Ok(Type::Integer),
            Term::Float(_) => Ok(Type::Float),
            Term::String(_) => Ok(Type::String),
            Term::List(data) => match data.len() {
                0 => Err("attempting to get the type of an empty list!".into()),
                _ => Ok(Type::List(Box::new(data.get(0).unwrap().convert()?))),
            },
            Term::Array(data) => match data.len() {
                0 => Err("attempting to get the type of an empty array!".into()),
                _ => Ok(Type::Array(Box::new(data.get(0).unwrap().convert()?), data.len()))
            },
            Term::Union(data) => data.convert(),
            Term::Struct(data) => {
                let mut result = HashMap::new();
                for (key, val) in data {
                    result.insert(key.clone(), val.convert()?);
                }
                Ok(Type::Struct(result))
            },
            Term::Tuple(data, fields) => {
                let mut result = Vec::new();
                for val in data {
                    result.push(val.convert()?);
                }
                Ok(Type::Tuple(result, fields.clone()))
            },
            Term::Function(func) => match *func.clone() {
                Expression::Annotation { expr, kind } => match kind {
                    Type::Function { from, to } => Ok(Type::Function { from, to }),
                    _ => Err("function term value not a function!".into())
                }
                _ => Err("function term value does not have an annotation!".into())
            },
        }
    }
}

impl Type {
    /// Get the default value of a type. Throws an error if it doesn't exist.
    pub fn default(&self) -> Result<Term> {
        match self {
            Type::Empty => Err("attempting to take the default term for empty".into()),
            Type::Error => Err("attempting to take the default term for error".into()),
            Type::Unit => Ok(Term::Unit()),
            Type::Boolean => Ok(Term::Boolean(false)),
            Type::Natural => Ok(Term::Natural(0)),
            Type::Integer => Ok(Term::Integer(0)),
            Type::Float => Ok(Term::Float(0.0)),
            Type::String => Ok(Term::String(String::new())),
            Type::List(data) => Ok(Term::List(Vec::<Term>::new())),
            Type::Array(data, len) => Ok(Term::Array(vec![data.default()?; *len])),
            Type::Slice(_) => Err("attempting to take the default term of a slice".into()),
            Type::Union(data) => Err("attempting to take the default term of a union".into()),
            Type::Struct(data) => {
                let mut result = HashMap::new();
                for (key, val) in data {
                    result.insert(key.clone(), val.default()?);
                }
                Ok(Term::Struct(result))
            },
            Type::Tuple(data, fields) => {
                let mut result = Vec::new();
                for kind in data {
                    result.push(kind.default()?)
                }
                Ok(Term::Tuple(result, fields.clone()))
            },
            Type::Function { from, to } =>
                Err("attempting to take the default term of a function type".into()),
        }
    }
}

impl core::fmt::Display for Expression {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Type::Empty => write!(f, "⊤"),
            Type::Error => write!(f, "⊥"),
            Type::Unit => write!(f, "unit"),
            Type::Boolean => write!(f, "bool"),
            Type::Natural => write!(f, "nat"),
            Type::Integer => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "str"),
            Type::List(data) => write!(f, "list[{}]", data),
            Type::Array(data, len) => write!(f, "array[{}, {}]", data, len),
            Type::Slice(data) => write!(f, "slice[{}]", data),
            Type::Union(data) => {
                write!(f, "union[")?;
                for (i, val) in data.iter().enumerate() {
                    write!(f, "{}", val)?;
                    if !(i == data.len() - 1) {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            Type::Struct(data) => {
                write!(f, "struct[")?;
                for (i, (key, val)) in data.iter().enumerate() {
                    write!(f, "{}: {}", key, val)?;
                    if !(i == data.len() - 1) {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            Type::Tuple(data, fields) =>  {
                write!(f, "tuple[")?;
                for (i, (val, ident)) in std::iter::zip(data, fields).enumerate() {
                    match ident {
                        Some(key) => write!(f, "{}: {}", key, val)?,
                        None => write!(f, "{}", val)?
                    }
                    if !(i == data.len() - 1) {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            Type::Function { from, to } => write!(f, "{}->{}", from, to),
        }
    }
}

// hatehatehate that you can't implement a trait for foreign types
// let me impl display for vec god dammit
impl core::fmt::Display for Term {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Term::Unit() => write!(f, "∅"),
            Term::Boolean(term) => write!(f, "{}", term),
            Term::Natural(term) => write!(f, "{}", term),
            Term::Integer(term) => write!(f, "{}", term),
            Term::Float(term) => write!(f, "{}", term),
            Term::String(data) => write!(f, "\"{}\"", data),
            Term::List(data) => write!(f, "[{:?}]", data),
            Term::Array(data) => write!(f, "[{:?}]", data),
            Term::Union(data) => write!(f, "{{{:?}}}", data),
            Term::Struct(term) => write!(f, "{{{:?}}}", term),
            Term::Tuple(data, fields) => write!(f, "({:?})", data),
            Term::Function(expr) => write!(f, "{}", *expr),
        }
    }
}

impl Context {
    pub fn new() -> Self {
        Context(HashMap::new())
    }
    pub fn get(&self, k: &Identifier) -> Option<&Term> {
        self.0.get(k)
    }
    pub fn insert(&mut self, k: Identifier, v: Term) -> Option<Term> {
        self.0.insert(k, v)
    }
}
