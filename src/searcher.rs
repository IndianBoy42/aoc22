use std::iter::once;
use std::marker::PhantomData;
use std::{
    cmp::Reverse, collections::BinaryHeap, collections::HashSet, collections::VecDeque,
    hash::BuildHasher, hash::Hash,
};

use crate::utils::{fset, BitSet, FSet};

pub trait SearchQueue<T> {
    fn newq() -> Self;
    fn newq_cap(cap: usize) -> Self;
    fn pushq(&mut self, e: T);
    fn popq(&mut self) -> Option<T>;
}

pub trait Visited<T> {
    fn newset() -> Self;
    fn newset_cap(cap: usize) -> Self;
    fn mark(&mut self, e: T) -> bool;
    fn check(&self, e: &T) -> bool;
    fn len(&self) -> usize;
}

impl<T> SearchQueue<T> for BinaryHeap<Reverse<T>>
where
    Reverse<T>: Ord,
{
    fn newq() -> Self {
        Self::new()
    }
    fn newq_cap(cap: usize) -> Self {
        Self::with_capacity(cap)
    }
    fn pushq(&mut self, e: T) {
        self.push(Reverse(e));
    }
    fn popq(&mut self) -> Option<T> {
        self.pop().map(|Reverse(e)| e)
    }
}

impl<T> SearchQueue<T> for VecDeque<T> {
    fn newq() -> Self {
        Self::new()
    }
    fn newq_cap(cap: usize) -> Self {
        Self::with_capacity(cap)
    }
    fn pushq(&mut self, e: T) {
        self.push_back(e);
    }
    fn popq(&mut self) -> Option<T> {
        self.pop_front()
    }
}

impl<T> SearchQueue<T> for Vec<T> {
    fn newq() -> Self {
        Self::new()
    }
    fn newq_cap(cap: usize) -> Self {
        Self::with_capacity(cap)
    }
    fn pushq(&mut self, e: T) {
        self.push(e);
    }
    fn popq(&mut self) -> Option<T> {
        self.pop()
    }
}

impl<T, B: Default> Visited<T> for HashSet<T, B>
where
    T: Eq + Hash,
    B: BuildHasher,
{
    fn newset() -> Self {
        Self::with_hasher(B::default())
    }
    fn newset_cap(cap: usize) -> Self {
        Self::with_capacity_and_hasher(cap, B::default())
    }
    fn mark(&mut self, e: T) -> bool {
        self.insert(e)
    }
    fn check(&self, e: &T) -> bool {
        !self.contains(e)
    }

    fn len(&self) -> usize {
        self.len()
    }
}

// impl<T, F> Visited<T> for F
// where
//     F: FnMut(T) -> bool,
//     T: Clone,
// {
//     fn newset() -> Self {
//         unreachable!()
//     }
//     fn newset_cap(cap: usize) -> Self {
//         unreachable!()
//     }
//     fn mark(&mut self, e: T) -> bool {
//         self(e)
//     }
//     fn check(&self, e: &T) -> bool {
//         self(e.clone())
//     }
// }

impl Visited<usize> for BitSet {
    fn newset() -> Self {
        Self::new()
    }
    fn newset_cap(cap: usize) -> Self {
        Self::with_capacity(cap)
    }
    fn mark(&mut self, e: usize) -> bool {
        self.insert(e)
    }
    fn check(&self, &e: &usize) -> bool {
        !self.contains(e)
    }

    fn len(&self) -> usize {
        self.capacity()
    }
}

#[derive(Clone, Debug)]
pub struct Searcher<T, Queue, VisitSet, NeighboursFn> {
    visited: VisitSet,
    queue: Queue,
    neighbours: NeighboursFn,
    _dummy_t: PhantomData<T>,
}
pub type DFSearcher<T, V, SF> = Searcher<T, Vec<T>, V, SF>;
pub type DFSearcherInt<SF> = Searcher<usize, Vec<usize>, BitSet, SF>;
pub type BFSearcher<T, V, SF> = Searcher<T, VecDeque<T>, V, SF>;
pub type BFSearcherInt<SF> = Searcher<usize, VecDeque<usize>, BitSet, SF>;
pub type DijSearcher<T, V, SF> = Searcher<T, BinaryHeap<Reverse<T>>, V, SF>;
pub type DijSearcherInt<SF> = Searcher<usize, BinaryHeap<Reverse<usize>>, BitSet, SF>;

impl<T, Queue, VisitSet, NeighboursFn /* , SearchIter */> Searcher<T, Queue, VisitSet, NeighboursFn>
where
    T: Hash + Clone + Eq,
    VisitSet: Visited<T>,
    Queue: SearchQueue<T>,
    // NeighboursFn: FnMut(&T) -> SearchIter,
    // SearchIter: IntoIterator<Item = T>,
{
    pub fn new(init: T, neighbours: NeighboursFn) -> Self {
        Self::with_capacity(1, init, neighbours)
    }
    pub fn new_all<I: IntoIterator<Item = T>>(init: I, neighbours: NeighboursFn) -> Self {
        let init = init.into_iter();
        let (l, u) = init.size_hint();
        let size = u.unwrap_or(l);

        Self::with_capacity_all(size, init, neighbours)
    }
    pub fn with_capacity_all<I: IntoIterator<Item = T>>(
        cap: usize,
        init: I,
        neighbours: NeighboursFn,
    ) -> Self {
        let mut slf = Searcher {
            visited: VisitSet::newset_cap(cap),
            queue: Queue::newq_cap(cap),
            neighbours,
            _dummy_t: PhantomData,
        };
        for e in init {
            slf.push(e);
        }
        slf
    }
    pub fn with_capacity(cap: usize, init: T, neighbours: NeighboursFn) -> Self {
        Self::with_capacity_all(cap, once(init), neighbours)
    }

    pub fn new_empty(neighbours: NeighboursFn) -> Self {
        Self::with_capacity_empty(1, neighbours)
    }
    pub fn new_all_empty(neighbours: NeighboursFn) -> Self {
        Self::with_capacity_all_empty(1, neighbours)
    }
    pub fn with_capacity_all_empty(cap: usize, neighbours: NeighboursFn) -> Self {
        Searcher {
            visited: VisitSet::newset_cap(cap),
            queue: Queue::newq_cap(cap),
            neighbours,
            _dummy_t: PhantomData,
        }
    }
    pub fn with_capacity_empty(cap: usize, neighbours: NeighboursFn) -> Self {
        Self::with_capacity_all_empty(cap, neighbours)
    }

    pub fn push(&mut self, e: T) -> bool {
        if self.visited.mark(e.clone()) {
            self.queue.pushq(e);
            true
        } else {
            false
        }
    }

    /// Just to get a better error message for the type of `NeighboursFn`
    pub fn check<SearchIter>(self) -> Self
    where
        NeighboursFn: FnMut(&T) -> SearchIter,
        SearchIter: IntoIterator<Item = T>,
    {
        self
    }
}

impl<T, VisitSet, Queue, NeighboursFn, SearchIter> Iterator
    for Searcher<T, Queue, VisitSet, NeighboursFn>
where
    T: Hash + Clone + Eq,
    VisitSet: Visited<T>,
    Queue: SearchQueue<T>,
    NeighboursFn: FnMut(&T) -> SearchIter,
    SearchIter: IntoIterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.popq().map(|elem| {
            // Mark visited
            // self.visited.mark(elem.clone());

            // Find Neighbours
            for e in (self.neighbours)(&elem) {
                self.push(e);
            }

            elem
        })
    }
}
