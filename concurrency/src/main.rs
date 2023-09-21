use std::sync::Arc;
use std::thread::JoinHandle;
use std::{io, thread};

#[derive(Debug)]
struct GigabyteMap {
    count: i32,
}

/*
process: file1.txt
process: file2.txt
sucess: ()
files chunk: [["file1.txt", "file2.txt"], ["file3.txt"]]
processing: ["file1.txt", "file2.txt"]
glossary: 0
processing: ["file3.txt"]
glossary: 0
process success
*/
fn main() {
    println!("Rust Concurrency");

    if let Ok(v) = process_fiels_sequential(vec!["file1.txt".to_string(), "file2.txt".to_string()])
    {
        println!("sucess: {:?}", v);
    }

    let glossary = Arc::new(GigabyteMap { count: 0 });

    match process_files_paralle(
        vec![
            "file1.txt".to_string(),
            "file2.txt".to_string(),
            "file3.txt".to_string(),
        ],
        glossary,
    ) {
        Ok(()) => println!("process success"),
        _ => println!("process failed"),
    };
}

fn process_fiels_sequential(names: Vec<String>) -> io::Result<()> {
    for name in names {
        // let text = load(&name)?;
        // let results = process(text);
        // save(&name, results)?;
        println!("process: {}", name);
    }
    Ok(())
}

fn process_files_paralle(names: Vec<String>, glossary: Arc<GigabyteMap>) -> thread::Result<()> {
    const THREADS: usize = 2;
    let work_lists = split_vec_into_chunks(names, THREADS);

    let mut threads_handle: Vec<JoinHandle<_>> = vec![];
    for worklist in work_lists {
        let glossary_child = glossary.clone();
        threads_handle.push(thread::spawn(move || {
            process_files(worklist, &glossary_child)
        }));
    }

    for handle in threads_handle {
        handle.join().unwrap();
    }

    Ok(())
}

fn split_vec_into_chunks(files: Vec<String>, num_thread: usize) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = vec![];
    for file in files.as_slice().chunks(num_thread) {
        result.push(file.to_vec());
    }

    println!("files chunk: {:?}", result);
    result
}

fn process_files(list: Vec<String>, glossary: &GigabyteMap) {
    println!("processing: {:?}", list);
    println!("glossary: {:?}", glossary.count);
}
