#![allow(non_upper_case_globals)]

use chrysanthemum::ast::*;
use chrysanthemum::bidirectional::*;
use chrysanthemum::parser::*;
use chrysanthemum::util::*;

// rust you KNOW these are &'static strs
const sanity_check: &'static str = "413: int";
const negate: &'static str = "(λx. if x then false else true): (bool -> bool)";
const basic_abstraction: &'static str = "(λx. x): (int -> int)";
const basic_application: &'static str = "((λx. x): (int -> int)) 413";
const correct_cond_abs: &'static str = "(λx. if x then 1 else 0): (bool -> int)";
const correct_cond: &'static str = "if false then 1: nat else 0: nat";
const not_inferrable: &'static str = "(λx. (λy. (λz. if x then y else z)))";
const incorrect_branches: &'static str = "if false: bool then true: bool else 2: int";
const incorrect_cond_abs: &'static str = "(λx. if x then true: bool else false: bool): (int -> bool)";

#[test]
fn test_parsing_succeeds() {
    assert!(parse_lambda(sanity_check).is_ok());
    assert!(parse_lambda(negate).is_ok());
    assert!(parse_lambda(basic_abstraction).is_ok());
    assert!(parse_lambda(basic_application).is_ok());
    assert!(parse_lambda(correct_cond_abs).is_ok());
    assert!(parse_lambda(correct_cond).is_ok());
    assert!(parse_lambda(not_inferrable).is_ok());
    assert!(parse_lambda(incorrect_branches).is_ok());
    assert!(parse_lambda(incorrect_cond_abs).is_ok());
}

#[test]
fn test_inference() {
    let context = Context::new();
    assert_eq!(context.infer(parse_lambda(sanity_check).unwrap()).unwrap(), Int);
    assert_eq!(context.infer(parse_lambda(negate).unwrap()).unwrap(), Func(Bool, Bool));
    assert_eq!(context.infer(parse_lambda(basic_abstraction).unwrap()).unwrap(), Func(Int, Int));
    assert_eq!(context.infer(parse_lambda(basic_application).unwrap()).unwrap(), Int);
    assert_eq!(context.infer(parse_lambda(correct_cond_abs).unwrap()).unwrap(), Func(Bool, Int));
    assert_eq!(context.infer(parse_lambda(correct_cond).unwrap()).unwrap(), Nat);
    assert!(context.infer(parse_lambda(not_inferrable).unwrap()).is_err());
    assert!(context.infer(parse_lambda(incorrect_branches).unwrap()).is_err());
    assert!(context.infer(parse_lambda(incorrect_cond_abs).unwrap()).is_err());
}

#[test]
fn test_checking() {
    let context = Context::new();

    // uninteresting
    assert!(context.check(parse_lambda(sanity_check).unwrap(), &Int).is_ok());
    assert!(context.check(parse_lambda(negate).unwrap(), &Func(Bool, Bool)).is_ok());
    assert!(context.check(parse_lambda(basic_abstraction).unwrap(), &Func(Int, Int)).is_ok());
    assert!(context.check(parse_lambda(basic_application).unwrap(), &Int).is_ok());
    assert!(context.check(parse_lambda(correct_cond_abs).unwrap(), &Func(Bool, Int)).is_ok());
    assert!(context.check(parse_lambda(correct_cond).unwrap(), &Nat).is_ok());
    assert!(context.check(parse_lambda(incorrect_branches).unwrap(), &Unit).is_err());
    assert!(context.check(parse_lambda(incorrect_cond_abs).unwrap(), &Error).is_err());

    // more fun
    assert!(context.check(parse_lambda(not_inferrable).unwrap(), &Func(Bool, Func(Int, Func(Int, Int)))).is_ok());
    assert!(context.check(parse_lambda(not_inferrable).unwrap(), &Func(Bool, Func(Nat, Func(Nat, Nat)))).is_ok());
    assert!(context.check(parse_lambda(not_inferrable).unwrap(), &Func(Bool, Func(Unit, Func(Unit, Unit)))).is_ok());
}
