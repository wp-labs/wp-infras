pub trait ConfParser<T: ?Sized> {
    fn from_parse(items: &T) -> Self;
    fn validate(items: &T) -> Result<(), String>;
}
