extern crate robin_core;

mod eval_expr_tests {
    use robin_core::ast::Expression;
    use robin_core::analysis::table::Table;
    use robin_core::to_javascript::ToJavaScript;
    use robin_core::error::Error;
    use robin_core::stdlib::Stdlib;

    #[test]
    fn binding_with_one_argument_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("var".to_string())),
            vec![
                Box::new(Expression::Identifier("something".to_string())),
                Box::new(Expression::Number(50.0))
            ]
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("var something = 50")));
    }

    #[test]
    fn binding_with_no_arguments_should_return_an_error() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("var".to_string())),
            vec![]
        );

        let err = Err(Error::too_few_arguments("binding"));

        assert_eq!(expr.eval(&mut stdlib), err);
    }

    #[test]
    fn return_with_one_argument_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("return".to_string())),
            vec![Box::new(Expression::Number(50.0))],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("return 50")));
    }

    #[test]
    fn return_with_no_arguments_should_return_an_error() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("return".to_string())),
            vec![],
        );

        let err = Err(Error::too_few_arguments("return"));

        assert_eq!(expr.eval(&mut stdlib), err);
    }

    #[test]
    fn return_with_more_than_one_arguments_should_return_an_error() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("return".to_string())),
            vec![
                Box::new(Expression::Number(50.0)),
                Box::new(Expression::Number(50.0)),
            ],
        );

        let err = Err(Error::too_many_arguments("return"));

        assert_eq!(expr.eval(&mut stdlib), err);
    }

    #[test]
    fn plus_unary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("+".to_string())),
            vec![Box::new(Expression::Number(50.0))],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("+50")));
    }

    #[test]
    fn minus_unary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("-".to_string())),
            vec![Box::new(Expression::Number(50.0))],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("-50")));
    }

    #[test]
    fn not_unary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("!".to_string())),
            vec![Box::new(Expression::Boolean(true))],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("!true")));
    }

    #[test]
    fn increment_unary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("++".to_string())),
            vec![Box::new(Expression::Number(50.0))],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("++50")));
    }

    #[test]
    fn bitwise_not_unary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("~".to_string())),
            vec![Box::new(Expression::Number(50.0))],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("~50")));
    }

    #[test]
    fn typeof_unary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("typeof".to_string())),
            vec![Box::new(Expression::Number(50.0))],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("typeof 50")));
    }

    #[test]
    fn delete_unary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("delete".to_string())),
            vec![Box::new(Expression::Number(50.0))],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("delete 50")));
    }

    #[test]
    fn binary_op_with_more_than_two_exprs_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("+".to_string())),
            vec![
                Box::new(Expression::Number(50.0)),
                Box::new(Expression::Number(50.0)),
                Box::new(Expression::Number(50.0)),
            ],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("50+50+50")));
    }

    #[test]
    fn plus_binary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("+".to_string())),
            vec![
                Box::new(Expression::Number(50.0)),
                Box::new(Expression::Number(50.0)),
            ],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("100")));
    }

    #[test]
    fn minus_binary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("-".to_string())),
            vec![
                Box::new(Expression::Number(50.0)),
                Box::new(Expression::Number(50.0)),
            ],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("0")));
    }

    #[test]
    fn times_binary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("*".to_string())),
            vec![
                Box::new(Expression::Number(50.0)),
                Box::new(Expression::Number(50.0)),
            ],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("2500")));
    }

    #[test]
    fn divide_binary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("/".to_string())),
            vec![
                Box::new(Expression::Number(50.0)),
                Box::new(Expression::Number(50.0)),
            ],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("1")));
    }

    #[test]
    fn divide_binary_op_should_return_an_error() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("/".to_string())),
            vec![
                Box::new(Expression::Number(50.0)),
                Box::new(Expression::Number(0.0)),
            ],
        );

        let err = Err(Error::invalid_expression(
            "Divide by zero encountered on numeric literal binary operation",
        ));

        assert_eq!(expr.eval(&mut stdlib), err);
    }

    #[test]
    fn modulo_binary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("%".to_string())),
            vec![
                Box::new(Expression::Number(50.0)),
                Box::new(Expression::Number(50.0)),
            ],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("0")));
    }

    #[test]
    fn if_with_no_args_should_return_an_err() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(Box::new(Expression::Identifier("if".to_string())), vec![]);

        let err = Err(Error::too_few_arguments("if statement"));

        assert_eq!(expr.eval(&mut stdlib), err)
    }

    #[test]
    fn if_with_no_expression_after_condition_should_return_an_err() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("if".to_string())),
            vec![Box::new(Expression::Boolean(true))],
        );

        let err = Err(Error::too_few_arguments("if statement condition"));

        assert_eq!(expr.eval(&mut stdlib), err)
    }

    #[test]
    fn if_with_only_one_branch_should_return_an_func() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("if".to_string())),
            vec![
                Box::new(Expression::Boolean(true)),
                Box::new(Expression::Number(1.0)),
            ],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("if (true) { 1 }")))
    }

    #[test]
    fn if_with_else_branch_should_return_a_func() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        stdlib.populate();

        let mut expr = Expression::FuncCall(
            Box::new(Expression::Identifier("if".to_string())),
            vec![
                Box::new(Expression::Boolean(true)),
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(1.0)),
            ],
        );

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("if (true) { 1 } else { 1 }")))
    }
}
