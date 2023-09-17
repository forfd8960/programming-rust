fn main() {
    create_vector();

    access_elements();
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
