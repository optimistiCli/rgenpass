use rand::seq::SliceRandom;

use char_class::accu::Accumulator;
use char_class::interval::IntervalCharClass;
use char_class::array::ArrayCharClass;
use char_class::CharClass;

mod char_class;

pub struct GenPass {
    char_classes: Vec<Box<dyn CharClass>>
}

pub type GenPassResult = Result<String, String>;

impl GenPass {
    pub fn new() -> GenPass {
        GenPass {
            char_classes: Vec::new(),
        }
    }

    pub fn add_interval(
            &mut self,
            a_from: char,
            a_to: char,
            ) {
        self.char_classes
            .push(Box::new(
                IntervalCharClass::new(
                        a_from, 
                        a_to,
                        )));
    }

    pub fn add_list(
            &mut self,
            a_str: &str,
            ) {
        self.char_classes
            .push(Box::new(
                ArrayCharClass::from_exact_size_iter(
                    a_str
                        .chars()
                        .collect::<Vec<char>>()
                        .iter()
                        )));
    }

    fn cook_combined(&self) -> impl CharClass {
        let mut accumulator = Accumulator::new();
        self.char_classes
            .iter()
            .for_each(|char_class|
                accumulator
                    .collect_from(char_class)
                    );
        accumulator.cook_char_class()
    }

    pub fn generate(
            &self,
            a_num: usize,
            ) -> GenPassResult {
        Ok(String::from_iter(self.cook_combined().take(a_num)))
    }

    pub fn generate_all(
            &mut self,
            a_num: usize,
            ) -> GenPassResult {
        let num_classes = self.char_classes.len();
        if a_num < num_classes {
            return Err(format!(
                    "Password too short to accomodate all {} character classes", 
                    self.char_classes.len()
                    ));
        }
        let mut buffer: Vec<char> = Vec::with_capacity(a_num);
        self.char_classes
            .iter_mut()
            .map(|char_class| 
                char_class.next().expect("Random ran out of numbers"))
            .for_each(|c| buffer.push(c));
        let extra_chars = a_num - num_classes;
        if extra_chars > 0 {
            self.cook_combined()
                .take(extra_chars)
                .for_each(|c| 
                    buffer.push(c))
        }
        let mut rng = rand::thread_rng();
        buffer.shuffle(&mut rng);
        Ok(String::from_iter(buffer))
    }
}