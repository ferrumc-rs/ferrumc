#![feature(thread_id_value)]

use rusty_pool::JoinHandle;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

/// A thread pool for managing and executing tasks concurrently.
pub struct ThreadPool {
    pool: Arc<rusty_pool::ThreadPool>,
    starting_thread: AtomicU64,
}

/// A batch of tasks to be executed in the thread pool.
pub struct ThreadPoolBatch<'a, R: Send + 'static> {
    pool: &'a Arc<rusty_pool::ThreadPool>,
    handles: Vec<JoinHandle<Box<R>>>,
    completed: bool,
    starting_thread: u64,
}

impl Default for ThreadPool {
    /// Creates a new `ThreadPool` with default settings.
    fn default() -> Self {
        Self::new()
    }
}

impl ThreadPool {
    /// Creates a new `ThreadPool`.
    pub fn new() -> Self {
        let pool = Arc::new(rusty_pool::ThreadPool::default());
        let starting_thread = std::thread::current().id().as_u64().get().into();
        Self {
            pool,
            starting_thread,
        }
    }

    /// Creates a new batch of tasks to be executed in the thread pool.
    ///
    /// # Returns
    /// A `ThreadPoolBatch` instance.
    pub fn batch<R>(&self) -> ThreadPoolBatch<'_, R>
    where
        R: Send + 'static,
    {
        ThreadPoolBatch {
            pool: &self.pool,
            handles: vec![],
            completed: false,
            starting_thread: self.starting_thread.load(Relaxed),
        }
    }
}

impl<'a, R: Send + 'static> ThreadPoolBatch<'a, R> {
    /// Executes a task in the thread pool batch.
    ///
    /// # Arguments
    /// * `f` - A function to be executed.
    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() -> R + Send + 'static,
    {
        if self.starting_thread != std::thread::current().id().as_u64().get() {
            panic!("Thread pool has been moved to a different thread");
        }
        let boxed = move || Box::new(f());
        let handle = self.pool.evaluate(boxed);
        self.handles.push(handle);
    }

    /// Waits for all tasks in the batch to complete and returns their results.
    ///
    /// # Returns
    /// A vector of results from the completed tasks.
    ///
    /// # Panics
    /// Will panic if the batch has already been completed.
    pub fn wait(mut self) -> Vec<R> {
        if self.completed {
            panic!("Batch already completed");
        }

        let mut results = vec![];
        for handle in self.handles {
            let result = handle.await_complete();
            results.push(*result);
        }
        self.completed = true;
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_thread_pool() {
        let pool = ThreadPool::new();
        let mut batch = pool.batch();

        for i in 0..10 {
            batch.execute(move || {
                sleep(Duration::from_millis(100));
                i * 2
            });
        }

        let results = batch.wait();
        assert_eq!(results.len(), 10);
        for i in 0..10 {
            assert_eq!(*results.get(i).unwrap(), i * 2);
        }
    }

    #[test]
    fn test_drops() {
        let pool = ThreadPool::new();
        let mut batch = pool.batch();

        for i in 0..10 {
            batch.execute(move || {
                sleep(Duration::from_millis(100));
                i * 2
            });
        }

        let results = batch.wait();
        assert_eq!(results.len(), 10);
        for i in 0..10 {
            assert_eq!(*results.get(i).unwrap(), i * 2);
        }
        let mut new_batch = pool.batch();
        for i in 0..10 {
            new_batch.execute(move || {
                sleep(Duration::from_millis(100));
                i * 3
            });
        }
        let results = new_batch.wait();
        for i in 0..10 {
            assert_eq!(*results.get(i).unwrap(), i * 3);
        }
    }

    #[test]
    fn test_empty_batch() {
        let pool = ThreadPool::new();
        let batch: ThreadPoolBatch<()> = pool.batch();
        let results = batch.wait();
        assert!(results.is_empty());
    }
}
