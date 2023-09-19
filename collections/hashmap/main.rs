use std::collections::HashMap;

fn main() {
    hashmap_op();

    words_count();
}

/*
map1 len:0, cap: 0, isEmpty: true
map1 get abc: None
map1 contains abc: true
map1 contains def: false
map1 len:2, cap: 3, isEmpty: false
after mut: Some(10)
get moive: Some(10)
get moive: Some(99)
*/
fn hashmap_op() {
    let mut map1: HashMap<&str, i32> = HashMap::new();
    let mut map2: HashMap<i32, i32> = HashMap::with_capacity(10);

    println!(
        "map1 len:{}, cap: {}, isEmpty: {}",
        map1.len(),
        map1.capacity(),
        map1.is_empty()
    );

    println!("map1 get abc: {:?}", map1.get("abc"));

    map1.insert("abc", 1);
    println!("map1 contains abc: {}", map1.contains_key("abc"));
    println!("map1 contains def: {}", map1.contains_key("def"));

    map1.insert("def", 2);
    println!(
        "map1 len:{}, cap: {}, isEmpty: {}",
        map1.len(),
        map1.capacity(),
        map1.is_empty()
    );

    if let Some(val) = map1.get_mut("abc") {
        *val = 10;
    }
    println!("after mut: {:?}", map1.get("abc"));

    map1.extend(vec![("moive", 10), ("test", 99)]);
    // get moive: Some(10)
    println!("get moive: {:?}", map1.get("moive"));
    println!("get moive: {:?}", map1.get("test"));

    map1.remove("def");
    // None
    println!("get def: {:?}", map1.get("def"));
}

/*
words count: {"of": 2, "evoke": 2,
"Web,": 2, "via": 2,
"which": 2, "programming": 2,
"user's": 2, "than": 2,
"executed": 2, "language,": 2,
"assembly": 3, "will": 2, "suggests": 2, "this,": 2,
"1950s.": 2, "is": 2,
"WebAssembly": 3, "To": 2,
"concept": 2, "dates": 2, "to": 4,
"named": 2, "bringing": 2,
"language.": 2, "true": 2, "name": 2,
"web": 2, "browser.": 2, "website-user's": 2,
"must": 2, "The": 2, "term": 2, "computer": 2,
"hardware-independent": 2, "the": 6, "a": 3, "where": 2, "by": 2, "it": 2, "—": 2,
"be": 3, "client-side": 2, "more": 2, "assembly-like": 2, "much": 2, "accomplish": 2}
*/
fn words_count() {
    let mut words_map: HashMap<&str, i32> = HashMap::with_capacity(100);
    let words = "WebAssembly is named to evoke the concept of assembly language, 
    a term which dates to the 1950s. 
    The name suggests bringing assembly-like programming to the Web, 
    where it will be executed client-side — by the website-user's computer via the user's web browser. 
    To accomplish this, WebAssembly must be much more hardware-independent than a true assembly language.";

    for wd in words.split_whitespace() {
        let count = words_map.entry(new_wd).or_insert(1);
        *count += 1;
    }

    println!("words count: {:?}", words_map);
}
