use std::collections::HashSet;
use super::{Collector, Emitter};

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

    pub fn collect<T>(&mut self, a_emitter: &impl Emitter<T>) where Self: Collector<T> {
        self.collect_worker(a_emitter);
    }

    // fn oo1(&self) -> Box<[char]> {
    //     let oo2 = Vec::from_iter(self.set.iter().cloned());
    //     let oo3: &[char] = &oo2;
    //     Box::new(oo3)
    // }
}

impl Collector<char> for Accumulator {
    fn collect_worker(&mut self, a_emitter: &impl Emitter<char>) {
        self.set.extend(a_emitter.emit());
    }
}
impl<'a> Collector<&'a char> for Accumulator {
    fn collect_worker(&mut self, a_emitter: &impl Emitter<&'a char>) {
        self.set.extend(a_emitter.emit());
    }
}
