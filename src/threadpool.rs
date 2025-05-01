use std::{
    sync::{
        Arc, Mutex,
        mpsc::{Sender, channel},
    },
    thread::{self, JoinHandle},
};

type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    _workers: Vec<JoinHandle<()>>,
    tx: Sender<Task>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (tx, rx) = channel::<Task>();

        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            let rx = Arc::clone(&rx);
            workers.push(thread::spawn(move || {
                loop {
                    rx.lock().unwrap().recv().unwrap()();
                }
            }));
        }

        ThreadPool {
            _workers: workers,
            tx,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Box::new(f);

        self.tx.send(task).unwrap();
    }
}
