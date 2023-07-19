#![allow(non_upper_case_globals)]

use chrysanthemum::ast::*;
use chrysanthemum::parser::*;
use chrysanthemum::util::*;

#[test]
fn test_simple_phrases() {
    assert_eq!(parse_lambda("-123").unwrap(), Const(Term::Integer(-123)));
    assert_eq!(parse_lambda("x12").unwrap(), Var("x12"));
    assert_eq!(parse_lambda("x12x2").unwrap(), Var("x12x2"));
    // so i _don't_ want these to be valid identifiers:
    // but i actually have no idea why my peg is rejecting them lmao
    assert!(parse_lambda("12x").is_err());
    assert!(parse_lambda("12x23").is_err());
}

#[test]
fn test_simple_annotations() {
    assert_eq!(parse_lambda("t: int").unwrap(), Ann(Var("t"), Int));
    assert_eq!(parse_lambda("12: nat").unwrap(), Ann(Const(Term::Natural(12)), Nat));
    assert!(parse_lambda("t: fake").is_err());
}

#[test]
fn test_simple_expressions() {
    assert_eq!(parse_lambda("λx.y").unwrap(), Abs("x", Var("y")));
    assert_eq!(parse_lambda("λ x.y").unwrap(), Abs("x", Var("y")));
    assert_eq!(parse_lambda("λx.y").unwrap(), Abs("x", Var("y")));
    assert_eq!(parse_lambda("lambda x . y").unwrap(), Abs("x", Var("y")));
    assert_eq!(parse_lambda("(λx.y)").unwrap(), Abs("x", Var("y")));
    assert_eq!(parse_lambda("(λx.y) x").unwrap(), App(Abs("x", Var("y")), Var("x")));
    assert_eq!(parse_lambda("(λx.y) x").unwrap(), App(Abs("x", Var("y")), Var("x")));
    assert_eq!(parse_lambda("if x then y else z").unwrap(), Cond(Var("x"), Var("y"), Var("z")));
    assert_eq!(parse_lambda("if xeme then yak else zebra").unwrap(), Cond(Var("xeme"), Var("yak"), Var("zebra")));
    assert_eq!(parse_lambda("if 413 then 612 else 1025").unwrap(), Cond(Const(Term::Natural(413)), Const(Term::Natural(612)), Const(Term::Natural(1025)))); // invalid, but should parse
}

#[test]
fn test_complex_expressions() {
    assert_eq!(parse_lambda("(λy.if y then false else true) z").unwrap(), App(Abs("y", Cond(Var("y"), Const(Term::Boolean(false)), Const(Term::Boolean(true)))), Var("z")));
}

#[test]
fn test_complex_annotations() {
    assert_eq!(parse_lambda("(lambda x . y)  : int").unwrap(), Ann(Abs("x", Var("y")), Int));
    assert_eq!(parse_lambda("((lambda x. y): (int -> int)) -413: int").unwrap(), App(Ann(Abs("x", Var("y")), Func(Int, Int) ), Ann(Const(Term::Integer(-413)), Int)));
    assert_eq!(parse_lambda("if false: bool then true: bool else 2: int").unwrap(), Cond(Ann(Const(Term::Boolean(false)), Bool), Ann(Const(Term::Boolean(true)), Bool), Ann(Const(Term::Natural(2)), Int)));
    assert_eq!(parse_lambda("(lambda x. if x then true: bool else false: bool): (int -> bool)").unwrap(), Ann(Abs("x", Cond(Var("x"), Ann(Const(Term::Boolean(true)), Bool), Ann(Const(Term::Boolean(false)), Bool))), Func(Int, Bool)));
    assert_eq!(parse_lambda("(lambda x. if x then 1: int else 0: int): (bool -> int)").unwrap(), Ann(Abs("x", Cond(Var("x"), Ann(Const(Term::Natural(1)), Int), Ann(Const(Term::Natural(0)), Int))), Func(Bool, Int)));
    assert_eq!(parse_lambda("(lambda x. if x then false else true): (bool -> bool)").unwrap(), Ann(Abs("x", Cond(Var("x"), Const(Term::Boolean(false)), Const(Term::Boolean(true)))), Func(Bool, Bool)));
}
