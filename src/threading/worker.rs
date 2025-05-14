use std::thread::JoinHandle;

type Task = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
    Task(Task),
    Break,
}

pub struct Worker {
    pub idx: usize,
    pub handle: JoinHandle<()>,
}
