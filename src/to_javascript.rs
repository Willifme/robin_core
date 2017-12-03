pub trait ToJavaScript {
    fn eval(&self) -> String;
}
