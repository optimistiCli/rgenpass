use rand::distributions::{Uniform, Distribution};
use super::CharClass;

pub struct IntervalCharClass {
    distr: Uniform<char>,
    from: char,
    to: char,
}

impl IntervalCharClass {
    pub fn new(a_from: char, a_to: char) -> Self{
        if a_from < a_to {
            Self {
                distr: Uniform::new_inclusive(a_from, a_to),
                from: a_from.clone(),
                to: a_to.clone(),
            }
        } else {
            Self {
                distr: Uniform::new_inclusive(a_to, a_from),
                from: a_to.clone(),
                to: a_from.clone(),
            }
        }
    }
}

impl Iterator for IntervalCharClass {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();
        Some(self.distr.sample(&mut rng))
    }
}

impl CharClass for IntervalCharClass {
    fn chars(&self) -> Box<dyn Iterator<Item = char>> {
        Box::new((self.from..=self.to).into_iter())
    }
}
