use rand::distributions::{Uniform, Distribution};
use super::{CharClass, Emitter};

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

impl CharClass for IntervalCharClass {
    fn get_char(&self) -> char {
        let mut rng = rand::thread_rng();
        self.distr.sample(&mut rng)
    }

    fn get_iter(&self) -> impl IntoIterator {
        (self.from..=self.to).into_iter()
    }
}

impl Emitter<char> for IntervalCharClass {
    fn emit(&self) -> impl IntoIterator<Item = char> {
        (self.from..=self.to).into_iter()
    }
}
