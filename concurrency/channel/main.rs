use std::{
    fs, io,
    path::PathBuf,
    sync::mpsc,
    thread::{self, JoinHandle},
};

struct InMemoryIndex {
    // word to doc IDs
    doc_id: usize,
    text: String,
    // idx: HashMap<String, Vec<usize>>,
}

impl InMemoryIndex {
    fn from_single_document(doc_id: usize, text: String) -> Self {
        Self {
            doc_id: doc_id,
            text: text,
        }
    }
}

fn main() {
    let docs = vec![PathBuf::from("file1.txt")];
    match build_index(docs, PathBuf::from("result.txt")) {
        Ok(()) => println!("indexing success"),
        _ => println!("indexing failed"),
    }
}

fn build_index(documents: Vec<PathBuf>, output_dir: PathBuf) -> io::Result<()> {
    // let documents = vec!["file.txt", "file1.txt"];

    let (texts, file_read_handle) = start_file_read_thread(documents);
    let (indexes, index_handle) = start_file_indexing_thread(texts);
    let r1 = file_read_handle.join().unwrap();
    index_handle.join().unwrap();

    r1?;
    Ok(())
}

fn start_file_read_thread(
    documents: Vec<PathBuf>,
) -> (mpsc::Receiver<String>, thread::JoinHandle<io::Result<()>>) {
    // The channel function returns a pair of values: a sender and a receiver
    let (sender, receiver) = mpsc::channel();

    let handle: JoinHandle<io::Result<()>> = thread::spawn(move || {
        for filename in documents {
            let text = fs::read_to_string(filename)?;

            // sender.send(text) moves the value text into the channel
            if sender.send(text).is_err() {
                break;
            }
        }
        Ok(())
    });

    (receiver, handle)
}

fn start_file_indexing_thread(
    texts: mpsc::Receiver<String>,
) -> (mpsc::Receiver<InMemoryIndex>, thread::JoinHandle<()>) {
    let (sender, receiver) = mpsc::channel::<InMemoryIndex>();

    let handle: JoinHandle<()> = thread::spawn(move || {
        for (doc_id, text) in texts.into_iter().enumerate() {
            let index = InMemoryIndex::from_single_document(doc_id, text);
            if sender.send(index).is_err() {
                break;
            }
        }
    });

    (receiver, handle)
}

fn parse_text(text: String) {
    println!("received string: {}", text);
}
