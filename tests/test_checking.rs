#![allow(non_upper_case_globals)]

use chrysanthemum::ast::*;
use chrysanthemum::parser::*;
use chrysanthemum::simple::*;
use chrysanthemum::util::*;

// rust you KNOW these are &'static strs
const sanity_check: &'static str = "413: int";
const negate: &'static str = "(λx. if x then 0 else 1): (bool -> bool)";
const basic_abstraction: &'static str = "(λx. x): (int -> int)";
const basic_application: &'static str = "((λx. x): (int -> int)) 413";
const correct_cond_abs: &'static str = "(λx. if x then 1 else 0): (bool -> int)";
const correct_cond: &'static str = "if 0 then 1: nat else 0: nat";
const not_inferrable: &'static str = "(λx. (λy. (λz. if x then y else z)))";
const incorrect_branches: &'static str = "if 0: bool then 1: bool else 2: int";
const incorrect_cond_abs: &'static str = "(λx. if x then 1: bool else 0: bool): (int -> bool)";

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
    assert_eq!(infer(Context::new(), parse_lambda(sanity_check).unwrap()), Ok(Int));
    assert_eq!(infer(Context::new(), parse_lambda(negate).unwrap()), Ok(Func(Bool, Bool)));
    assert_eq!(infer(Context::new(), parse_lambda(basic_abstraction).unwrap()), Ok(Func(Int, Int)));
    assert_eq!(infer(Context::new(), parse_lambda(basic_application).unwrap()), Ok(Int));
    assert_eq!(infer(Context::new(), parse_lambda(correct_cond_abs).unwrap()), Ok(Func(Bool, Int)));
    assert_eq!(infer(Context::new(), parse_lambda(correct_cond).unwrap()), Ok(Nat));
    assert!(infer(Context::new(), parse_lambda(not_inferrable).unwrap()).is_err());
    assert!(infer(Context::new(), parse_lambda(incorrect_branches).unwrap()).is_err());
    assert!(infer(Context::new(), parse_lambda(incorrect_cond_abs).unwrap()).is_err());
}

#[test]
fn test_checking() {
    // uninteresting
    assert!(check(Context::new(), parse_lambda(sanity_check).unwrap(), Int).is_ok());
    assert!(check(Context::new(), parse_lambda(negate).unwrap(), Func(Bool, Bool)).is_ok());
    assert!(check(Context::new(), parse_lambda(basic_abstraction).unwrap(), Func(Int, Int)).is_ok());
    assert!(check(Context::new(), parse_lambda(basic_application).unwrap(), Int).is_ok());
    assert!(check(Context::new(), parse_lambda(correct_cond_abs).unwrap(), Func(Bool, Int)).is_ok());
    assert!(check(Context::new(), parse_lambda(correct_cond).unwrap(), Nat).is_ok());
    assert!(check(Context::new(), parse_lambda(incorrect_branches).unwrap(), Empty).is_err());
    assert!(check(Context::new(), parse_lambda(incorrect_cond_abs).unwrap(), Empty).is_err());

    // more fun
    assert_eq!(check(Context::new(), parse_lambda(not_inferrable).unwrap(), Func(Bool, Func(Int, Func(Int, Int)))), Ok(()));
    assert_eq!(check(Context::new(), parse_lambda(not_inferrable).unwrap(), Func(Bool, Func(Nat, Func(Nat, Nat)))), Ok(()));
    assert_eq!(check(Context::new(), parse_lambda(not_inferrable).unwrap(), Func(Bool, Func(Unit, Func(Unit, Unit)))), Ok(()));
}
