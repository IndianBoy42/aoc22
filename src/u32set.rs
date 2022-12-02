use core::iter::FromIterator;
use std::{fmt::Debug, iter::successors};

use arrayvec::IntoIter;
use itertools::Itertools;
use num::PrimInt;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct USet {
    val: u32,
}
impl Debug for USet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.val)
    }
}
pub type U32Set = USet;
impl USet {
    pub const fn new(val: u32) -> USet {
        USet { val }
    }
    pub fn intersect_with(&mut self, other: USet) {
        self.val &= other.val;
    }
    pub const fn intersect(self, other: USet) -> USet {
        USet {
            val: self.val & other.val,
        }
    }
    pub fn union_with(&mut self, other: USet) {
        self.val |= other.val;
    }
    pub const fn union(self, other: USet) -> USet {
        USet {
            val: self.val | other.val,
        }
    }
    pub const fn len(self) -> usize {
        self.val.count_ones() as _
    }
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }
    /// Reverse the first `i` bits
    pub const fn revn(self, i: usize) -> USet {
        Self::new(self.val.reverse_bits() >> (32 - i))
    }
    pub const fn rev(self) -> USet {
        Self::new(self.val.reverse_bits())
    }

    pub fn bits(self) -> impl Iterator<Item = bool> + Clone {
        (0..32).map(move |i| self.val & (1 << i) != 0)
    }
    pub fn ones(self) -> impl Iterator<Item = usize> {
        self.bits().enumerate().filter(|&(_, x)| x).map(|(i, _)| i)
        // let i = self.bits().enumerate().filter(|&(_, x)| x).map(|(i, _)| i);
        // let j = successors(Some((self.val, u32::MAX)), |&(val, b)| {
        //     let tz = val.trailing_zeros() + 1;
        //     (val != 0).then(|| {
        //         let val = val >> tz;
        //         (val, b.wrapping_add(tz))
        //     })
        // })
        // .dropping(1)
        // .map(|(_, b)| b as _);
        // assert_eq!(i.clone().collect_vec(), j.clone().collect_vec());
        // j
    }
}

impl FromIterator<usize> for USet {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        USet {
            val: iter.into_iter().fold(0, |acc, bit| acc | (1u32 << bit)),
        }
    }
}
impl FromIterator<u8> for USet {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        USet {
            val: iter.into_iter().fold(0, |acc, bit| acc | (1u32 << bit)),
        }
    }
}
impl FromIterator<u16> for USet {
    fn from_iter<I: IntoIterator<Item = u16>>(iter: I) -> Self {
        USet {
            val: iter.into_iter().fold(0, |acc, bit| acc | (1u32 << bit)),
        }
    }
}
impl FromIterator<u32> for USet {
    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
        USet {
            val: iter.into_iter().fold(0, |acc, bit| acc | (1u32 << bit)),
        }
    }
}
impl FromIterator<u64> for USet {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        USet {
            val: iter.into_iter().fold(0, |acc, bit| acc | (1u32 << bit)),
        }
    }
}

impl FromIterator<bool> for USet {
    fn from_iter<I: IntoIterator<Item = bool>>(iter: I) -> Self {
        USet {
            val: iter
                .into_iter()
                .fold(0, |acc, bit| (acc << 1) | (bit as u32)),
        }
    }
}
