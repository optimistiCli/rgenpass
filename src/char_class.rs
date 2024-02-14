pub mod interval;
pub mod accu;
pub mod array;

pub trait CharClass: Iterator<Item = char> {
    fn chars(&self) -> impl Iterator<Item = char>;
}

// pub trait Emitter<T>: CharClass {
//     fn emit(&self) -> impl IntoIterator<Item = T>;
// }
