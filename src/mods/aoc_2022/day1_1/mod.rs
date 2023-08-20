use std::env::Args;
pub fn main<T>(_: Box<T>)
where
    T: Iterator<Item = String> + ?Sized,
{
}
