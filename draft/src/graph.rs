use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Add;

pub trait Graph<'a, T: 'a, V: 'a> {
    type Output: Iterator<Item = (V, T)>;
    fn delta(&'a self, vertex: &V) -> Self::Output;
}

pub trait IndexedGraph<V> {
    fn len(&self) -> usize;
    fn index(&self, vertex: &V) -> usize;
}

pub struct AdjacencyList<T>(Vec<Vec<(usize, T)>>);

impl<T> AdjacencyList<T> {
    pub fn from_edges(n: usize, edges: Vec<(usize, usize, T)>) -> Self {
        let mut g: Vec<_> = (0..n).map(|_| vec![]).collect();
        for (u, v, w) in edges {
            g[u].push((v, w));
        }
        Self(g)
    }
}

impl<'a, T: 'a + Clone> Graph<'a, T, usize> for AdjacencyList<T> {
    type Output = std::iter::Cloned<std::slice::Iter<'a, (usize, T)>>;
    fn delta(&'a self, &vertex: &usize) -> Self::Output {
        self.0[vertex].iter().cloned()
    }
}

impl<T> IndexedGraph<usize> for AdjacencyList<T> {
    fn len(&self) -> usize { self.0.len() }
    fn index(&self, &vertex: &usize) -> usize { vertex }
}

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_zero_int {
    ( $($ty:ty)* ) => { $(
        impl Zero for $ty {
            fn zero() -> Self { 0 }
        }
    )* }
}
impl_zero_int! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

// single-source shortest path
pub struct SsspStruct<'a, T, V, G> {
    src: V,
    distance: Vec<Option<T>>,
    prev: Vec<Option<V>>,
    graph: &'a G,
}

impl<'a, T: 'a, V: 'a, G: 'a> SsspStruct<'a, T, V, G>
where
    G: Graph<'a, T, V> + IndexedGraph<V>,
{
    pub fn cost(&'a self, dst: &V) -> Option<&T> {
        let i = self.graph.index(dst);
        self.distance[i].as_ref()
    }

    pub fn path<'b>(&'a self, dst: &'b V) -> Option<Vec<&'b V>>
    where
        'a: 'b,
    {
        let mut i = self.graph.index(&dst);
        if self.prev[i].is_none() {
            return (self.graph.index(&self.src) == i).then(|| vec![dst]);
        }
        let mut res = vec![dst];
        while let Some(v) = self.prev[i].as_ref() {
            res.push(v);
            i = self.graph.index(&v);
        }
        res.reverse();
        Some(res)
    }
}

pub trait Sssp<'a, T, V> {
    type Output;
    fn sssp(&'a self, src: V) -> Self::Output;
}

impl<'a, T: 'a, V: 'a, G: 'a> Sssp<'a, T, V> for G
where
    G: Graph<'a, T, V> + IndexedGraph<V>,
    T: Clone + Ord + Zero,
    for<'b> &'b T: Add<&'b T, Output = T>,
    V: Clone + Ord,
{
    type Output = SsspStruct<'a, T, V, G>;
    fn sssp(&'a self, src: V) -> Self::Output {
        let n = self.len();
        let mut distance = vec![None; n];
        let mut prev = vec![None; n];
        distance[self.index(&src)] = Some(T::zero());

        let mut q = BinaryHeap::new();
        q.push((Reverse(T::zero()), src.clone()));
        while let Some((Reverse(w), v)) = q.pop() {
            let i = self.index(&v);
            if let Some(cur_w) = &distance[i] {
                if cur_w < &w {
                    continue;
                }
            }
            for (nv, dw) in self.delta(&v) {
                let nw = &w + &dw;
                let ni = self.index(&nv);
                match &distance[ni] {
                    Some(cur_nw) if cur_nw <= &nw => {}
                    _ => {
                        distance[ni] = Some(nw.clone());
                        q.push((Reverse(nw), nv));
                        prev[ni] = Some(v.clone());
                    }
                }
            }
        }

        SsspStruct { src, distance, prev, graph: self }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::*;

    #[test]
    fn sanity_check() {
        let es = vec![(0, 1, 10), (0, 2, 20), (1, 2, 15)];
        let g = AdjacencyList::from_edges(4, es);
        let sssp = g.sssp(0);

        assert_eq!(sssp.cost(&0), Some(&0));
        assert_eq!(sssp.cost(&1), Some(&10));
        assert_eq!(sssp.cost(&2), Some(&20));
        assert_eq!(sssp.cost(&3), None);

        assert_eq!(sssp.path(&0), Some(vec![&0]));
        assert_eq!(sssp.path(&1), Some(vec![&0, &1]));
        assert_eq!(sssp.path(&2), Some(vec![&0, &2]));
        assert_eq!(sssp.path(&3), None);
    }
}
