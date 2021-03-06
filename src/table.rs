///! # Table
///!
///! Define a module which is a hashmap with a child-parent association.
///! This module is used for the variable, function and alias table.
///! These tables will first look in their hashmap then their parent (etc.) until a value has been found
use std::boxed::Box;
use std::collections::HashMap;

/// Create a type alias for the table containing the symbols
/// All items are acccessible via their name
/// The `T` is the value inherited from `Table`
pub type Container<'a, T> = HashMap<String, T>;

/// This is the generic container for all symbol tables
/// `T` must implement `ToJavaScript`
#[derive(Clone, Debug)]
pub struct Table<'a, T: 'a> {
    /// This is the optional parent struct.
    /// The parent will most likely have a value unless the scope is global
    parent: Option<Box<&'a Table<'a, T>>>,
    /// The container is the contents of the table
    pub container: Box<Container<'a, T>>,
}

impl<'a, T> Table<'a, T> {
    pub fn new(table: Option<Box<&'a Table<'a, T>>>) -> Table<'a, T> {
        Table {
            parent: table,
            container: Box::new(Container::new()),
        }
    }

    /// Insert a value into the table
    pub fn insert(&mut self, key: String, value: T) {
        self.container.insert(key, value);
    }

    /// Attempt to get a value from the table
    /// If the value can't be found within this scope. try the parent.
    /// After descending through all the scopes, return an undefiend error
    pub fn get(&self, key: &'a String) -> Option<&T> {
        // First we get from the local scope
        match self.container.get(key) {
            Some(value) => Some(value),
            None => self.parent.as_ref().and_then(|i| i.get(key)),
        }
    }
    /// Attempt to get a mutable value from the table
    /// If the value can't be found within this scope. try the parent.
    /// After descending through all the scopes, return an undefiend error
    pub fn get_mut(&mut self, key: &'a String) -> Option<&T> {
        // First we get from the local scope
        match self.container.get_mut(key) {
            Some(value) => Some(value),
            None => self.parent.as_ref().and_then(|i| i.get(key)),
        }
    }
}
