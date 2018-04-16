extern crate robin_core;

mod eval_expr_tests {
    use robin_core::ast::{BooleanExpression, Expression, IdentifierExpression, ListExpression,
                          NumberExpression};
    use robin_core::error::Error;
    use robin_core::stdlib::Stdlib;
    use robin_core::table::Table;
    use robin_core::to_javascript::ToJavaScript;

    #[test]
    fn array_literal_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Number(NumberExpression::new(60.0))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Ok(String::from("[60,50]"))
        );
    }

    #[test]
    fn lambda_with_multiple_args_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new("lambda".to_string()))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![
                Box::new(Expression::Identifier(IdentifierExpression::new("x".to_string()))),
                Box::new(Expression::Identifier(IdentifierExpression::new("y".to_string()))),
            ]))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![
                Box::new(Expression::Identifier(IdentifierExpression::new("+".to_string()))),
                Box::new(Expression::Number(NumberExpression::new(50.0))),
                Box::new(Expression::Number(NumberExpression::new(50.0))),
            ]))),
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Ok(String::from("((x,y) => { 100 })"))
        );
    }

    #[test]
    fn lambda_with_one_arg_should_evalute_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new("lambda".to_string()))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![
                Box::new(Expression::Identifier(IdentifierExpression::new("x".to_string()))),
            ]))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![
                Box::new(Expression::Identifier(IdentifierExpression::new("+".to_string()))),
                Box::new(Expression::Number(NumberExpression::new(50.0))),
                Box::new(Expression::Number(NumberExpression::new(50.0))),
            ]))),
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Ok(String::from("((x) => { 100 })"))
        );
    }

    #[test]
    fn lambda_with_no_args_should_evalute_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new("lambda".to_string()))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![]))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![
                Box::new(Expression::Identifier(IdentifierExpression::new("+".to_string()))),
                Box::new(Expression::Number(NumberExpression::new(50.0))),
                Box::new(Expression::Number(NumberExpression::new(50.0))),
            ]))),
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Ok(String::from("(() => { 100 })"))
        );
    }

    #[test]
    fn lambda_with_multiple_args_and_no_body_should_return_an_error() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new("lambda".to_string()))),
            Box::new(Expression::Identifier(IdentifierExpression::new("x".to_string()))),
            Box::new(Expression::Identifier(IdentifierExpression::new("y".to_string()))),
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Err(Error::invalid_expression("non-list given to lambda expression".to_string()))
        );
    }

    #[test]
    fn lambda_with_one_arg_and_no_body_should_return_an_error() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new("lambda".to_string()))),
            Box::new(Expression::Identifier(IdentifierExpression::new("x".to_string()))),
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Err(Error::too_few_arguments("lambda".to_string()))
        );
    }

    #[test]
    fn lambda_with_no_args_and_no_body_should_return_an_error() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new("lambda".to_string())))
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Err(Error::too_few_arguments("lambda".to_string()))
        );
    }

    #[test]
    fn quote_with_no_args_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_quoted(vec![
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Ok(String::from("\"()\""))
        );
    }

    #[test]
    fn quote_with_one_args_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_quoted(vec![
            Box::new(Expression::Number(NumberExpression::new(50.0)))
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Ok(String::from("\"(50)\""))
        );
    }

    #[test]
    fn quote_with_a_list_expr_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_quoted(vec![
            Box::new(Expression::List(ListExpression::new_unquoted(vec![
                Box::new(Expression::Number(NumberExpression::new(50.0)))
            ]))),
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Ok(String::from("\"((50))\""))
        );
    }

    #[test]
    fn function_literal_with_too_few_args_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "function".to_string(),
            ))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![]))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Err(Error::too_few_arguments("function definition".to_string()))
        );
    }

    #[test]
    fn function_literal_with_no_args_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "function".to_string(),
            ))),
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "hello".to_string(),
            ))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![]))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![
                Box::new(Expression::Identifier(IdentifierExpression::new(
                    "+".to_string(),
                ))),
                Box::new(Expression::Number(NumberExpression::new(50.0))),
                Box::new(Expression::Number(NumberExpression::new(50.0))),
            ]))),
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Ok(String::from("function hello(){ 100; }"))
        );
    }

    #[test]
    fn function_literal_with_one_arg_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "function".to_string(),
            ))),
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "hello".to_string(),
            ))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![
                Box::new(Expression::Identifier(IdentifierExpression::new(
                    "x".to_string(),
                ))),
            ]))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Ok(String::from("function hello(x){ 50; }"))
        );
    }

    #[test]
    fn function_literal_with_multiple_arg_should_return_an_error() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "function".to_string(),
            ))),
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "hello".to_string(),
            ))),
            Box::new(Expression::List(ListExpression::new_unquoted(vec![
                Box::new(Expression::Identifier(IdentifierExpression::new(
                    "x".to_string(),
                ))),
                Box::new(Expression::Identifier(IdentifierExpression::new(
                    "y".to_string(),
                ))),
            ]))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Ok(String::from("function hello(x,y){ 50; }"))
        );
    }

    #[test]
    fn binding_with_one_argument_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "var".to_string(),
            ))),
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "something".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Ok(String::from("var something = 50"))
        );
    }

    #[test]
    fn binding_with_no_arguments_should_return_an_error() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "var".to_string(),
            ))),
        ]));

        let err = Err(Error::too_few_arguments("binding".to_string()));

        assert_eq!(expr.eval(&mut stdlib), err);
    }

    #[test]
    fn return_with_one_argument_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "return".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("return 50")));
    }

    #[test]
    fn return_with_no_arguments_should_return_an_error() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "return".to_string(),
            ))),
        ]));

        let err = Err(Error::too_few_arguments("return".to_string()));

        assert_eq!(expr.eval(&mut stdlib), err);
    }

    #[test]
    fn return_with_more_than_one_arguments_should_return_an_error() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "return".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        let err = Err(Error::too_many_arguments("return".to_string()));

        assert_eq!(expr.eval(&mut stdlib), err);
    }

    #[test]
    fn plus_unary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "+".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("+50")));
    }

    #[test]
    fn minus_unary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "-".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("-50")));
    }

    #[test]
    fn not_unary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "!".to_string(),
            ))),
            Box::new(Expression::Boolean(BooleanExpression::new(true))),
        ]));

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("!true")));
    }

    #[test]
    fn increment_unary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "++".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("++50")));
    }

    #[test]
    fn bitwise_not_unary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "~".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("~50")));
    }

    #[test]
    fn typeof_unary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "typeof".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("typeof 50")));
    }

    #[test]
    fn delete_unary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "delete".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("delete 50")));
    }

    #[test]
    fn binary_op_with_more_than_two_exprs_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "+".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("50+50+50")));
    }

    #[test]
    fn plus_binary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "+".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("100")));
    }

    #[test]
    fn minus_binary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "-".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("0")));
    }

    #[test]
    fn times_binary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "*".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("2500")));
    }

    #[test]
    fn divide_binary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "/".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("1")));
    }

    #[test]
    fn divide_binary_op_should_return_an_error() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "/".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
            Box::new(Expression::Number(NumberExpression::new(0.0))),
        ]));

        let err = Err(Error::invalid_expression(
            "Divide by zero encountered on numeric literal binary operation".to_string(),
        ));

        assert_eq!(expr.eval(&mut stdlib), err);
    }

    #[test]
    fn modulo_binary_op_should_evaluate_correctly() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "%".to_string(),
            ))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
            Box::new(Expression::Number(NumberExpression::new(50.0))),
        ]));

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("0")));
    }

    #[test]
    fn if_with_no_args_should_return_an_err() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "if".to_string(),
            ))),
        ]));

        let err = Err(Error::too_few_arguments("if statement".to_string()));

        assert_eq!(expr.eval(&mut stdlib), err)
    }

    #[test]
    fn if_with_no_expression_after_condition_should_return_an_err() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "if".to_string(),
            ))),
            Box::new(Expression::Boolean(BooleanExpression::new(true))),
        ]));

        let err = Err(Error::too_few_arguments(
            "if statement condition".to_string(),
        ));

        assert_eq!(expr.eval(&mut stdlib), err)
    }

    #[test]
    fn if_with_only_one_branch_should_return_an_func() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "if".to_string(),
            ))),
            Box::new(Expression::Boolean(BooleanExpression::new(true))),
            Box::new(Expression::Number(NumberExpression::new(1.0))),
        ]));

        assert_eq!(expr.eval(&mut stdlib), Ok(String::from("if (true) { 1 }")))
    }

    #[test]
    fn if_with_else_branch_should_return_a_func() {
        // The global variable table
        let mut stdlib = Stdlib::new(Table::new(None));

        let mut expr = Expression::List(ListExpression::new_unquoted(vec![
            Box::new(Expression::Identifier(IdentifierExpression::new(
                "if".to_string(),
            ))),
            Box::new(Expression::Boolean(BooleanExpression::new(true))),
            Box::new(Expression::Number(NumberExpression::new(1.0))),
            Box::new(Expression::Number(NumberExpression::new(1.0))),
        ]));

        assert_eq!(
            expr.eval(&mut stdlib),
            Ok(String::from("if (true) { 1 } else { 1 }"))
        )
    }
}
