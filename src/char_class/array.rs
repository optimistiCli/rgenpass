use rand::seq::SliceRandom;
use iwrtb::BoxedArray;
use super::CharClass;

pub struct ArrayCharClass {
    array: Box<[char]>,
}

impl ArrayCharClass {
    pub fn from_exact_size_iter<'a>(a_iter: impl ExactSizeIterator<Item = &'a char>) -> ArrayCharClass {
        let len = a_iter.len();
        ArrayCharClass {
            array: Box::array_from_iter_and_len(a_iter.cloned(), len),
        }
    }
}

impl Iterator for ArrayCharClass {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();
        Some(*self.array.choose(&mut rng).expect("Can't pick a char"))
    }
}

impl CharClass for ArrayCharClass {
    fn chars(&self) -> Box<dyn Iterator<Item = char> + '_> {
        Box::new(self.array.iter().cloned())
    }
}
