use core::cmp::Ordering;

#[derive(Debug)]
struct MyData {
    name: String,
    price: u32,
}

fn main() {
    println!("sum(10): {}", sum(10));
    println!("product(10): {}", product(10));

    let data = vec![3, 2, 99, 6, -1];
    println!("max: {:?}", vec_max(&data));
    println!("max: {:?}", vec_min(&data));

    let data = vec!["abc", "d", "ef", "HelloHelloWorld"];
    let max_str = max_length_str(&data);
    // max length str: Some("HelloHelloWorld")
    println!("max length str: {:?}", max_str);
    // min length str: Some("d")
    println!("min length str: {:?}", min_length_str(&data));

    let data_list = vec![
        MyData {
            name: "Apple".to_string(),
            price: 10,
        },
        MyData {
            name: "Watermelon".to_string(),
            price: 30,
        },
        MyData {
            name: "Tomato".to_string(),
            price: 6,
        },
    ];
    // max data list: Some(MyData { name: "Watermelon", price: 30 })
    println!("max of data list: {:?}", max_mydata(&data_list));
    // min data list: Some(MyData { name: "Tomato", price: 6 })
    println!("min of data list: {:?}", min_mydata(&data_list)); // Tomato

    max_by_key();
    min_by_key();

    iter_eq();
    any_all();

    position();

    fold();
    join_str();
}

fn sum(n: u32) -> u32 {
    (1..n).sum()
}

fn product(n: i64) -> i64 {
    (1..n).product()
}

fn vec_max(data: &Vec<i32>) -> Option<&i32> {
    data.iter().max()
}

fn vec_min(data: &Vec<i32>) -> Option<&i32> {
    data.iter().min()
}

fn compare<'a, 'b>(lhs: &'a &&str, rhs: &'b &&str) -> Ordering {
    lhs.len().cmp(&rhs.len())
}

fn max_length_str<'a>(data: &'a Vec<&str>) -> Option<&'a &'a str> {
    data.iter().max_by(compare)
}

fn min_length_str<'a>(data: &'a Vec<&str>) -> Option<&'a &'a str> {
    data.iter().min_by(compare)
}

fn compare_data<'a, 'b>(lhs: &'a &MyData, rhs: &'b &MyData) -> Ordering {
    lhs.price.cmp(&rhs.price)
}

fn max_mydata(data_list: &Vec<MyData>) -> Option<&MyData> {
    data_list.iter().max_by(compare_data)
}

fn min_mydata(data_list: &Vec<MyData>) -> Option<&MyData> {
    data_list.iter().min_by(compare_data)
}

fn max_by_key() {
    let data_list = vec![
        MyData {
            name: "Apple".to_string(),
            price: 10,
        },
        MyData {
            name: "Watermelon".to_string(),
            price: 30,
        },
        MyData {
            name: "Tomato".to_string(),
            price: 6,
        },
    ];

    // max by key: Some(MyData { name: "Watermelon", price: 30 })
    let max = data_list.iter().max_by_key(|d| d.price);
    println!("max by key: {:?}", max);
}

fn min_by_key() {
    let data_list = vec![
        MyData {
            name: "Apple".to_string(),
            price: 10,
        },
        MyData {
            name: "Watermelon".to_string(),
            price: 30,
        },
        MyData {
            name: "Tomato".to_string(),
            price: 6,
        },
        MyData {
            name: "Tomato".to_string(),
            price: 6,
        },
    ];

    // min by key: Some(MyData { name: "Tomato", price: 6 })
    let min = data_list.iter().min_by_key(|d| d.price);
    println!("min by key: {:?}", min);
}

/*
Hello Good Morning is not equal to Hello
         Good
         Morning: true
but they are equal to if skip whitespace: true
*/
fn iter_eq() {
    let v1 = "Hello Good Morning";
    let v2 = "Hello \n\t Good \n\t Morning";
    println!("{} is not equal to {}: {}", v1, v2, v1 != v2);
    println!(
        "but they are equal to if skip whitespace: {}",
        v1.split_whitespace().eq(v2.split_whitespace())
    );
}

/*
has word: true
all_positive: true
is_free: false
*/
fn any_all() {
    let v = vec!["HI", "good", "day"];
    let has_word = v.iter().any(|s| s.ends_with("d"));
    println!("has word: {}", has_word);

    let nums = vec![2, 7, 1024, 9, 808];
    let all_positive = nums.iter().all(|num| i32::is_positive(*num));
    println!("all_positive: {}", all_positive);

    let data_list = vec![
        MyData {
            name: "Apple".to_string(),
            price: 10,
        },
        MyData {
            name: "Watermelon".to_string(),
            price: 30,
        },
        MyData {
            name: "Tomato".to_string(),
            price: 6,
        },
        MyData {
            name: "Disposable plastic bags".to_string(),
            price: 0,
        },
    ];
    let is_free = data_list.iter().all(|data| data.price <= 0);
    println!("is_free: {}", is_free);
}

fn position() {
    let text = "Turn our golden faces into the sun";
    let words = text.split(" ");

    // Some(2)
    println!("{:?}", words.into_iter().position(|w| w.eq("golden")));

    let words1 = b"Hello, Friend";

    // Some(2)
    println!("{:?}", words1.into_iter().rposition(|&c| c == b'e'));
}

// How to calculate count, sum, product on the same iterator
fn fold() {
    let data = 1..10;
    let data1 = data.clone();
    let data2 = data.clone();

    println!("{}", data.into_iter().fold(0, |n, _| n + 1)); // 10 count

    println!("{}", data1.into_iter().fold(0, |n, i| n + i)); // 45 sum

    println!("{}", data2.into_iter().fold(1, |n, i| n * i)); // 362880 product
}

fn join_str() {
    let words = vec!["awesome", "rust", "iterator"];
    let sentense = words.iter().fold(String::new(), |s, w| s + w + " ");
    println!("{}", sentense);
}
