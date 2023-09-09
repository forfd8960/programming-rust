use core::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
struct WordsCounter {
    data: HashMap<String, u32>,
}

impl WordsCounter {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn put_word(&mut self, word: String) {
        if let Some(count) = self.data.get_mut(&word) {
            *count += 1
        } else {
            self.data.insert(word, 1);
        }
    }

    fn get_count(&self, word: String) -> u32 {
        let count = self.data.get(&word);
        match count {
            Some(n) => *n,
            None => 0,
        }
    }
}

#[derive(Debug)]
struct Blog {
    title: String,
    content: String,
    word_counter: RefCell<WordsCounter>,
}

impl Blog {
    fn new(title: String, content: String, counter: WordsCounter) -> Self {
        Self {
            title: title,
            content: content,
            word_counter: RefCell::new(counter),
        }
    }

    fn update(&mut self, content: String) {
        self.content = content.clone();
        let mut counter = self.word_counter.borrow_mut();
        for word in self.content.split(" ") {
            counter.put_word(word.to_string())
        }
    }

    fn get_frequent(&self, word: String) -> u32 {
        let counter = self.word_counter.borrow();
        counter.get_count(word)
    }
}

fn main() {
    let words_counter = WordsCounter::new();
    let mut blog = Blog::new(
        "A Beautiful Day".to_string(),
        "A Awesome and A Nice day".to_string(),
        words_counter,
    );

    println!("{:?}", blog);
    blog.update("A Awesome and A Very Nice day".to_string());

    println!("A count: {}", blog.get_frequent("A".to_string()));
}
