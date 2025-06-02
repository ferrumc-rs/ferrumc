use rusty_pool::JoinHandle;
use std::cmp::max;
use std::sync::Arc;
use std::time::Duration;

/// A thread pool for managing and executing tasks concurrently.
pub struct ThreadPool {
    pool: Arc<rusty_pool::ThreadPool>,
}

/// A batch of tasks to be executed in the thread pool.
// DO NOT IMPLEMENT `Clone` FOR THIS STRUCTURE, SEE THE `ThreadPoolBatch::execute_unchecked` FOR WHY
pub struct ThreadPoolBatch<'a, R: Send + 'static> {
    pool: &'a Arc<rusty_pool::ThreadPool>,
    handles: Vec<JoinHandle<Box<R>>>,
    completed: bool,
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
        // Use all but 3 cores for the thread pool. 1 core is for the main thread, 1 for the network thread and 1 for the control-c handler.
        let core_count = max(1, num_cpus::get() as i32 - 3) as usize;
        let pool = Arc::new(rusty_pool::ThreadPool::new_named(
            "ferrumc_threadpool".to_string(),
            core_count,
            core_count,
            Duration::from_secs(60),
        ));
        Self { pool }
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
        }
    }

    /// Executes a single task in the thread pool and returns its result.
    ///
    /// # Arguments
    /// * `func` - A function to be executed.
    ///
    /// # Returns
    /// The result of the executed function.
    pub fn oneshot<F, R>(&self, func: F) -> R
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        if std::thread::current()
            .name()
            .unwrap()
            .contains("ferrumc_threadpool")
        {
            panic!("Thread pool is trying to run a task on itself, this is not allowed");
        }
        let boxed = move || Box::new(func());
        let handle = self.pool.evaluate(boxed);
        let result = handle.await_complete();
        *result
    }
}

impl<'a, R: Send + 'static> ThreadPoolBatch<'a, R> {
    /// Executes a task in the thread pool batch.
    ///
    /// # Arguments
    /// * `func` - A function to be executed.
    pub fn execute<F>(&mut self, func: F)
    where
        F: FnOnce() -> R + Send + 'static,
    {
        if std::thread::current()
            .name()
            .unwrap()
            .contains("ferrumc_threadpool")
        {
            panic!("Thread pool is trying to run a task on itself, this is not allowed");
        }
        let boxed = move || Box::new(func());
        let handle = self.pool.evaluate(boxed);
        self.handles.push(handle);
    }

    /// Executes a task in the thread pool batch without checking for thread safety.
    ///
    /// # Safety
    /// This function does the same as `execute`, but does not check if the thread pool has been
    /// moved to a different thread. When the threadpool is full, if those threads try spawn things
    /// on the threadpool it can deadlock.
    ///
    /// Imagine we have a threadpool with only 1 thread and 2
    /// tasks to complete, A and B. If A is run on the threadpool and then tries to spawn B on the
    /// threadpool, B can't start until A finishes, but A can't finish until B completes. This
    /// scenario is less likely to happen with a threadpool with more threads, but it can still
    /// happen, so don't use this function unless you absolutely need to.
    ///
    /// This function is unsafe because it allows you to bypass the thread safety checks that are
    /// normally enforced by the `execute` method. Use it only if you are sure that the thread pool
    /// has not been moved to a different thread and that it is safe to execute the task.
    ///
    /// You generally won't be able to move the threadpool to a different thread due to borrow
    /// checker nonsense, but if you figure out a way, you can use this function to bypass the
    /// panic that would happen in `execute`.
    ///
    /// # Arguments
    /// * `func` - A function to be executed.
    ///
    /// # Returns
    /// The result of the executed function.
    pub unsafe fn execute_unchecked<F>(&mut self, func: F)
    where
        F: FnOnce() -> R + Send + 'static,
    {
        let boxed = move || Box::new(func());
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

    /// Waits for the next task in the batch to complete and returns its result.
    ///
    /// # Returns
    /// An `Option` containing the result of the completed task, or `None` if there are no more tasks.
    pub fn wait_next(&mut self) -> Option<R> {
        self.completed = true;

        if let Some(handle) = self.handles.pop() {
            let result = handle.await_complete();
            Some(*result)
        } else {
            None
        }
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
