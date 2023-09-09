use std::collections::VecDeque;

// Generic Structure
#[derive(Debug)]
struct Queue<T> {
    queue: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, data: T) {
        self.queue.push_back(data)
    }

    fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

fn main() {
    let mut queue = Queue::<i32>::new();
    queue.enqueue(10);
    queue.enqueue(100);
    queue.enqueue(10000);

    println!("dequeue: {:?}", queue.dequeue());
    println!("queue isempty: {:?}", queue.is_empty());
}
