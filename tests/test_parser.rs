use chrysanthemum::ast::*;
use chrysanthemum::parser::*;
use chrysanthemum::util::*;

#[test]
fn test_simple_phrases() {
    assert_eq!(parse_str("123"), Ok(Const(123, Type::Empty)));
    assert_eq!(parse_str("x12"), Ok(Var("x12")));
    assert_eq!(parse_str("x12x2"), Ok(Var("x12x2")));
    // so i _don't_ want these to be valid identifiers:
    // but i actually have no idea why my peg is rejecting them lmao
    assert!(parse_str("12x").is_err());
    assert!(parse_str("12x23").is_err());
}

#[test]
fn test_simple_annotations() {
    assert_eq!(parse_str("t: int"), Ok(Ann(Var("t"), Type::Integer)));
    assert_eq!(parse_str("12: nat"), Ok(Ann(Const(12, Type::Empty), Type::Natural)));
    assert!(parse_str("t: fake").is_err());
}

#[test]
fn test_simple_expressions() {
    assert_eq!(parse_str("λx.y"), Ok(Abs("x", Var("y"))));
    assert_eq!(parse_str("λ x.y"), Ok(Abs("x", Var("y"))));
    assert_eq!(parse_str("λx.y"), Ok(Abs("x", Var("y"))));
    assert_eq!(parse_str("lambda x . y"), Ok(Abs("x", Var("y"))));
    assert!(parse_str("(λx.y)").is_err()); // fixme: should be fine
    assert_eq!(parse_str("(λx.y) x"), Ok(App(Abs("x", Var("y")), Var("x"))));
    assert_eq!(parse_str("(λx.y) x"), Ok(App(Abs("x", Var("y")), Var("x"))));
    assert_eq!(parse_str("if x then y else z"), Ok(Cond(Var("x"), Var("y"), Var("z"))));
    assert_eq!(parse_str("if xeme then yak else zebra"), Ok(Cond(Var("xeme"), Var("yak"), Var("zebra"))));
    assert_eq!(parse_str("if 413 then 612 else 1025"), Ok(Cond(Const(413, Type::Empty), Const(612, Type::Empty), Const(1025, Type::Empty)))); // invalid, but should parse
}

#[test]
fn test_complex_expressions() {
    assert_eq!(parse_str("(λy.if y then 0 else 1) z"), Ok(App(Abs("y", Cond(Var("y"), Const(0, Type::Empty), Const(1, Type::Empty))), Var("z"))));
}

#[test]
fn test_file() {

}

