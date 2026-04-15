// EXPECT: Blocking in Async
// EXPECT: Deadlock Risk
// EXPECT: Spawn Without Join
// EXPECT: Missing Send Bound
// EXPECT: Sync Drop Blocking
// EXPECT: Std Mutex in Async
// EXPECT: Blocking Channel in Async
// EXPECT: Holding Lock Across Await
// EXPECT: Dropped JoinHandle

#![allow(dead_code, unused_variables)]

use std::sync::{mpsc, Mutex};

async fn blocking_io() {
    let _ = std::fs::read_to_string("config.toml");
}

fn inconsistent_locks(a: &Mutex<i32>, b: &Mutex<i32>) {
    let _a = a.lock().unwrap();
    let _b = b.lock().unwrap();
}

fn inconsistent_locks_again(a: &Mutex<i32>, b: &Mutex<i32>) {
    let _b = b.lock().unwrap();
    let _a = a.lock().unwrap();
}

async fn spawn_detached() {
    tokio::spawn(async {});
    let _ = tokio::spawn(async {});
}

trait Job {
    fn run(self);
}

fn spawn_generic<T: Job + 'static>(job: T) {
    tokio::spawn(async move {
        job.run();
    });
}

struct Client;
impl Client {
    fn flush_blocking(&mut self) {
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

async fn std_mutex_in_async(lock: &Mutex<i32>) {
    let guard = lock.lock().unwrap();
    pending().await;
    let _ = guard;
}

async fn blocking_channel(rx: mpsc::Receiver<i32>) {
    let _ = rx.recv().unwrap();
}

async fn holding_lock(lock: tokio::sync::Mutex<i32>) {
    let guard = lock.lock().await;
    pending().await;
    let _ = guard;
}

async fn pending() {}
