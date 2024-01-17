use char_class::CharClass;
use char_class::interval::IntervalCharClass;
use char_class::list::ListCharClass;
use char_class::accu::Accumulator;

use char_class::{GenAccumulator, CharIterable, RefIterable};

pub mod char_class;

fn main() {
    let lower = IntervalCharClass::new('a', 'z');
    let special = ListCharClass::new(&['!', '$', '%', '@', '#']);
    let mut accu = Accumulator::new();
    accu.do_take_in(lower.get_char_iter());
    accu.do_take_in(special.get_ref_iter());

    println!("{}-{}-{}", lower.get_char(), lower.get_char(), lower.get_char());
    println!("{}-{}-{}", special.get_char(), special.get_char(), special.get_char());
}
