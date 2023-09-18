use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<i32> = VecDeque::new();

    /*
    queue: [15, 10]
    */
    // Adds a value at the front of the queue.
    queue.push_front(10);
    queue.push_front(15);
    println!("queue: {:?}", queue);

    // [15, 10, 100]
    // add value at the end of queue
    queue.push_back(100);
    println!("{queue:?}");

    // pop from front: Some(15)
    // revmove and return the front value of queue
    println!("pop from front: {:?}", queue.pop_front());

    // remove and return the value from the back
    // pop from back: Some(100)
    println!("pop from back: {:?}", queue.pop_back());

    // front value: Some(10), back value: Some(10)
    println!(
        "front value: {:?}, back value: {:?}",
        queue.front(),
        queue.back()
    );

    queue.push_back(20);
    if let Some(v) = queue.front_mut() {
        *v *= 10;
    }
    if let Some(v) = queue.back_mut() {
        *v *= 10;
    }
    // queue: [100, 200]
    println!("queue: {queue:?}");

    create_vecdeque();

    stack();

    queue_ops();
}

fn create_vecdeque() {
    let deque = VecDeque::from(vec![1, 10, 6]);
    /*
        Some(1)
    Some(6)
    false
        */
    println!("{:?}", deque.front());
    println!("{:?}", deque.back());
    println!("{}", deque.is_empty());
}

fn stack() {
    let mut stack: VecDeque<i32> = VecDeque::new();
    stack.push_front(1);
    stack.push_front(3);
    stack.push_front(6);

    // should be 6: Some(6)
    println!("should be 6: {:?}", stack.pop_front()); // 6
}

fn queue_ops() {
    let mut q: VecDeque<i32> = VecDeque::new();
    q.push_back(1);
    q.push_back(8);
    q.push_back(99);

    // pop front: Some(1)
    println!("pop front: {:?}", q.pop_front()); // 1
}
