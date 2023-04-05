// Simple types for effects

use crate::ast::*;

// bad and wrong and useless
pub struct Effection {
    expr: Expression,
    effect: Effect,
}

// yeah i'm not dealing with this yet
pub enum Effect {
    Empty,
    Total,
    Exn,
    Pure,
    IO,
}

