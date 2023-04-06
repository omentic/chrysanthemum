use chrysanthemum::ast::*;
use chrysanthemum::simple::*;
use chrysanthemum::util::*;

#[test]
fn test_simple() {
    assert_eq!(execute(Context::new(), Const(0, Type::Boolean)), Ok(Term(0, Type::Boolean)));
    assert_eq!(execute(Context::new(), Const(123, Type::Natural)), Ok(Term(123, Type::Natural)));
    assert_eq!(execute(Context::new(), Const(123, Type::Integer)), Ok(Term(123, Type::Integer)));
    assert!(execute(Context::new(), Var("x")).is_err());
}

#[test]
fn test_complex() {
    let mut context = Context::new();
    context.insert(String::from("x"), Term(413, Type::Natural));
    context.insert(String::from("y"), Term(1, Type::Boolean));
    assert_eq!(execute(context.clone(), Var("x")), Ok(Term(413, Type::Natural)));
    assert_eq!(execute(context.clone(), Cond(Var("y"), Const(612, Type::Integer), Var("x"))), Ok(Term(612, Type::Integer)));
    assert_eq!(execute(context.clone(), App(Abs("z", Cond(Const(0, Type::Boolean), Var("x"), Var("z"))), Const(1025, Type::Integer))), Ok(Term(1025, Type::Integer)));
}
