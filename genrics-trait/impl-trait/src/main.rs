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

fn main() {
    let words = vec!["a", "B", "C"];
    let my_set = MySet::from_slice(words.as_slice());
    println!("{:?}", my_set);
    println!("B exist: {:?}", my_set.contains("B")); // true

    println!("is emoji: {}", '$'.is_emoji()); // true
}
