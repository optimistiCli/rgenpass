pub mod interval;
pub mod accu;
pub mod array;

pub trait CharClass: Iterator<Item = char> {
    fn chars(&self) -> Box<dyn Iterator<Item = char> + '_>;
}