use std::fmt::Debug;
use std::hash::Hash;
use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
    let mut buf: Vec<u8> = vec![];
    say_hello(&mut buf)?; // say_hello::<Vec<u8>>

    assert_eq!(buf, b"Hello");
    println!("buf: {:?}", buf);

    let mut file = File::create("hello.txt")?;
    say_hello(&mut file)?; // say_hello::<File>

    Ok(())
}

// generic function
fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"Hello")?;
    out.flush()
}

// type parameter
fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {}
