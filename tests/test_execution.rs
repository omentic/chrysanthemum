use chrysanthemum::ast::*;
use chrysanthemum::simple::*;
use chrysanthemum::util::*;

#[test]
fn test_simple() {
    let context = Context::new();
    assert_eq!(execute(&context, Const(Term::Boolean(false))), Ok(Term::Boolean(false)));
    assert_eq!(execute(&context, Const(Term::Natural(123))), Ok(Term::Natural(123)));
    assert_eq!(execute(&context, Const(Term::Integer(123))), Ok(Term::Integer(123)));
    assert!(execute(&context, Var("x")).is_err());
}

#[test]
fn test_complex() {
    let mut context = Context::new();
    context.insert(String::from("x"), Term::Natural(413));
    context.insert(String::from("y"), Term::Boolean(true));
    assert_eq!(execute(&context, Var("x")), Ok(Term::Natural(413)));
    assert_eq!(execute(&context, Cond(Var("y"), Const(Term::Integer(612)), Var("x"))), Ok(Term::Integer(612)));
    assert_eq!(execute(&context,
        App(Abs("z", Cond(Const(Term::Boolean(false)), Var("x"), Var("z"))), Const(Term::Integer(1025)))), Ok(Term::Integer(1025)));
}
