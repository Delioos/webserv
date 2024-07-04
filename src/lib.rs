use std::{thread, sync::{Arc, mpsc, Mutex}};

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<std::sync::mpsc::Receiver<Job>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });


        Worker {
            id,
            thread
        }
    }
}


pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;


impl ThreadPool {
    pub fn new(_size: usize) -> ThreadPool {
        assert!(_size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers= Vec::with_capacity(_size);

        for id in 0.._size {
            // create some threads and store them in the vector
            // essai perso
            let worker: Worker = Worker::new(id, Arc::clone(&receiver));
            workers.push(worker);
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where 
            F: FnOnce() -> () + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.send(job).unwrap();
        }
}
