use std::io;

fn main() {
    println!("Hello, world!");
    println!("{}", div(100, 2));

    /*
    thread 'main' panicked at 'attempt to divide by zero', main.rs:8:5
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */
    // println!("{}", div(100, 0));

    let arr = vec![1, 2, 6, 9];
    println!("{}", access_arr(&arr, 0)); // 1

    // thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 6', main.rs:21:5
    // println!("{}", access_arr(&arr, 6)); // panic

    println!("n: {:?}", ret_result(2));
    // thread 'main' panicked at ': Custom { kind: Other, error: "oh bad num" }', main.rs:20:39
    // println!("n: {:?}", ret_result(3).expect(""));
}

fn div(total: i32, divider: i32) -> i32 {
    total / divider
}

fn access_arr(arr: &[i32], idx: usize) -> i32 {
    arr[idx]
}

fn ret_result(n: i32) -> Result<i32, io::Error> {
    if n % 2 == 0 {
        return Ok(n);
    }

    Err(io::Error::new(io::ErrorKind::Other, "oh bad num"))
}
