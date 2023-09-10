use std::io::Write;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mut buf: Vec<u8> = vec![];
    say_hello(&mut buf)?;
    // let writer: &dyn Write = &mut buf;
    assert_eq!(buf, b"Hello");

    println!("i32 min: {}", min(1, 2));
    println!("str min: {}", min("a", "z"));
    println!("i64 min: {}", min(26 as i64, 18 as i64));
    Ok(())
}

fn say_hello(writer: &mut dyn Write) -> std::io::Result<()> {
    writer.write_all(b"Hello")?;
    writer.flush()
}

fn min<T: Ord>(a: T, b: T) -> T {
    if a <= b {
        a
    } else {
        b
    }
}
