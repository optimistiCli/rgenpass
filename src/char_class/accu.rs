use std::{collections::HashSet, any::TypeId};

use super::GenAccumulator;

pub struct Accumulator {
    set: HashSet<char>,
}

impl Accumulator {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }

	pub fn do_take_in<T>(&mut self, a_iterable: impl IntoIterator<Item = T>) where Self: super::GenAccumulator<T> {
		self.take_in(a_iterable)
	}
}

impl super::GenAccumulator<char> for Accumulator {
    fn take_in(&mut self, a_iterable: impl IntoIterator<Item = char>) {
        self.set.extend(a_iterable);
    }
}

impl<'a> super::GenAccumulator<&'a char> for Accumulator {
    fn take_in(&mut self, a_iterable: impl IntoIterator<Item = &'a char>) {
        self.set.extend(a_iterable);
    }
}
