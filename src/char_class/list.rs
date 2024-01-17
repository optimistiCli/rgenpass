use rand::seq::SliceRandom;
use super::{CharClass, Emitter};

pub struct ListCharClass<'a> {
    list: &'a [char],
}

impl<'a> Emitter<&'a char> for ListCharClass<'a> {
    fn emit(&self) -> impl IntoIterator<Item = &'a char> {
        self.list.into_iter()
    }
}

impl ListCharClass<'_> {
    pub fn new<'a> (a_chars: &'a [char]) -> ListCharClass<'a> {
        ListCharClass {
            list: a_chars,
        }
    }
}

impl CharClass for ListCharClass<'_> {
    fn get_char(&self) -> char {
        let mut rng = rand::thread_rng();
        *self.list.choose(&mut rng).expect("The list is probably empty")
    }

    fn get_iter(&self) -> impl IntoIterator {
        self.list.into_iter()
    }
}