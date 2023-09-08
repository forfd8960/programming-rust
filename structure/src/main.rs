#[derive(Debug)]
struct Note {
    title: String,
    content: Vec<String>,
    author: String,
    length: u32,
}

impl Note {
    // Type-Associated Functions - new
    fn new(title: String, content: Vec<String>, author: String) -> Self {
        let len = content.len();
        Self {
            title: title,
            content: content,
            author: author,
            length: len as u32,
        }
    }

    fn append_content(&mut self, content: &mut Vec<String>) {
        self.content.append(content);
    }

    fn print(&self) {
        let mut note_str = "title: \n\t".to_string();
        note_str.push_str(self.title.as_str());
        note_str.push_str("\n");
        note_str.push_str("Author: \n\t");
        note_str.push_str(self.author.as_str());
        println!("{}", note_str);
        println!("content({}): \n\t{:?}", self.length, self.content);
    }
}

// tuple struct
#[derive(Debug)]
struct Limits(usize, usize);

#[derive(Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub const DefaultX: f32 = 0.0;
    pub const DefaultY: f32 = 0.0;
}

// A value of such a type occupies no memory
#[derive(Debug)]
struct UnitStruct;

fn main() {
    let content = vec!["A Content".to_string()];
    let mut note = Note::new("A Note".to_string(), content, "Alice".to_string());
    println!("a note: {:?}", note);
    note.print();

    let mut new_content = vec!["new content".to_string()];
    note.append_content(&mut new_content);
    note.print();

    let limits = Limits(1024, 4096);
    println!("{:?}", limits);

    let vector = Vector { x: 1.0, y: 3.6 };
    println!("vector: {:?}", vector);
    println!("DefaultX: {:?}", Vector::DefaultX);
    println!("DefaultY: {:?}", Vector::DefaultY);

    let ustruct = UnitStruct;
    println!("{:?}", ustruct);
}
