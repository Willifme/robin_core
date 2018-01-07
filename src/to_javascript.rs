use error::Error;

pub trait ToJavaScript {
    fn eval(&self) -> Result<String, Error>;
}
