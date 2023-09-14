use std::iter::repeat;
use std::str;
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
    enumerate();
    zip();
    build_words();
    by_ref();
    clone();
    cycle();
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

fn enumerate() {
    let vv = vec![0, 1, 2, 3];
    for (idx, v) in vv.iter().enumerate() {
        println!("idx: {}, val: {}", idx, v);
    }
}

fn zip() {
    let v1 = (1..5);
    let v2 = (6..10);
    let zip_v1_v2: Vec<_> = v1.zip(v2).collect();
    // zip_v1_v2: [(1, 6), (2, 7), (3, 8), (4, 9)]
    println!("zip_v1_v2: {:?}", zip_v1_v2);
}

/*
(("with", "one"), "一")
(("with", "two"), "二")
(("with", "three"), "三")
*/
fn build_words() {
    let cn_wd = vec!["一", "二", "三"];
    let en_wd = vec!["one", "two", "three"];
    let rhyme: Vec<_> = repeat("with").zip(en_wd).zip(cn_wd).collect();
    for wd in rhyme {
        println!("{:?}", wd);
    }
}

// An iterator’s by_ref method borrows a mutable reference to the iterator
// so that you can apply adapters to the reference.
// When you’re done consuming items from these adapters,
// you drop them, the borrow ends, and you regain access to your original iterator.
fn by_ref() {
    /*
    Headers...
    header: TO:Alice
    header: From: Bob
    Body...
    Oooooh, a msg for u
    */
    let message = "TO:Alice\r\n\
                        From: Bob\r\n\
                        \r\n\
                        Oooooh, a msg for u\r\n";
    let mut lines = message.lines();

    println!("Headers...");
    for header in lines.by_ref().take_while(|line| !line.is_empty()) {
        println!("header: {}", header);
    }

    println!("Body...");
    for body in lines {
        println!("{}", body);
    }
}

fn clone() {
    let mut data = 1..3;
    let mut cloned_data = data.clone();
    println!("{:?}", data.next()); // 1
    println!("{:?}", cloned_data.next()); // 1
}

// Repeats an iterator endlessly.
fn cycle() {
    let days = vec!["mon", "tue", "wed", "thu", "fri", "sat", "sun"];
    let mut days_repeate = days.iter().cycle();
    for i in 1..10 {
        print!("the {}'s day is: {:?} ", i, days_repeate.next());
    }
    println!();
}
