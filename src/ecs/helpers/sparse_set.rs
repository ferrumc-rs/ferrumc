/// A sparse set data structure optimized for efficient insertion, removal, and lookup operations.
///
/// This implementation uses a sparse vector of optional indices paired with a dense vector of values,
/// allowing for O(1) average time complexity for most operations while remaining memory efficient.
#[derive(Debug, Default)]
pub struct SparseSet<T> {
    sparse: Vec<Option<usize>>,
    dense: Vec<(usize, T)>,
}

impl<T> SparseSet<T> {}

impl<T> SparseSet<T> {
    /// Creates a new, empty `SparseSet`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crateecs::dsa::sparse_set::SparseSet;
    /// let set: SparseSet<i32> = SparseSet::new();
    /// ```
    pub fn new() -> Self {
        SparseSet {
            sparse: Vec::new(),
            dense: Vec::new(),
        }
    }

    /// Inserts a value into the set at the specified index.
    ///
    /// If the index already contains a value, it will be overwritten.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crateecs::dsa::sparse_set::SparseSet;
    /// let mut set = SparseSet::new();
    /// set.insert(5, "value");
    /// assert_eq!(set.get(5), Some(&"value"));
    /// ```
    pub fn insert(&mut self, index: usize, value: T) {
        if index >= self.sparse.len() {
            self.grow(index);
        }

        match self.sparse[index] {
            Some(dense_index) => self.dense[dense_index] = (index, value),
            None => {
                let dense_index = self.dense.len();
                self.sparse[index] = Some(dense_index);
                self.dense.push((index, value));
            }
        }
    }

    /// Removes and returns the value at the specified index, if it exists.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crateecs::dsa::sparse_set::SparseSet;
    /// let mut set = SparseSet::new();
    /// set.insert(5, "value");
    /// assert_eq!(set.remove(5), Some("value"));
    /// assert_eq!(set.remove(5), None);
    /// ```
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.sparse.len() {
            return None;
        }

