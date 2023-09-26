use std::{
    sync::Arc,
    sync::RwLock,
    thread::{self, JoinHandle},
};

#[derive(Debug)]
struct AppConfig {
    count: i32,
}

impl AppConfig {
    fn new(init: i32) -> Self {
        Self { count: init }
    }

    fn read_count(&self) -> i32 {
        self.count
    }

    fn incre_count(&mut self, delta: i32) -> i32 {
        self.count += delta;
        self.count
    }
}

fn main() {
    let config = Arc::new(RwLock::new(AppConfig::new(0)));

    let mut handles: Vec<JoinHandle<_>> = vec![];

    for idx in 0..10 {
        let conf1 = Arc::clone(&config);

        let read_hdl = thread::spawn(move || {
            println!("read count in: {}", idx);
            let config = conf1.read().unwrap();
            println!("read count: {}", config.read_count());
        });

        handles.push(read_hdl);

        let conf2 = Arc::clone(&config);
        let hdl = thread::spawn(move || {
            println!("add 1 to config: {}", idx);
            let mut config = conf2.write().unwrap();
            config.incre_count(1);
            println!("config count: {}", config.count);
        });

        handles.push(hdl);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let conf = Arc::clone(&config);
    println!(
        "count after concurrently increase: {:?}",
        conf.read().unwrap().read_count()
    );
}
