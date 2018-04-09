extern crate robin_core;

#[cfg(test)]
mod parser_tests {
    use robin_core::ast::{BooleanExpression, Expression};
    use robin_core::table::Table;

    #[test]
    fn find_known_local_variable_without_parent() {
        let mut table = Table::new(None);

        table.insert(
            "example".to_string(),
            Expression::Boolean(BooleanExpression::new(true)),
        );

        assert_eq!(
            Some(&Expression::Boolean(BooleanExpression::new(true))),
            table.get(&"example".to_string())
        );
    }

    #[test]
    fn find_known_local_variable_with_parent() {
        let parent = Table::new(None);

        let mut table = Table::new(Some(Box::new(&parent)));

        table.insert(
            "example".to_string(),
            Expression::Boolean(BooleanExpression::new(true)),
        );

        assert_eq!(
            Some(&Expression::Boolean(BooleanExpression::new(true))),
            table.get(&"example".to_string())
        );
    }

    #[test]
    fn find_unknown_local_variable_without_parent() {
        let table = Table::<Expression>::new(None);

        assert_eq!(None, table.get(&"example".to_string()));
    }

    #[test]
    fn find_unknown_local_variable_with_parent() {
        let parent = Table::new(None);

        let table = Table::<Expression>::new(Some(Box::new(&parent)));

        assert_eq!(None, table.get(&"example".to_string()));
    }
}
