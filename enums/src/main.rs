use core::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_end_matches("s")
    }
}

#[derive(Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),

    // A HashMap is larger still. If we had to leave room for it in every Json value, they would be quite large,
    // But a Box<HashMap> is a single word: it’s just a pointer to heap-allocated data
    Object(Box<HashMap<String, Json>>),
}

// Generic Enum

#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T> TreeNode<T> {
    fn new(element: T, left: BinaryTree<T>, right: BinaryTree<T>) -> Self {
        Self {
            element: element,
            left: left,
            right: right,
        }
    }
}

fn main() {
    let tu = TimeUnit::Hours;
    println!("plural: {}", tu.plural());
    println!("singular: {}", tu.singular());
    println!("order: {:?}", compare(8, 66)); // Less

    let json = Json::Null;
    println!("null: {:?}", json);

    let json1 = Json::Boolean(true);
    println!("null: {:?}", json1);

    let bin_tree = BinaryTree::<String>::Empty;
    println!("{:?}", bin_tree); // Empty

    let left = BinaryTree::<String>::Empty;
    let right = BinaryTree::<String>::Empty;
    let tree_node = TreeNode::<String>::new("A".to_string(), left, right);
    let bin_tree: BinaryTree<String> = BinaryTree::<String>::NonEmpty(Box::new(tree_node));

    // NonEmpty: NonEmpty(TreeNode { element: "A", left: Empty, right: Empty })
    println!("NonEmpty: {:?}", bin_tree);
}

fn compare(a: i32, b: i32) -> Ordering {
    if a > b {
        Ordering::Greater
    } else if a < b {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}
