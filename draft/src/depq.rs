use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Debug;

/// <https://twitter.com/0x3b800001/status/1575295046399979520>
#[derive(Clone, Debug)]
struct MinMaxHeap<T: Ord> {
    min_heap: BinaryHeap<Reverse<T>>,
    max_heap: BinaryHeap<T>,
    mid: Option<T>,
}

impl<T: Ord> MinMaxHeap<T> {
    pub fn new() -> Self {
        Self {
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
            mid: None,
        }
    }

    pub fn push(&mut self, x: T) {
        match &self.mid {
            None => self.mid = Some(x),
            Some(mid) if mid <= &x => self.max_heap.push(x),
            Some(_) => self.min_heap.push(Reverse(x)),
        }
    }

    pub fn pop_min(&mut self) -> Option<T> {
        if let Some(Reverse(x)) = self.min_heap.pop() {
            Some(x)
        } else if let Some(x) = self.mid.take() {
            self.rotate();
            Some(x)
        } else {
            None
        }
    }

    pub fn pop_max(&mut self) -> Option<T> {
        if let Some(x) = self.max_heap.pop() {
            Some(x)
        } else if let Some(x) = self.mid.take() {
            self.rotate();
            Some(x)
        } else {
            None
        }
    }

    fn rotate(&mut self) {
        assert!(self.mid.is_none());
        assert!(self.min_heap.is_empty() || self.max_heap.is_empty());

        let (min, max) = if !self.max_heap.is_empty() {
            let mut min = std::mem::take(&mut self.max_heap).into_sorted_vec();
            let i = min.len() / 2;
            let max = min.split_off(i + 1);
            self.mid = min.pop();
            (min, max)
        } else if !self.min_heap.is_empty() {
            let tmp = std::mem::take(&mut self.min_heap).into_sorted_vec();
            let mut max: Vec<_> = tmp.into_iter().map(|Reverse(x)| x).collect();
            let i = max.len() / 2;
            let min = max.split_off(i + 1);
            self.mid = max.pop();
            (min, max)
        } else {
            return;
        };

        self.min_heap.extend(min.into_iter().map(|x| Reverse(x)));
        self.max_heap.extend(max);
    }
}

impl<T: Ord> Extend<T> for MinMaxHeap<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        if let Some(elt) = iter.next() {
            self.push(elt);
        }
        for elt in iter {
            if self.mid.as_ref().unwrap() <= &elt {
                self.max_heap.push(elt);
            } else {
                self.min_heap.push(Reverse(elt));
            }
        }
    }
}
