use std::sync::{Arc, Mutex, mpsc};

use crate::errors::EnkryptitError;
use crate::parallelism::EnkryptitExecutable;
use crate::parallelism::EnkryptitJob;
use crate::parallelism::worker::EnkryptitWorker;

pub struct EnkryptitPool<T: EnkryptitExecutable> {
    _workers: Vec<EnkryptitWorker<T>>,
    sender: mpsc::SyncSender<EnkryptitJob<T>>,
    receiver: mpsc::Receiver<Result<T::Output, EnkryptitError>>,
}

impl<T: EnkryptitExecutable + Send + 'static> EnkryptitPool<T> {
    pub fn new(size: usize) -> Result<Self, EnkryptitError> {
        if size == 0 {
            return Err(EnkryptitError::InvalidWorkerCount);
        }
        // Jobs : bounded for limiting RAM
        let (sender_job, receiver_job) = mpsc::sync_channel(size * 2);

        let (sender_result, receiver_result) = mpsc::channel();

        let receiver_job = Arc::new(Mutex::new(receiver_job));

        let mut workers: Vec<EnkryptitWorker<T>> = Vec::with_capacity(size);

        for id in 0..size {
            let receiver = Arc::clone(&receiver_job);
            let sender = sender_result.clone();

            workers.push(EnkryptitWorker::new(id, receiver, sender));
        }

        drop(sender_result);

        Ok(Self {
            _workers: workers,
            sender: sender_job,
            receiver: receiver_result,
        })
    }

    pub fn submit(&self, job: EnkryptitJob<T>) -> Result<(), EnkryptitError> {
        self.sender.send(job).map_err(|_| EnkryptitError::SendError)
    }

    pub fn recv(&self) -> Result<Result<T::Output, EnkryptitError>, EnkryptitError> {
        self.receiver
            .recv()
            .map_err(|e| EnkryptitError::ReceiveError(e))
    }
}
