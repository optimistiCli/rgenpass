use char_class::CharClass;
use char_class::interval::IntervalCharClass;
use char_class::list::ListCharClass;
use char_class::accu::Accumulator;

pub mod char_class;

fn main() {
    let lower = IntervalCharClass::new('a', 'z');
    let special = ListCharClass::new(&['!', '$', '%', '@', '#']);
    let mut accu = Accumulator::new();
    accu.collect(&lower);
    accu.collect(&special);

    println!("{}-{}-{}", lower.get_char(), lower.get_char(), lower.get_char());
    println!("{}-{}-{}", special.get_char(), special.get_char(), special.get_char());
    println!("{}", accu.to_string())
}
