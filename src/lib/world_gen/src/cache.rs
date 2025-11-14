use std::{
    array::from_fn,
    ops::{Index, IndexMut},
};

pub struct Cache<T, const N: usize, const STEP: usize> {
    cache: [[T; N]; N],
}

impl<T, const N: usize, const STEP: usize> Cache<T, N, STEP> {
    pub fn new<F: FnMut((usize, usize)) -> T>(mut f: F) -> Self {
        Self {
            cache: from_fn(|x| from_fn(|z| f((x * STEP, z * STEP)))),
        }
    }
}

impl<T, const N: usize, const STEP: usize> Index<(usize, usize)> for Cache<T, N, STEP> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.cache[index.0 / STEP][index.1 / STEP]
    }
}

impl<T, const N: usize, const STEP: usize> IndexMut<(usize, usize)> for Cache<T, N, STEP> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.cache[index.0 / STEP][index.1 / STEP]
    }
}

impl<T: Copy, const N: usize, const STEP: usize> Cache<T, N, STEP> {
    pub fn fill(item: T) -> Self {
        Self {
            cache: [[item; N]; N],
        }
    }
}
