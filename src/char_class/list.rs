use rand::seq::SliceRandom;
use super::CharClass;

pub struct ListCharClass<'a> {
    list: &'a [char],
}

impl<'a> super::RefIterable<'a> for ListCharClass<'a> {
    fn get_ref_iter(&self) -> impl IntoIterator<Item = &'a char> {
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

// impl<'a> IntoIterator for ListCharClass<'a> {
//     type Item = <&'a [char] as IntoIterator>::Item;
//     type IntoIter = <&'a [char] as IntoIterator>::IntoIter;

//     fn into_iter(self) -> Self::IntoIter {
//         self.list.into_iter()
//     }
// }
