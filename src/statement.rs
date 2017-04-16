use expression;

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    Assignment(String, expression::Expression),
    ProcedureCall(String, Vec<expression::Expression>),
    Return(Option<expression::Expression>),
}

#[test]
fn test_return_statements() {
    use ada_grammar;
    assert_eq!(ada_grammar::statement("return 11 + X;"),
        Ok(Statement::Return(Some(expression::Expression::Binary(
            "+".to_string(),
            Box::new(expression::Expression::FloatValue(11.0)),
            Box::new(expression::Expression::Variable("X".to_string())))))));
}

#[test]
fn test_assignments() {
    use ada_grammar;
    assert_eq!(ada_grammar::statement("C := 11 + X;"),
        Ok(Statement::Assignment("C".to_string(),
            expression::Expression::Binary(
                "+".to_string(),
                Box::new(expression::Expression::FloatValue(11.0)),
                Box::new(expression::Expression::Variable("X".to_string()))))));
}

#[test]
fn test_procedure_calls() {
    use ada_grammar;
    assert_eq!(ada_grammar::statement("Fire_Missile;"),
               Ok(Statement::ProcedureCall("Fire_Missile".to_string(), vec![])));
    assert_eq!(ada_grammar::statement("Set_Bank_Angle(322, 4);"),
               Ok(Statement::ProcedureCall("Set_Bank_Angle".to_string(),
                                           vec![expression::Expression::FloatValue(322.0),
                                                expression::Expression::FloatValue(4.0)])));
}
