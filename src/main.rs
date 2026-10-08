use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;
use tokio;

struct Work {
    text: String,
}

impl Work {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

async fn worker(
    id: i32,
    rx: mpsc::Receiver<Work>,
    worker_map: Arc<Mutex<HashMap<i32, mpsc::Sender<Work>>>>,
) {
    println!(
        "Dedicated Worker for {}: Worker started listening, timeout 10 seconds",
        id
    );

    loop {
        match rx.recv_timeout(Duration::from_secs(10)) {
            Ok(messsage) => {
                let wait_duration = 4000;
                let _ = tokio::time::sleep(Duration::from_millis(wait_duration as u64)).await;

                println!("Dedicated Worker for {}: {}", id, messsage.text.to_string());
            }

            Err(mpsc::RecvTimeoutError::Timeout) => {
                println!(
                    "Dedicated Worker for {}: Last message was received more than 10 seconds ago, disposing...",
                    id
                );
                break;
            }

            Err(mpsc::RecvTimeoutError::Disconnected) => {
                println!("Dedicated Worker for {}: Disconnected...closing worker", id);
                break;
            }
        }
    }

    {
        let _ = worker_map.lock().unwrap().remove(&id);
    }
    println!("Dedicated Worker for {}: Worker Disposed", id);
}

struct WorkerDispatcher {
    workers: Arc<Mutex<HashMap<i32, mpsc::Sender<Work>>>>,
}

impl WorkerDispatcher {
    fn new() -> Self {
        Self {
            workers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn send(&self, id: i32, text: &str) {
        match self.workers.lock().unwrap().get(&id) {
            Some(sender) => {
                let _ = sender.send(Work::new(text));
            }
            None => println!("Cannot find worker for id {}", id),
        };
    }

    fn dispatch(&mut self, id: i32, text: &str) {
        if self.workers.lock().unwrap().contains_key(&id) {
            self.send(id, text);
            return;
        }

        self.spawn_and_send(id, text);
    }

    fn spawn_and_send(&mut self, id: i32, text: &str) {
        let (tx, rx) = mpsc::channel();
        {
            self.workers.lock().unwrap().insert(id, tx);
        }
        tokio::spawn(worker(id, rx, self.workers.clone()));
        self.send(id, text);
    }
}

#[tokio::main]
async fn main() {
    let mut work_dispatcher = WorkerDispatcher::new();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Main thread: failed to read line");

        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("exit") {
            print!("Main thread: Goodbye");
            break;
        }

        let operations: Vec<&str> = trimmed.split(";").collect();
        for operation in operations {
            let operation_split: Vec<&str> = operation.split(",").collect();

            let id = match operation_split[0].parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Cannot parse: {}", operation);
                    continue;
                }
            };

            work_dispatcher.dispatch(id, operation_split[1]);
        }
    }
}