        self.sparse[index].take().map(|dense_index| {
            let (_, value) = self.dense.swap_remove(dense_index);
            if dense_index < self.dense.len() {
                let (swapped_index, _) = self.dense[dense_index];
                self.sparse[swapped_index] = Some(dense_index);
            }
            value
        })
    }

    /// Returns a reference to the value at the specified index, if it exists.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crateecs::dsa::sparse_set::SparseSet;
    /// let mut set = SparseSet::new();
    /// set.insert(5, "value");
    /// assert_eq!(set.get(5), Some(&"value"));
    /// assert_eq!(set.get(6), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        self.sparse
            .get(index)
            .and_then(|&dense_index| dense_index.map(|di| &self.dense[di].1))
    }

    /// Returns a mutable reference to the value at the specified index, if it exists.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crateecs::dsa::sparse_set::SparseSet;
    /// let mut set = SparseSet::new();
    /// set.insert(5, String::from("value"));
    /// if let Some(value) = set.get_mut(5) {
    ///     value.push_str(" modified");
    /// }
    /// assert_eq!(set.get(5), Some(&String::from("value modified")));
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.sparse
            .get(index)
            .and_then(|&dense_index| dense_index.map(|di| &mut self.dense[di].1))
    }

    /// Returns an iterator over the values in the set.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crateecs::dsa::sparse_set::SparseSet;
    /// let mut set = SparseSet::new();
    /// set.insert(5, 1);
    /// set.insert(10, 2);
    /// let sum: i32 = set.iter().sum();
    /// assert_eq!(sum, 3);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (&usize, &T)> {
        self.dense.iter().map(|(key, value)| (key, value))
    }

    /// Returns a mutable iterator over the values in the set.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crateecs::dsa::sparse_set::SparseSet;
    /// let mut set = SparseSet::new();
    /// set.insert(5, 1);
    /// set.insert(10, 2);
    /// for value in set.iter_mut() {
    ///     *value *= 2;
    /// }
    /// assert_eq!(set.get(5), Some(&2));
    /// assert_eq!(set.get(10), Some(&4));
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut usize, &mut T)> {
        // self.dense.iter_mut().map(|(key, value)| value)
        // somehow only get the value as mutable, the key should remain immutable
        self.dense.iter_mut().map(|(key, value)| (key, value))
    }

    /// Removes all elements from the set.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crateecs::dsa::sparse_set::SparseSet;
    /// let mut set = SparseSet::new();
    /// set.insert(5, 1);
    /// set.insert(10, 2);
    /// set.clear();
    /// assert!(set.iter().next().is_none());
    /// ```
    pub fn clear(&mut self) {
        self.dense.clear();
        self.sparse.fill(None);
    }

    // Private helper method to grow the sparse vector
    fn grow(&mut self, index: usize) {
        let new_len = (index + 1).next_power_of_two().max(64);
        self.sparse.reserve(new_len);
        self.sparse.resize_with(new_len, || None);
        self.dense.reserve(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut set = SparseSet::new();
        set.insert(0, "zero");
        set.insert(1, "one");
        set.insert(100, "hundred");

        assert_eq!(set.get(0), Some(&"zero"));
        assert_eq!(set.get(1), Some(&"one"));
        assert_eq!(set.get(100), Some(&"hundred"));
        assert_eq!(set.get(50), None);
    }

    #[test]
    fn test_insert_overwrite() {
        let mut set = SparseSet::new();
        set.insert(1, "one");
        set.insert(1, "new one");

        assert_eq!(set.get(1), Some(&"new one"));
    }

    #[test]
    fn test_remove() {
        let mut set = SparseSet::new();
        set.insert(0, "zero");
        set.insert(1, "one");
        set.insert(2, "two");

        assert_eq!(set.remove(1), Some("one"));
        assert_eq!(set.get(1), None);
        assert_eq!(set.remove(1), None);
        assert_eq!(set.get(0), Some(&"zero"));
        assert_eq!(set.get(2), Some(&"two"));
    }

    #[test]
    fn test_large_index() {
        let mut set = SparseSet::new();
        set.insert(1_000_000, "large index");

        assert_eq!(set.get(1_000_000), Some(&"large index"));
        assert_eq!(set.remove(1_000_000), Some("large index"));
        assert_eq!(set.get(1_000_000), None);
    }

    #[test]
    fn test_iter() {
        let mut set = SparseSet::new();
        set.insert(0, 0);
        set.insert(10, 10);
        set.insert(20, 20);

        let mut iter = set.iter().map(|(_, value)| value);
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut set = SparseSet::new();
        set.insert(0, 0);
        set.insert(10, 10);
        set.insert(20, 20);

        for (_, value) in set.iter_mut() {
            *value += 1;
        }

        assert_eq!(set.get(0), Some(&1));
        assert_eq!(set.get(10), Some(&11));
        assert_eq!(set.get(20), Some(&21));
    }

    #[test]
    fn test_clear() {
        let mut set = SparseSet::new();
        set.insert(0, "zero");
        set.insert(1, "one");
        set.insert(100, "hundred");

        set.clear();

        assert_eq!(set.get(0), None);
        assert_eq!(set.get(1), None);
        assert_eq!(set.get(100), None);
        assert!(set.iter().next().is_none());
    }

    #[test]
    fn test_remove_and_reinsert() {
        let mut set = SparseSet::new();
        set.insert(0, "zero");
        set.insert(1, "one");

        set.remove(0);
        set.insert(0, "new zero");

        assert_eq!(set.get(0), Some(&"new zero"));
        assert_eq!(set.get(1), Some(&"one"));
    }

    #[test]
    fn test_sparse_behavior() {
        let mut set = SparseSet::new();
        set.insert(0, 0);
        set.insert(1000, 1000);
        set.insert(1_000_000, 1_000_000);

        assert_eq!(set.get(0), Some(&0));
        assert_eq!(set.get(1000), Some(&1000));
        assert_eq!(set.get(1_000_000), Some(&1_000_000));

        for i in 1..1000 {
            assert_eq!(set.get(i), None);
        }
        for i in 1001..1_000_000 {
            assert_eq!(set.get(i), None);
        }
    }
}
