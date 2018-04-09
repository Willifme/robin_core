use error::Error;
use stdlib::Stdlib;

pub trait ToJavaScript {
    fn eval(&mut self, variable_table: &mut Stdlib) -> Result<String, Error>;
}
