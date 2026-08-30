use std::{
    marker::PhantomData, sync::{
        Arc,
        Mutex,
        mpsc::{Receiver, Sender},
    }, thread::{self, JoinHandle},
};

use crate::{errors::EnkryptitError, parallelism::{
    EnkryptitJob, executable::EnkryptitExecutable,
}};

pub struct EnkryptitWorker<T: EnkryptitExecutable> {
    id: usize,
    thread: JoinHandle<()>,
    _phantom: PhantomData<T>
}

impl<T: EnkryptitExecutable + Send + 'static> EnkryptitWorker<T> {
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<Receiver<EnkryptitJob<T>>>>,
        sender: Sender<Result<T::Output, EnkryptitError>>,
    ) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let job = match receiver.lock().unwrap().recv() {
                    Ok(job) => job,
                    Err(_) => break,
                };

                println!("Worker {} got job {}", id, job.index);

                let result = job.execute();

                if sender.send(result).is_err() {
                    break;
                }
            }
        });

        Self {
            id,
            thread,
            _phantom: PhantomData,
        }
    }
}