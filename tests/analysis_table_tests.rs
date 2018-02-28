extern crate robin_core;

#[cfg(test)]
mod parser_tests {
    use robin_core::ast::Expression;
    use robin_core::analysis::table::Table;

    // Fix these tests, they are broken
    // #[test]
    // fn find_known_local_variable_without_parent<'a>() {
    //     let mut table = Table::new(None);

    //     table.insert(&"example".to_string(), Expression::Boolean(true));

    //     assert_eq!(Some(&Expression::Boolean(true)), table.get(&"example".to_string()));
    // }

    // #[test]
    // fn find_known_local_variable_with_parent() {
    //     let mut table = Table::new(Some(Box::new(&Table::new(None))));

    //     table.insert(&"example".to_string(), Expression::Boolean(true));

    //     assert_eq!(Some(&Expression::Boolean(true)), table.get(&"example".to_string()));
    // }

    // #[test]
    // fn find_unknown_local_variable_without_parent() {
    //     let table = Table::<Expression>::new(None);

    //     assert_eq!(None, table.get(&"Example".to_string()));
    // }

    // #[test]
    // fn find_unknown_local_variable_with_parent() {
    //     let parent = Box::new(Table::new(None));

    //     let table = Table::<Expression>::new(Some(parent));

    //     assert_eq!(None, table.get(&"example".to_string()));
    // }
}
