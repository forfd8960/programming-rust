// #[derive(Debug)]
// struct MyDataRange {
//     start: i32,
//     end: i32,
//     current: i32,
// }

// impl IntoIterator for MyDataRange {
//     type Item = i32;
//     type IntoIter = std::vec::IntoIter<i32>;

//     fn into_iter(self) -> Self::IntoIter {

//     }
// }

#[derive(Debug)]
struct Element {
    value: i32,
}

fn main() {
    run_iterator();
    run_into_iterator();
    call_next();
    into_iter();
}

/*
As we’ve said, an iterator is any type that implements Iterator.
*/
fn run_iterator() {
    let elements = vec!["A", "B", "D", "F"];
    for v in &elements {
        println!("{}", v);
    }
}

/*
An iterable is any type that implements IntoIterator:
   you can get an iterator over it by calling its into_iter method.
   The vector reference &v is the iterable in this case.
*/
fn run_into_iterator() {
    println!("vector.into_iter()...");

    let elements = vec!["A", "B", "D", "F"];
    let mut iter = (&elements).into_iter();
    while let Some(v) = iter.next() {
        println!("{}", v);
    }
}

fn call_next() {
    let nums = vec![3, 2, 9, 0];
    let mut iter = nums.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&9));
    assert_eq!(iter.next(), Some(&0));
}

fn into_iter() {
    println!("calling into_iter for slice");

    let v1 = vec![1, 2, 3];

    // Given a shared reference to the collection,
    // into_iter returns an iterator that produces shared references to its items
    for e in &v1 {
        println!("{}", e); // e: &i32
    }

    let mut v2 = vec![
        Element { value: 1 },
        Element { value: 3 },
        Element { value: 1000 },
    ];
    // Given a mutable reference to the collection,
    // into_iter returns an iterator that produces mutable references to the items.
    for e in &mut v2 {
        print!("{:?} ", e); // e: &mut Element
        println!("{:?}", e.value);
    }

    // When passed the collection by value,
    // into_iter returns an iterator that takes ownership of the collection and returns items by value
    let v3 = vec!["hello", "hello", "good", "day"];
    for v in v3 {
        println!("{}", v); // v: &str
    }
}
