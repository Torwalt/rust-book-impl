use std::{
    sync::mpsc,
    sync::{
        mpsc::{Receiver, SendError, Sender},
        Arc, Mutex,
    },
    thread,
};

#[derive(Debug)]
pub enum Error {
    InvalidInput(String),
    SendError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidInput(s) => write!(f, "Invalid input {s}"),
            Error::SendError(s) => write!(f, "Send failed {s}"),
        }
    }
}

impl std::error::Error for Error {}

impl<T> From<SendError<T>> for Error {
    fn from(error: SendError<T>) -> Self {
        Error::SendError(format!("Failed to send a message: {:?}", error))
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Threadpool {
    workers: Vec<Worker>,
    snd: Option<Sender<Job>>,
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

        let (snd, rcv): (Sender<Job>, Receiver<Job>) = mpsc::channel();

        let rcv = Arc::new(Mutex::new(rcv));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&rcv)));
        }

        Ok(Threadpool {
            workers,
            snd: Some(snd),
        })
    }

    pub fn execute<F>(&self, f: F) -> Result<(), Error>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.snd.as_ref().unwrap().send(job)?;

        Ok(())
    }
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        drop(self.snd.take());

        for worker in &mut self.workers {
            println!("Shutting down Worker {}", worker.id);

            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, rcv: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let msg = rcv.lock().unwrap().recv();

            match msg {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }

                Err(_) => {
                    println!("Worker {id} disconnected.");
                    break;
                }
            }
        });

        Worker {
            id,
            handle: Some(thread),
        }
    }
}
