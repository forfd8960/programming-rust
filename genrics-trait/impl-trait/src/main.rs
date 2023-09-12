use std::collections::HashSet;

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        match self {
            '@' | '#' | '$' => true,
            _ => false,
        }
    }
}

// Type Associated Function
trait StringSet<'a> {
    fn new() -> Self;
    fn from_slice(strings: &'a [&str]) -> Self;
    fn contains(&self, c: &str) -> bool;
    fn add(&mut self, string: &'a str);
}

fn unknow_words<'a, S: StringSet<'a>>(document: &'a [String], word_list: &S) -> S {
    let mut unknows = S::new();
    for word in document {
        if !word_list.contains(word) {
            unknows.add(word);
        }
    }
    unknows
}

#[derive(Debug)]
struct MySet<'a> {
    set: HashSet<&'a str>,
}

impl<'a> StringSet<'a> for MySet<'a> {
    fn new() -> Self {
        Self {
            set: HashSet::<&str>::new(),
        }
    }

    fn add(&mut self, string: &'a str) {
        self.set.insert(string);
    }

    fn contains(&self, word: &str) -> bool {
        self.set.contains(word)
    }

    fn from_slice(strings: &'a [&str]) -> Self {
        let mut my_set = Self {
            set: HashSet::<&str>::new(),
        };

        for ss in strings {
            my_set.add(ss);
        }

        my_set
    }
}

fn cyclical_zip(v1: Vec<String>, v2: Vec<String>) -> Box<dyn Iterator<Item = String>> {
    Box::new(v1.into_iter().chain(v2).cycle())
}

fn cyclical_zip1(v1: Vec<String>, v2: Vec<String>) -> impl Iterator<Item = String> {
    v1.into_iter().chain(v2).cycle()
}

fn main() {
    println!("is emoji: {}", '$'.is_emoji()); // true

    let words = vec!["a", "B", "C"];
    let my_set = MySet::from_slice(words.as_slice());
    println!("{:?}", my_set);
    println!("B exist: {:?}", my_set.contains("B")); // true

    let v1 = vec!["abc".to_string(), "def".to_string()];
    let v2 = vec!["abc".to_string(), "def".to_string()];
    let mut cycle_vec = cyclical_zip(v1, v2);
    println!("{:?}", cycle_vec.next());

    let v3 = vec!["abc".to_string(), "def".to_string()];
    let v4 = vec!["abc".to_string(), "def".to_string()];
    let mut cycle_vec1 = cyclical_zip1(v3, v4);

    // Some("abc")
    // Some("def")
    println!("{:?}", cycle_vec1.next());
    println!("{:?}", cycle_vec1.next());
}
