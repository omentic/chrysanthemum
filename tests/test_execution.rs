use chrysanthemum::ast::*;
use chrysanthemum::simple::*;
use chrysanthemum::util::*;

#[test]
fn test_simple() {
    let context = Context::new();
    assert_eq!(context.execute(Const(Term::Boolean(false))).unwrap(), Term::Boolean(false));
    assert_eq!(context.execute(Const(Term::Natural(123))).unwrap(), Term::Natural(123));
    assert_eq!(context.execute(Const(Term::Integer(123))).unwrap(), Term::Integer(123));
    assert!(context.execute(Var("x")).is_err());
}

#[test]
fn test_complex() {
    let mut context = Context::new();
    context.insert_term(String::from("x"), Term::Natural(413));
    context.insert_term(String::from("y"), Term::Boolean(true));
    assert_eq!(context.execute(Var("x")).unwrap(), Term::Natural(413));
    assert_eq!(context.execute(Cond(Var("y"), Const(Term::Integer(612)),
        Var("x"))).unwrap(), Term::Integer(612));
    assert_eq!(context.execute(App(Abs("z", Cond(Const(Term::Boolean(false)),
        Var("x"), Var("z"))), Const(Term::Integer(1025)))).unwrap(), Term::Integer(1025));
}
