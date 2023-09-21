use std::io;

use rayon::prelude::*;

#[derive(Debug)]
struct GigabyteMap {
    count: i32,
}

/*
processing: "file1.txt"
glossary: 0
processing: "file2.txt"
glossary: 0
processing: "file3.txt"
glossary: 0
process files success
*/
fn main() {
    let files = vec![
        "file1.txt".to_string(),
        "file2.txt".to_string(),
        "file3.txt".to_string(),
    ];

    let glossary = GigabyteMap { count: 0 };
    match process_files_in_parallel(files, &glossary) {
        Ok(()) => println!("process files success"),
        _ => println!("process files failed"),
    };
}

fn process_files_in_parallel(files: Vec<String>, glossary: &GigabyteMap) -> io::Result<()> {
    files
        .par_iter()
        .map(|name| process_file(name, glossary))
        .reduce_with(|r1, r2| if r1.is_err() { r1 } else { r2 })
        .unwrap_or(Ok(()))
}

fn process_file(name: &String, glossary: &GigabyteMap) -> io::Result<()> {
    println!("processing: {:?}", name);
    println!("glossary: {:?}", glossary.count);
    Ok(())
}
