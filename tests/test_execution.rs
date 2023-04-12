use chrysanthemum::ast::*;
use chrysanthemum::simple::*;
use chrysanthemum::util::*;

#[test]
fn test_simple() {
    assert_eq!(execute(Context::new(), Const(0)), Ok(Term(0, Empty)));
    assert_eq!(execute(Context::new(), Const(123)), Ok(Term(123, Empty)));
    assert_eq!(execute(Context::new(), Const(123)), Ok(Term(123, Empty)));
    assert!(execute(Context::new(), Var("x")).is_err());
}

#[test]
fn test_complex() {
    let mut context = Context::new();
    context.insert(String::from("x"), Term(413, Empty));
    context.insert(String::from("y"), Term(1, Empty));
    assert_eq!(execute(context.clone(), Var("x")), Ok(Term(413, Empty)));
    assert_eq!(execute(context.clone(), Cond(Var("y"), Const(612), Var("x"))), Ok(Term(612, Empty)));
    assert_eq!(execute(context.clone(), App(Abs("z", Cond(Const(0), Var("x"), Var("z"))), Const(1025))), Ok(Term(1025, Empty)));
}
