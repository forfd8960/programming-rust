use core::time::Duration;
use std::{
    sync::atomic::{AtomicBool, AtomicIsize, Ordering},
    thread,
};

use std::sync::Arc;

fn main() {
    let atom = AtomicIsize::new(0);
    atom.fetch_add(1, Ordering::SeqCst);
    println!("atom isize: {:?}", atom.load(Ordering::SeqCst));

    let cancel_flag = Arc::new(AtomicBool::new(false));
    let worker_flag = Arc::clone(&cancel_flag);
    let workder_thread = thread::spawn(move || {
        let mut result = 0;
        for i in 1..100 {
            println!("rendering: {}", i);
            // after render each pixel, the thread checks the value of the flag by calling .load() method:
            if worker_flag.load(Ordering::SeqCst) {
                return None;
            }
            result += i;
        }
        Some(result)
    });

    // let cancel_thread_flag = Arc::clone(&cancel_flag);
    let cancel_thread = thread::spawn(move || {
        println!("waiting 10 micro sec to render");
        thread::sleep(Duration::from_micros(10));
        // in the main thread decide to cancel the worker thread.
        cancel_flag.store(true, Ordering::SeqCst);
    });

    cancel_thread.join().unwrap();

    let res = workder_thread.join().unwrap();
    println!("workder_thread result: {:?}", res);
}
