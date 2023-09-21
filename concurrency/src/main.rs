use std::thread::JoinHandle;
use std::{io, thread};

fn main() {
    println!("Hello, world!");
    match process_files_paralle(vec!["file1.txt".to_string(), "file2.txt".to_string()]) {
        Ok(()) => println!("process success"),
        _ => println!("process failed"),
    };
}

fn process_fiels_sequential(names: Vec<String>) -> io::Result<()> {
    for name in names {
        // let text = load(&name)?;
        // let results = process(text);
        // save(&name, results)?;
    }
    Ok(())
}

fn process_files_paralle(names: Vec<String>) -> io::Result<()> {
    const THREADS: usize = 8;
    let work_lists = split_vec_into_chunks(names, THREADS);

    let mut threads_handle: Vec<JoinHandle<_>> = vec![];
    for worklist in work_lists {
        threads_handle.push(thread::spawn(move || process_files(worklist)));
    }

    for handle in threads_handle {
        handle.join().unwrap();
    }

    Ok(())
}

fn split_vec_into_chunks(names: Vec<String>, num_thread: usize) -> Vec<Vec<String>> {
    vec![]
}

fn process_files(list: Vec<String>) {
    println!("processing: {:?}", list);
}
