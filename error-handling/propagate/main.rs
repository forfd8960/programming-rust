use std::path::Path;
use std::{
    fs,
    io::{self},
};

// error propagate up
fn main() {
    // thread 'main' panicked at 'expect error: "error"', propagate/main.rs:3:22
    // ret_result(true).expect("expect error");

    // println!(
    //     "{:?}",
    //     move_all(Path::new("/foo/bar.txt"), Path::new("/blob"))
    // );

    /*
    call ret result: Err(Custom { kind: Other, error: "..." })
    10
    call ret result: Ok(10)
    */

    println!("call ret result: {:?}", call_ret_result(true));
    println!("call ret result: {:?}", call_ret_result(false));
}

fn call_ret_result(is_err: bool) -> Result<i32, io::Error> {
    let res = ret_result(is_err);
    let v = res?;
    println!("{}", v);
    Ok(v)
}

fn ret_result(is_err: bool) -> Result<i32, io::Error> {
    if is_err {
        Err(io::Error::new(io::ErrorKind::Other, "..."))
    } else {
        Ok(10)
    }
}

fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_res in src.read_dir()? {
        let entry = entry_res?;
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?;
    }

    Ok(())
}
