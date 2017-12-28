use std::collections::HashMap;
use std::boxed::Box;

use to_javascript::ToJavaScript;
use error::{Error, ErrorKind, ErrorLevel};

/// Create a type alias for the table containing the symbols
/// All items are acccessible via their name
/// The `T` is the value inherited from `Table`
pub type Container<T> = HashMap<&'static str, T>;

/// This is the generic container for all symbol tables
/// `T` must implement `ToJavaScript`
pub struct Table<T: ToJavaScript> {
    /// This is the optional parent struct.
    /// The parent will most likely have a value unless the scope is global
    parent: Option<Box<Table<T>>>,
    /// The container is the contents of th table
    container: Box<Container<T>>,
}

impl<T: ToJavaScript> Table<T> {
    pub fn new(table: Option<Box<Table<T>>>) -> Table<T> {
        Table {
            parent: table,
            container: Box::new(Container::new()),
        }
    }

    /// Insert a value into the table
    pub fn insert(&mut self, key: &'static str, value: T) {
        self.container.insert(key, value);
    }

    /// Attempt to get a value from the table
    /// If the value can't be found within this scope. try the parent.
    /// After descending through all the scopes, return an undefiend error
    pub fn get(&self, key: &'static str) -> Result<&T, Error> {
        // First we get from the local scope
        match self.container.get(key) {
            Some(value) => Ok(value),
            None => {
                self.parent
                    .as_ref()
                    // TODO: Improve error message
                    .map_or(Err(
                        Error(ErrorKind::UndefinedVar, ErrorLevel::Error, "Variable not found")),
                        |i| i.get(key))
            }
        }
    }
}
