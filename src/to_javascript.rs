use stdlib::Stdlib;
use error::Error;

pub trait ToJavaScript {
    fn eval(&mut self, variable_table: &mut Stdlib) -> Result<String, Error>;
}
