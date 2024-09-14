use std::sync::{Arc, Mutex};

use tokio::{
    sync::oneshot,
    time::{sleep, Duration},
};

#[derive(Clone)]
struct MonoTaskHandler {
    tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

impl MonoTaskHandler {
    pub fn new() -> Self {
        Self {
            tx: Arc::new(Mutex::new(None)),
        }
    }

    // Send stop message to previous task.
    // Return handle for new task.
    pub fn take(&mut self) -> oneshot::Receiver<()> {
        let mut tx_lock = self.tx.lock().unwrap();
        if let Some(tx) = tx_lock.take() {
            let _ = tx.send(());
        }
        let (tx, rx) = oneshot::channel();
        *tx_lock = Some(tx);
        rx
    }

    pub fn stop_current(&mut self) {
        let mut tx_lock = self.tx.lock().unwrap();
        if let Some(tx) = tx_lock.take() {
            let _ = tx.send(());
        }
        *tx_lock = None;
    }
}

#[derive(Clone)]
pub struct MrMono {
    task_handler: MonoTaskHandler,
}

#[derive(Debug)]
pub enum TaskResult {
    Completed,
    Cancelled,
}

async fn say(message: String) {
    for i in 0..10 {
        println!("{}:{}", i, message);
        sleep(Duration::from_millis(10)).await;
    }
}

impl MrMono {
    pub fn new() -> Self {
        Self {
            task_handler: MonoTaskHandler::new(),
        }
    }

    pub async fn say(&mut self, message: String) -> TaskResult {
        tokio::select! {
            _ = self.task_handler.take()=> {
                TaskResult::Cancelled
            }
            _ = say(message)=> {
                TaskResult::Completed
            }
        }
    }

    pub fn stop_current(&mut self) {
        self.task_handler.stop_current()
    }
}
