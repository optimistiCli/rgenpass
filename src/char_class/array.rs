use rand::seq::SliceRandom;
use super::CharClass;

fn cook_boxed_array<T: Clone>(
        a_iter: impl Iterator<Item = T>,
        a_len: usize,
        ) -> Box<[T]> {
    let mut boxed_array = unsafe {
        let layout = std::alloc::Layout::array::<T>(a_len).expect("Can't allocate memory");
        let ptr = std::alloc::alloc_zeroed(layout) as *mut usize;
        let slice = std::ptr::slice_from_raw_parts_mut(ptr, a_len) as *mut [T];
        Box::from_raw(slice)
    };

    for (i, item) in a_iter.take(a_len).enumerate() {
        boxed_array[i] = item;
    }

    boxed_array
}

pub struct ArrayCharClass {
    array: Box<[char]>,
}

impl ArrayCharClass {
    pub fn from_exact_size_iter<'a>(a_iter: impl ExactSizeIterator<Item = &'a char>) -> ArrayCharClass {
        let len = a_iter.len();
        ArrayCharClass {
            array: cook_boxed_array(a_iter.cloned(), len),
        }
    }

    pub fn from_str(a_str: &str) -> ArrayCharClass {
        Self::from_exact_size_iter(a_str.chars().collect::<Vec<char>>().iter())
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
    fn chars(&self) -> impl Iterator<Item = char> {
        self.array.iter().cloned()
    }
}
