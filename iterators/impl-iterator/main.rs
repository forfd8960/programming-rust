struct I32Range {
    start: i32,
    end: i32,
}

impl Iterator for I32Range {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let result = self.start;
        self.start += 1;
        Some(result)
    }
}

fn main() {
    let my_range = I32Range { start: 0, end: 10 };
    for data in my_range {
        print!("{} ", data);
    }
    println!()
}
