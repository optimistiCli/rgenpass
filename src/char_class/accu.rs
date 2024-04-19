use std::collections::HashSet;
use super::CharClass;
use super::array::ArrayCharClass;

pub struct Accumulator {
    set: HashSet<char>,
}

impl ToString for Accumulator {
    fn to_string(&self) -> String {
        String::from_iter(self.set.iter())
    }
}

impl Accumulator {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }

    pub fn collect_from(&mut self, a_char_class: &Box<dyn CharClass>) {
        self.set.extend(a_char_class.chars());
    }

    pub fn cook_char_class(&self) -> ArrayCharClass {
        ArrayCharClass::from_exact_size_iter(self.set.iter())
    }
}
