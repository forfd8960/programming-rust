use std::{collections::HashMap, str::FromStr};

fn main() {
    let text = " A Awesome\n day\n  beautiful sunny\n day\n".to_string();
    trim_strings(text);
    filter_element();
    filter_map();
    flat_map();
    flatten();
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
