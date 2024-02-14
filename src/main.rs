use char_class::CharClass;
use char_class::interval::IntervalCharClass;
use char_class::array::ArrayCharClass;
use char_class::accu::Accumulator;

pub mod char_class;

fn main() {
    let mut lower = IntervalCharClass::new('a', 'z');
    println!("{}-{}-{}", lower.next().unwrap(), lower.next().unwrap(), lower.next().unwrap());
    println!("{}", String::from_iter(lower.chars()));
    println!("{}-{}-{}", lower.next().unwrap(), lower.next().unwrap(), lower.next().unwrap());
    println!("{}", String::from_iter(lower.chars()));

    let mut special = ArrayCharClass::from_str("!$%@#");
    println!("{}-{}-{}", special.next().unwrap(), special.next().unwrap(), special.next().unwrap());
    println!("{}", String::from_iter(special.chars()));
    println!("{}-{}-{}", special.next().unwrap(), special.next().unwrap(), special.next().unwrap());
    println!("{}", String::from_iter(special.chars()));

    let mut accumulator = Accumulator::new();
    accumulator.collect_from(&lower);
    accumulator.collect_from(&special);
    let mut combined = accumulator.cook_char_class();
    println!("{}-{}-{}", combined.next().unwrap(), combined.next().unwrap(), combined.next().unwrap());
    println!("{}", String::from_iter(combined.chars()));
    println!("{}-{}-{}", combined.next().unwrap(), combined.next().unwrap(), combined.next().unwrap());
    println!("{}", String::from_iter(combined.chars()));
}
