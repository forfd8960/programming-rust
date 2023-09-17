fn main() {
    create_vector();

    access_elements();

    vector_capacity();
}

// [], ["rust", "vector"], [0, 255]
fn create_vector() {
    // capacity: 0, length: 0
    let mut numbers: Vec<&str> = vec![];

    // capacity: 2, length: 2
    let words = vec!["rust", "vector"];

    // capacity: 255, length: 255
    let mut buffer = vec![0u8, 255]; // 255 zeroed-out bytes
    println!("{:?}, {:?}, {:?}", numbers, words, buffer);
}

/*
burger
cloned: burger
second: king
[105, 115, 32, 105, 115, 32, 97]
buffer_copy: [105, 115, 32, 105, 115, 32, 97]
first from first method: "burger"
last from last method: "king"
get from data.get(1): "king"
first after mut: 300
last after mut: Some(1000)
the 3rd after mut: Some(920)
*/
fn access_elements() {
    let data = vec!["burger", "king"];
    let first = &data[0];
    let first_clone = data[0].clone();
    println!("{}", first);
    println!("cloned: {}", first_clone);

    let second = data[1];
    println!("second: {}", second);

    let buffer = b"this is awesome day";
    let my_buffer = &buffer[2..9]; // get reference to a slice
    let buffer_copy = &buffer[2..9].to_vec(); // a copy of slice
    println!("{:?}", my_buffer);
    println!("buffer_copy: {:?}", buffer_copy);

    // first return Option<&str>
    // The return type is Option<&T>, so the return value is None if slice is empty and Some(&slice[0]) if it’s not empty:
    if let Some(fst) = data.first() {
        println!("first from first method: {:?}", fst);
    }

    if let Some(lst) = data.last() {
        println!("last from last method: {:?}", lst);
    }

    if let Some(v) = data.get(1) {
        println!("get from data.get(1): {:?}", v);
    }

    let mut nums = vec![3, 100, 8, 92, 10];
    if let Some(first_mut) = nums.first_mut() {
        *first_mut = 300;
    }
    println!("first after mut: {}", nums[0]); // 300

    if let Some(lst_mut) = nums.last_mut() {
        *lst_mut = 1000;
    }
    println!("last after mut: {:?}", nums.last()); // 1000

    if let Some(get_mut) = nums.get_mut(3) {
        *get_mut = 920
    }
    println!("the 3rd after mut: {:?}", nums.get(3)); // 920

    // clone whole slice, returning new vector.
    let nums_clone = nums.to_vec();
    println!("{:?}", nums_clone);
}

// capacity: 10, length: 0
/*
poped: 890
vector: [123]
vector after pop: [123]
vector after insert at 0: [120, 123]
vector after insert at 1: [120, 122, 123]
vector after remove at 0: [122, 123, 256]
*/
fn vector_capacity() {
    let mut v1: Vec<i32> = Vec::with_capacity(10); // Creates a new, empty vector with capacity n.
    println!("capacity: {}, length: {}", v1.capacity(), v1.len());

    v1.push(123);
    v1.push(890);

    // Removes and returns the last element
    if let Some(v) = v1.pop() {
        println!("poped: {}", v); // 890
    }

    // [123]
    println!("vector after pop: {:?}", v1);

    v1.insert(0, 120);
    println!("vector after insert at 0: {:?}", v1); // [120, 123]
    v1.insert(1, 122);
    println!("vector after insert at 1: {:?}", v1); // [120, 122, 123]

    v1.insert(3, 256);
    // The longer the vector, the slower this operation gets
    // Both .insert() and .remove() are slower the more elements have to be shifted.
    v1.remove(0);
    println!("vector after remove at 0: {:?}", v1); // [122, 123, 256]

    v1.resize(20, 0);
    // length: 20, cap: 20
    println!("length: {}, cap: {}", v1.len(), v1.capacity());

    v1.truncate(10);
    println!(
        "after truncate -- length: {}, cap: {}",
        v1.len(),
        v1.capacity()
    );

    // length: 13, cap: 20, elements: [122, 123, 256, 0, 0, 0, 0, 0, 0, 0, 456, 1000, 1024]
    v1.extend_from_slice(vec![456, 1000, 1024].as_slice());
    println!(
        "length: {}, cap: {}, elements: {:?}",
        v1.len(),
        v1.capacity(),
        v1,
    );

    let split_vec = v1.split_off(6);
    /*
    split_vec: [0, 0, 0, 0, 456, 1000, 1024]
    origin_vec: [122, 123, 256, 0, 0, 0]
    */
    println!("split_vec: {:?}", split_vec);
    println!("origin_vec: {:?}", v1);

    let mut vec2 = vec![256, 1008];
    v1.append(&mut vec2);
    /*
    vec2: []
    , v1: [122, 123, 256, 0, 0, 0, 256, 1008]
    */
    println!("vec2: {:?}\n, v1: {:?}", vec2, v1);

    // Removes the specified range from the vector in bulk, returning all removed elements as an iterator.
    let drained: Vec<i32> = v1.drain(3..6).collect();
    // drained: [0, 0, 0]
    println!("drained: {:?}", drained);
    // [122, 123, 256, 256, 1008]
    println!("v1: {:?}", v1);

    v1.push(257);

    // Removes all elements that don’t pass the given test.
    let retain = v1.retain(|v| *v % 2 == 0);
    // retain: (), v1: [122, 256, 256, 1008]
    println!("retain: {:?}, v1: {:?}", retain, v1);

    let mut duplicated = b"Helllllllllooooo, Goooooooooood".to_vec();
    duplicated.dedup();
    // after dedup: [72, 101, 108, 111, 44, 32, 71, 111, 100]
    println!("after dedup: {:?}", duplicated);
    assert_eq!(&duplicated, b"Helo, God");
}
