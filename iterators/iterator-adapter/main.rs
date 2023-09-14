use std::{collections::HashMap, str::FromStr};

fn main() {
    let text = " A Awesome\n day\n  beautiful sunny\n day\n".to_string();
    trim_strings(text);
    filter_element();
    filter_map();
    flat_map();
    flatten();
    take_while();
    skip();
    // peek();
    rev();
    chain();
}

// Adapater Map
fn trim_strings(text: String) {
    let v: Vec<&str> = text.lines().map(str::trim).collect();
    // trimed strings: ["A Awesome", "day", "beautiful sunny", "day"]
    println!("trimed strings: {:?}", v);
}

// Adapater Filter
fn filter_element() {
    let nums = vec![1, 3, 2, 8, 15, 99, 1024];
    let filtered_nums: Vec<i32> = nums.into_iter().filter(|n| *n % 2 == 0).collect();
    println!("filtered_nums: {:?}", filtered_nums); // filtered_nums: [2, 8, 1024]

    let words = "How to build a distribute key value store".split(" ");
    let filtered_words: Vec<&str> = words.filter(|w| w.ends_with("e")).collect();
    // filtered_words: ["distribute", "value", "store"]
    println!("filtered_words: {:?}", filtered_words);
}

fn filter_map() {
    let data = "12.5 w 286\n pie 3.14";
    let data_filtered = data
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok());
    /*
    12.5
    286
    3.14
    */
    for d in data_filtered {
        println!("{}", d);
    }
}

fn flat_map() {
    let mut city_meals: HashMap<&str, Vec<&str>> = HashMap::new();
    city_meals.insert("Chengdu", vec!["Hotpot", "Sweetheart cake"]);
    city_meals.insert(
        "Beijing",
        vec!["Kung Pao Chicken", "Brush meat in copper pot"],
    );

    let city = vec!["Beijing", "Chengdu"];
    let meals = city.iter().flat_map(|city| &city_meals[city]);
    for meal in meals {
        println!("{}", meal);
    }
}

fn flatten() {
    let vectors_of_vector = vec![vec![1, 2, 3], vec![4, 5, 6]];
    vectors_of_vector
        .iter()
        .flatten()
        .for_each(|x| print!(" {} ", x));
    println!();
}

fn take_while() {
    println!("call take_while.......");
    let words: Vec<&str> = "How to build a LSM Tree Based KeyValue Store, like (RocksDB, BadgerDB)"
        .split(" ")
        .collect();
    // if take_while apply to a item returns false, then the iterator stops.
    for word in words.iter().take_while(|w| w.len() <= 10) {
        println!("{}", word);
    }
}

fn skip() {
    let data = vec![3, 2, 5, 6];
    let skipped = data.iter().skip(2);
    println!("skippped...");
    for d in skipped {
        println!("{:?}", d);
    }

    let data1 = vec![3, 2, 5, 6];
    let skipped1 = data1.iter().skip_while(|x| **x % 2 != 0);
    println!("skipped while...");
    for d in skipped1 {
        println!("{:?}", d);
    }
}

fn peek() {
    let mut tokens = "14654532472896435,5676954232543431223".chars().peekable();
    // peek will not advance to next item
    while let Some(tk) = tokens.peek() {
        println!("{:?}", tk);
        tokens.next();
    }
}

fn rev() {
    println!("call iterator rev...");
    let words = vec!["你", "看", "看", "你"];
    let rev_words = words.iter().rev();
    for word in rev_words {
        print!(" {} ", word);
    }

    println!();

    let nums = vec![-1, 0, 10];
    let rev_nums = nums.iter().rev();
    for num in rev_nums {
        print!(" {} ", num); // 10, 0, -1
    }
}

fn chain() {
    let v1 = vec!["A", "list", "of"];
    let v2 = vec!["programming", "languages"];
    let chainned: Vec<&&str> = v1.iter().chain(v2.iter()).collect();
    //         ["A", "list", "of", "programming", "languages"]
    println!("\n\t{:?}", chainned);
}
