use std::thread;
#[derive(Debug)]
pub enum Error {
    InvalidInput(String),
}

pub struct Threadpool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl Threadpool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// Returns InvalidInput error when size is 0.
    pub fn new(size: usize) -> Result<Threadpool, Error> {
        if size == 0 {
            return Err(Error::InvalidInput("size must be greater 0".to_string()));
        }

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {}

        Ok(Threadpool { threads })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
