#![allow(non_upper_case_globals)]

use chrysanthemum::ast::*;
use chrysanthemum::parser::*;
use chrysanthemum::util::*;

#[test]
fn test_simple_phrases() {
    assert_eq!(parse_lambda("123"), Ok(Const(123)));
    assert_eq!(parse_lambda("x12"), Ok(Var("x12")));
    assert_eq!(parse_lambda("x12x2"), Ok(Var("x12x2")));
    // so i _don't_ want these to be valid identifiers:
    // but i actually have no idea why my peg is rejecting them lmao
    assert!(parse_lambda("12x").is_err());
    assert!(parse_lambda("12x23").is_err());
}

#[test]
fn test_simple_annotations() {
    assert_eq!(parse_lambda("t: int"), Ok(Ann(Var("t"), Int)));
    assert_eq!(parse_lambda("12: nat"), Ok(Ann(Const(12), Nat)));
    assert!(parse_lambda("t: fake").is_err());
}

#[test]
fn test_simple_expressions() {
    assert_eq!(parse_lambda("λx.y"), Ok(Abs("x", Var("y"))));
    assert_eq!(parse_lambda("λ x.y"), Ok(Abs("x", Var("y"))));
    assert_eq!(parse_lambda("λx.y"), Ok(Abs("x", Var("y"))));
    assert_eq!(parse_lambda("lambda x . y"), Ok(Abs("x", Var("y"))));
    assert_eq!(parse_lambda("(λx.y)"), Ok(Abs("x", Var("y"))));
    assert_eq!(parse_lambda("(λx.y) x"), Ok(App(Abs("x", Var("y")), Var("x"))));
    assert_eq!(parse_lambda("(λx.y) x"), Ok(App(Abs("x", Var("y")), Var("x"))));
    assert_eq!(parse_lambda("if x then y else z"), Ok(Cond(Var("x"), Var("y"), Var("z"))));
    assert_eq!(parse_lambda("if xeme then yak else zebra"), Ok(Cond(Var("xeme"), Var("yak"), Var("zebra"))));
    assert_eq!(parse_lambda("if 413 then 612 else 1025"), Ok(Cond(Const(413), Const(612), Const(1025)))); // invalid, but should parse
}

#[test]
fn test_complex_expressions() {
    assert_eq!(parse_lambda("(λy.if y then 0 else 1) z"), Ok(App(Abs("y", Cond(Var("y"), Const(0), Const(1))), Var("z"))));
}

#[test]
fn test_complex_annotations() {
    assert_eq!(parse_lambda("(lambda x . y)  : int"), Ok(Ann(Abs("x", Var("y")), Int)));
    assert_eq!(parse_lambda("((lambda x. y): (int -> int)) 413: int"), Ok(App(Ann(Abs("x", Var("y")), Func(Int, Int) ), Ann(Const(413), Int))));
    assert_eq!(parse_lambda("if 0: bool then 1: bool else 2: int"), Ok(Cond(Ann(Const(0), Bool), Ann(Const(1), Bool), Ann(Const(2), Int))));
    assert_eq!(parse_lambda("(lambda x. if x then 1: bool else 0: bool): (int -> bool)"), Ok(Ann(Abs("x", Cond(Var("x"), Ann(Const(1), Bool), Ann(Const(0), Bool))), Func(Int, Bool))));
    assert_eq!(parse_lambda("(lambda x. if x then 1: int else 0: int): (bool -> int)"), Ok(Ann(Abs("x", Cond(Var("x"), Ann(Const(1), Int), Ann(Const(0), Int))), Func(Bool, Int))));
    assert_eq!(parse_lambda("(lambda x. if x then 0 else 1): (bool -> bool)"), Ok(Ann(Abs("x", Cond(Var("x"), Const(0), Const(1))), Func(Bool, Bool))));
}

const program: &'static str =
"func foo(x): (int -> int) =
  if this =
    that
  else =
    this

hello
foo
bar
baz
func foo: (bool -> int) =
  this
";

const lexed: &'static str =
"func foo(x): (int -> int) = {
  if this = {
    that;
  };
  else = {
    this;
  };
};
hello;
foo;
bar;
baz;
func foo: (bool -> int) = {
  this;
};";

#[test]
fn test_lexer() {
    let result = lex(program);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), lexed);
}

#[test]
fn test_parser() {
    let result = parse_lang(lexed);
    assert!(result.is_err());

    let file = std::fs::read_to_string("tests/src/negate.nim");
    assert!(file.is_ok());
    let result = parse_lang(&file.unwrap());
    assert!(result.is_err());

    let file = std::fs::read_to_string("tests/src/fib.nim");
    assert!(file.is_ok());
    let result = parse_lang(&file.unwrap());
    assert!(result.is_err());
}
