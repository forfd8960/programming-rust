use std::collections::BinaryHeap;

/*
A BinaryHeap is a collection whose elements are kept loosely organized
so that the greatest value always bubbles up to the front of the queue
*/
fn main() {
    bin_heap();
}

fn bin_heap() {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    println!("is heap empty: {}", heap.is_empty()); // true
    heap.push(8);
    heap.push(9);
    heap.push(7);
    heap.push(6);
    heap.push(100);

    // is heap empty: false
    println!("is heap empty: {}", heap.is_empty()); // false

    // heap cap: 8
    println!("heap cap: {}", heap.capacity());

    // remove the greatest item from the heap, and return
    // remove the greatest item: Some(100)
    println!("remove the greatest item: {:?}", heap.pop());
    assert_eq!(Some(9), heap.pop());
    assert_eq!(Some(&8), heap.peek());

    if let Some(mut top) = heap.peek_mut() {
        *top = 1000;
    }
    println!("top: {:?}", heap.peek());

    // consume values from binary heap in order
    /*
    top: 1000
    top: 7
    top: 6
    */
    while let Some(top) = heap.pop() {
        println!("top: {}", top);
    }
}
