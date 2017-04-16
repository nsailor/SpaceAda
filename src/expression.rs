
use std::boxed::Box;

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Binary(String, Box<Expression>, Box<Expression>),
    Variable(String),
    IntValue(i32),
    FloatValue(f32),
    FunctionCall(String, Vec<Expression>),
}

#[test]
fn test_identifiers() {
    use ada_grammar;
    assert_eq!(ada_grammar::identifier("test_x"), Ok("test_x".to_string()));
    assert!(ada_grammar::identifier("3numberfirst").is_err());
    assert!(ada_grammar::identifier("a-n").is_err());
}

#[test]
fn test_int_values() {
    use ada_grammar;
    assert_eq!(ada_grammar::int_value("32"), Ok(32));
    assert_eq!(ada_grammar::int_value("-42"), Ok(-42));
    assert!(ada_grammar::int_value("ab4").is_err());
    assert!(ada_grammar::int_value("-3e").is_err());
}

#[test]
fn test_terminal_expressions() {
    use ada_grammar;
    assert_eq!(ada_grammar::expression("11.0"),
               Ok(Expression::FloatValue(11.0)));
    assert_eq!(ada_grammar::expression("Pitch"),
               Ok(Expression::Variable("Pitch".to_string())));
}

#[test]
fn test_arithmetic() {
    use ada_grammar;
    assert_eq!(ada_grammar::expression("3.0 + 2.0 * 6.0"), Ok(Expression::Binary("+".to_string(),
        Box::new(Expression::FloatValue(3.0)),
        Box::new(Expression::Binary("*".to_string(),
            Box::new(Expression::FloatValue(2.0)),
            Box::new(Expression::FloatValue(6.0)))))));

    assert_eq!(ada_grammar::expression("(3.0 + 2.0) * 6.0"), Ok(Expression::Binary("*".to_string(),
        Box::new(Expression::Binary("+".to_string(),
            Box::new(Expression::FloatValue(3.0)),
            Box::new(Expression::FloatValue(2.0)))),
        Box::new(Expression::IntValue(6)))));
}

#[test]
fn test_function_calls() {
    use ada_grammar;
    assert_eq!(ada_grammar::expression("f(3.0, 2.0)"),
               Ok(Expression::FunctionCall("f".to_string(),
                                           vec![Expression::FloatValue(3.0),
                                                Expression::FloatValue(2.0)])));
}
