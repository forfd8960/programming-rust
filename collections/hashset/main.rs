use std::collections::HashSet;

/*
is_empty: false
len: 4
contains Bibo: false
set before remove: {"Bob", "Alice", "John", "Angela"}
remove bob: true
set after remove: {"Alice", "John", "Angela"}
set: None
get Alice name: "Alice"
take John out set: John
set intersection: ["Alice"]
set union: ["Alice", "Benjamin", "Xiaoming", "Alex", "Angela"]
set differ: ["Angela"]
dis joint: false
is_subset: false
is_superset: false
*/
fn main() {
    let mut name_set = HashSet::with_capacity(10);
    name_set.insert("Alice");
    name_set.insert("Bob");
    name_set.insert("John");
    name_set.insert("Angela");

    println!("is_empty: {}", name_set.is_empty());
    println!("len: {}", name_set.len());
    println!("contains Bibo: {}", name_set.contains("Bibo"));

    println!("set before remove: {:?}", name_set);
    println!("remove bob: {}", name_set.remove("Bob"));
    println!("set after remove: {:?}", name_set);
    println!("set: {:?}", name_set.get("Alex"));

    if let Some(name) = name_set.get("Alice") {
        println!("get Alice name: {:?}", *name);
    }

    // remove and return the value
    if let Some(name) = name_set.take("John") {
        println!("take John out set: {}", name);
    }

    let mut name_set1 = HashSet::new();
    for name in vec!["Alex", "Xiaoming", "Benjamin", "Alice"] {
        name_set1.insert(name);
    }

    let intersection = name_set.intersection(&name_set1);
    println!("set intersection: {:?}", intersection);

    let union = name_set.union(&name_set1);
    println!("set union: {:?}", union);

    // name in name_set, but not in name_set1
    let differ = name_set.difference(&name_set1);
    println!("set differ: {:?}", differ);

    println!("dis joint: {:?}", name_set.is_disjoint(&name_set1));
    println!("is_subset: {:?}", name_set.is_subset(&name_set1));
    println!("is_superset: {:?}", name_set.is_superset(&name_set1));
}
