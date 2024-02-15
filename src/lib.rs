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
        self.char_classes.push(Box::new(IntervalCharClass::new(a_from, a_to)));
    }

    pub fn add_list(
            &mut self,
            a_str: &str,
            ) {
        self.char_classes.push(Box::new(ArrayCharClass::from_exact_size_iter(a_str.chars().collect::<Vec<char>>().iter())));
    }

    pub fn generate(
            &self,
            a_num: usize,
            ) -> GenPassResult {
        let mut accumulator = Accumulator::new();
        self.char_classes
            .iter()
            .for_each(|char_class| accumulator.collect_from(char_class));
        let combined = accumulator.cook_char_class();
        Ok(String::from_iter(combined.take(a_num)))
    }

    // pub fn generate_all(
    //         &self,
    //         a_num: usize,
    //         ) -> GenPassResult {
    //     todo!()
    // }
}