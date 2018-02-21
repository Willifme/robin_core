extern crate robin_core;

#[cfg(test)]
mod parser_tests {
    use robin_core::ast::Expression;
    use robin_core::analysis::table::Table;

    #[test]
    fn find_known_local_variable_without_parent() {
        let mut table = Table::new(None);

        table.insert("example", Expression::Boolean(true));

        assert_eq!(Some(&Expression::Boolean(true)), table.get("example"));
    }

    #[test]
    fn find_known_local_variable_with_parent() {
        let mut table = Table::new(Some(Box::new(Table::new(None))));

        table.insert("example", Expression::Boolean(true));

        assert_eq!(Some(&Expression::Boolean(true)), table.get("example"));
    }

    #[test]
    fn find_unknown_local_variable_without_parent() {
        let table = Table::<Expression>::new(None);

        assert_eq!(None, table.get("Example"));
    }

    #[test]
    fn find_unknown_local_variable_with_parent() {
        let table = Table::<Expression>::new(Some(Box::new(Table::new(None))));

        assert_eq!(None, table.get("example"));
    }
}