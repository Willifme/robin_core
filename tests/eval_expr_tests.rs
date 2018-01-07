extern crate robin_core;

mod eval_expr_tests {
    use robin_core::ast::Expression;
    use robin_core::to_javascript::ToJavaScript;
    use robin_core::error::{Error, ErrorLevel, ErrorKind};

    #[test]
    fn if_with_no_args_should_return_an_err() {
        let expr = Expression::FuncCall("if".to_string(), vec![]);

        let err = Err(Error(ErrorLevel::Error,
                            ErrorKind::TooFewArguments,
                            "Too few Arguments applied for if"));

        assert_eq!(expr.eval(), err)
    }

    #[test]
    fn if_with_no_expression_after_condition_should_return_an_err() {
        let expr = Expression::FuncCall("if".to_string(),
                                        vec![Box::new(Expression::Boolean(true))]);

        let err = Err(Error(ErrorLevel::Error,
                            ErrorKind::TooFewArguments,
                            "No expression applied for condition"));

        assert_eq!(expr.eval(), err)
    }

    #[test]
    fn if_with_only_one_branch_should_return_an_func() {
        let expr = Expression::FuncCall("if".to_string(),
                                        vec![Box::new(Expression::Boolean(true)),
                                             Box::new(Expression::Number(1.0))]);

        assert_eq!(expr.eval(), Ok(String::from("if (true) { 1 }")))
    }

    #[test]
    fn if_with_else_branch_should_return_a_func() {
        let expr = Expression::FuncCall("if".to_string(),
                                        vec![Box::new(Expression::Boolean(true)),
                                             Box::new(Expression::Number(1.0)),
                                             Box::new(Expression::Number(1.0))]);

        assert_eq!(expr.eval(), Ok(String::from("if (true) { 1 } else { 1 }")))
    }
}
