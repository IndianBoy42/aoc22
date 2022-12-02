// pub use lazy_static;

pub use arrayvec::ArrayVec;
pub use bit_set::BitSet;
pub use boolinator::Boolinator;
pub use itertools::Itertools;
pub use itertools::{iproduct, izip};
pub use rayon::prelude::*;
pub use std::cmp::Reverse;
pub use std::collections::HashSet;
pub use std::collections::{BinaryHeap, HashMap};
pub use std::convert::TryInto;
pub use std::fs::File;
pub use std::hash::BuildHasherDefault;
pub use std::io;
pub use std::io::Read;
pub use std::iter::successors;
pub use std::iter::FromIterator;
pub use std::path::Path;
pub use std::str::FromStr;

use fxhash::FxHashMap;
use fxhash::FxHashSet;
pub type FSet<T> = FxHashSet<T>;
pub type FMap<K, V> = FxHashMap<K, V>;
// pub use fnv::FnvHasher;
// type FSet<T> = HashSet<T, BuildHasherDefault<FnvHasher>>;
// type FMap<K, V> = HashMap<K, V, BuildHasherDefault<FnvHasher>>;

pub fn fmap<K, V>(cap: usize) -> FMap<K, V> {
    FMap::with_capacity_and_hasher(cap, std::hash::BuildHasherDefault::default())
}

pub fn fset<V>(cap: usize) -> FSet<V> {
    FSet::with_capacity_and_hasher(cap, std::hash::BuildHasherDefault::default())
}

pub fn read_input<P: AsRef<Path>>(p: P) -> io::Result<String> {
    let mut out = String::with_capacity(100_000);
    File::open(p)?.read_to_string(&mut out)?;
    Ok(out)
}

pub fn pause() {
    io::stdin().read_line(&mut String::new()).unwrap();
}

// macro_rules! blackhole {
// ($($l:expr),+) => {};
// }

type MinHeap<T> = BinaryHeap<Reverse<T>>;

pub fn firstlast<T>(vec: &[T]) -> Option<(&T, &T)> {
    vec.split_first()
        .map(|(first, rest)| rest.last().map_or((first, first), |last| (first, last)))
}

pub fn firstlastex<T>(vec: &[T]) -> (Option<&T>, Option<&T>) {
    if let Some((first, rest)) = vec.split_first() {
        (Some(first), rest.last())
    } else {
        (None, None)
    }
}

pub fn minmax<T: Copy + PartialOrd>(it: impl IntoIterator<Item = T>) -> Option<(T, T)> {
    match it.into_iter().minmax() {
        itertools::MinMaxResult::NoElements => None,
        itertools::MinMaxResult::OneElement(a) => Some((a, a)),
        itertools::MinMaxResult::MinMax(a, b) => Some((a, b)),
    }
}

pub fn minmax_clone<T: Clone + PartialOrd>(it: impl IntoIterator<Item = T>) -> Option<(T, T)> {
    match it.into_iter().minmax() {
        itertools::MinMaxResult::NoElements => None,
        itertools::MinMaxResult::OneElement(a) => Some((a.clone(), a)),
        itertools::MinMaxResult::MinMax(a, b) => Some((a, b)),
    }
}

pub fn mov<T>(x: T) -> T {
    x
}
