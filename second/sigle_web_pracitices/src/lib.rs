use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    works: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(len: usize) -> ThreadPool {
        assert!(len > 0);

        let (sender, reciver) = mpsc::channel();

        let reciver = Arc::new(Mutex::new(reciver));

        let mut works = Vec::with_capacity(len);

        for id in 0..len {
            works.push(Worker::new(id, Arc::clone(&reciver)));
        }

        ThreadPool { works, sender }
    }

    pub fn execute<F>(&self, handler: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(handler);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        // 尚未实现..
        let thread = thread::spawn(move || {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job;execute");
            job();
        });
        // 每个 `Worker` 都拥有自己的唯一 id
        Worker { id, thread }
    }
}
