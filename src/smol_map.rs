// use core::iter::FromIterator;

// use arrayvec::ArrayVec;

// struct SmolMap<'a, const ArraySize: usize> {
//     arr: ArrayVec<[(&'a str, &'a str); ArraySize]>,
// }

// impl<'a, const ArraySize: usize> FromIterator<(&'a str, &'a str)> for SmolMap<'a, ArraySize> {
//     fn from_iter<I: IntoIterator<Item = (&'a str, &'a str)>>(iter: I) -> SmolMap<'a, ArraySize> {
//         SmolMap {
//             arr: iter.into_iter().collect(),
//         }
//     }
// }
// impl<'a, const ArraySize: usize> SmolMap<'a, ArraySize> {
//     fn get(&self, i: &str) -> Option<&&str> {
//         self.arr
//             .iter()
//             .find_map(|(k, v)| if *k == i { Some(v) } else { None })
//     }
// }
