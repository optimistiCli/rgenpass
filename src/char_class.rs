pub mod interval;
pub mod list;
pub mod accu;

pub trait CharClass {
    fn get_char(&self) -> char;
    fn get_iter(&self) -> impl IntoIterator;
}

pub trait Emitter<T>: CharClass {
    fn emit(&self) -> impl IntoIterator<Item = T>;
}

pub trait Collector<T> {
    fn collect_worker(&mut self, a_emitter: &impl Emitter<T>);
}
