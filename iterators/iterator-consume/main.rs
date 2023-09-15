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
