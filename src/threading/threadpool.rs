use std::{
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender, channel},
    },
    thread::{self, JoinHandle},
};

use super::worker::{Message, Worker};

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Sender<Message>,
    rx: Arc<Mutex<Receiver<Message>>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (tx, rx) = channel::<Message>();

        let rx = Arc::new(Mutex::new(rx));

        let workers = Vec::with_capacity(size);

        let mut pool = ThreadPool { workers, tx, rx };

        for _ in 0..size {
            pool.add_worker();
        }

        pool
    }

    fn generate_handle(rx: Arc<Mutex<Receiver<Message>>>, idx: usize) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                match rx.lock().unwrap().recv().unwrap() {
                    Message::Task(fn_once) => fn_once(),
                    Message::Break => {
                        println!("Closing thread: {}", idx);
                        break;
                    }
                }
            }
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Box::new(f);

        self.tx.send(Message::Task(task)).unwrap();
    }

    pub fn add_worker(&mut self) {
        let rx = Arc::clone(&self.rx);
        let handle = ThreadPool::generate_handle(rx, self.workers.len());
        let idx = self.workers.len();
        self.workers.push(Worker { idx, handle });
    }

    pub fn close(mut self) {
        for _ in 0..self.workers.len() {
            self.tx.send(Message::Break).unwrap();
        }
        self.workers = vec![]
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.workers.len() {
            self.tx.send(Message::Break).unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use super::ThreadPool;

    #[test]
    fn pool_test() {
        let mut pool = ThreadPool::new(1);

        pool.add_worker();
        pool.add_worker();

        pool.execute(|| println!("hello"));
        pool.close();
    }
}
