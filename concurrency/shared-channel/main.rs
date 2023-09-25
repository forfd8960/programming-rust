use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};

#[derive(Clone)]
pub struct SharedReceiver<T>(Arc<Mutex<Receiver<T>>>);

impl<T> Iterator for SharedReceiver<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let guard = self.0.lock().unwrap();
        // Covnert Result<T, E> to Option<T>
        guard.recv().ok()
    }
}

// create a new channel, whose receiver can be shared accross threads.
pub fn shared_channel<T>() -> (Sender<T>, SharedReceiver<T>) {
    let (sender, receiver) = channel();
    (sender, SharedReceiver(Arc::new(Mutex::new(receiver))))
}

fn main() {}
