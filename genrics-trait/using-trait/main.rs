use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"Hello")?;
    assert_eq!(buf, b"Hello");

    let mut buf1: Vec<u8> = vec![];

    // A variable’s size has to be known at compile time, and types that implement Write can be any size.
    // but in Rust, references are explicit:
    let writer: &mut dyn Write = &mut buf1;
    writer.write_all(b"Hello\n")?;

    println!("buf: {:?}", buf1);
    assert_eq!(buf1, b"Hello\n");

    let mut file = File::create("hello.txt")?;
    // Rust automatically converts ordinary references into trait objects when needed.
    // This is why we’re able to pass &mut local_file to say_hello in this example:
    say_hello(&mut file)?;
    Ok(())
}

fn say_hello(writer: &mut dyn Write) -> std::io::Result<()> {
    writer.write_all(b"Hello")?;
    writer.flush()
}
