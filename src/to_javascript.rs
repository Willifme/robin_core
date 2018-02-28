use analysis::table::Table;
use error::Error;

pub trait ToJavaScript {
    fn eval(&self, variable_table: &Table<String>) -> Result<String, Error>;
}
