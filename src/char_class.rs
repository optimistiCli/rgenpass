pub mod interval;
pub mod list;
pub mod accu;

pub trait CharClass {
    fn get_char(&self) -> char;
    fn get_iter(&self) -> impl IntoIterator;
}

pub trait RefIterable<'a>: CharClass {
    fn get_ref_iter(&self) -> impl IntoIterator<Item = &'a char>;
}
pub trait CharIterable: CharClass {
    fn get_char_iter(&self) -> impl IntoIterator<Item = char>;
}

pub trait GenAccumulator<T> {
    fn take_in(&mut self, a_iterable: impl IntoIterator<Item = T>);
}