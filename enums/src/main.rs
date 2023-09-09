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

fn main() {
    let tu = TimeUnit::Hours;
    println!("plural: {}", tu.plural());
    println!("singular: {}", tu.singular());
    println!("order: {:?}", compare(8, 66)); // Less

    let json = Json::Null;
    println!("null: {:?}", json);

    let json1 = Json::Boolean(true);
    println!("null: {:?}", json1);
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
