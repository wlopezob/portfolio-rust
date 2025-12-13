use std::{sync::{Arc, Mutex}, thread, time::{Duration, Instant}, vec};

struct Config {
    server_url: String,
    timeout: u64,
    max_retries: u32
}

struct WorkQueue {
    tasks: Mutex<Vec<String>>,
}

impl WorkQueue {
    fn new() -> Self {
        WorkQueue {
            tasks: Mutex::new(vec![
                "Task 1".to_string(),
                "Task 2".to_string(),
                "Task 3".to_string(),
                "Task 4".to_string(),
                "Task 5".to_string(),
            ]),
        }
    }
    
    fn get_task(&self) -> Option<String> {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.pop()
    }
}

fn main() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = std::thread::spawn(move || {
            println!("Thread: {:?}: {:?}", i, data_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Main thread: {:?}", data);
    println!("Reference count: {}", Arc::strong_count(&data));

    ////////////////
    let config = Arc::new(Config {
        server_url: String::from("https://api.example.com"),
        timeout: 30,
        max_retries: 5,
    });

    let mut handles = vec![];

    for i in 0..5 {
        let config_clone = Arc::clone(&config);
        let handle = std::thread::spawn(move || {
            println!("Thread {}: Connecting to {} with timeout {}s and max retries {}", 
                i, 
                config_clone.server_url, 
                config_clone.timeout, 
                config_clone.max_retries
            );
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    print!("Final reference count: {}", Arc::strong_count(&config));

    /// Mutex /////////////
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap()); // 10

    /// WorkQueue with Mutex /////////////
    let queue = Arc::new(WorkQueue::new());
    let mut handles = vec![];
    let start = Instant::now();
    
    // Crear 3 workers
    for worker_id in 0..3 {
        let queue_clone = Arc::clone(&queue);
        
        let handle = thread::spawn(move || {
            let mut task_count = 0;
            loop {
                match queue_clone.get_task() {
                    Some(task) => {
                        task_count += 1;
                        println!(
                            "[{:?}] Worker {} processing: {} (task #{} for this worker)",
                            start.elapsed(),
                            worker_id,
                            task,
                            task_count
                        );
                        thread::sleep(Duration::from_millis(500));
                    }
                    None => {
                        println!(
                            "[{:?}] Worker {} finished (processed {} tasks total)",
                            start.elapsed(),
                            worker_id,
                            task_count
                        );
                        break;
                    }
                }
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("All tasks completed!");
}