use std::collections::HashSet;

use char_class::CharClass;
use char_class::interval::IntervalCharClass;
use char_class::list::ListCharClass;
use char_class::accu::Accumulator;

pub mod char_class;

fn cook_boxed_array<T: Clone>(
        mut a_iter: impl Iterator<Item = T>,
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

fn main() {
    let mut some_hashset = HashSet::new();
    some_hashset.insert('q');
    some_hashset.insert('w');
    some_hashset.insert('e');
    some_hashset.insert('r');
    some_hashset.insert('t');
    some_hashset.insert('y');
    println!("{:?}", some_hashset);
    println!("{:?}", some_hashset.len());

    // let mut oo4 = oo2!(oo3.iter(); oo3.len());

    let mut oo4 = cook_boxed_array(
            some_hashset.iter().cloned(), 
            some_hashset.len(),
            );
    println!("{}", oo4.len());
    println!("{:?}", oo4);

    // let lower = IntervalCharClass::new('a', 'z');
    // let special = ListCharClass::new(&['!', '$', '%', '@', '#']);
    // let mut accu = Accumulator::new();
    // accu.collect(&lower);
    // accu.collect(&special);

    // println!("{}-{}-{}", lower.get_char(), lower.get_char(), lower.get_char());
    // println!("{}-{}-{}", special.get_char(), special.get_char(), special.get_char());
    // println!("{}", accu.to_string())
}
