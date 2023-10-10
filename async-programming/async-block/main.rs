use async_std::{io, net, task};

fn main() {
    // let serve_one = async {
    //     let listener = net::TcpListener::bind("localhost:8087").await?;
    //     let (mut socket, addr) = listener.accept().await?;
    //     Ok(())
    // };

    /*
    pls input text>>
    abc
    read line: abc

    read text result: Ok(())
        */
    let input = io::stdin();
    let read_text = async {
        let mut line = String::new();
        println!("pls input text>>");
        input.read_line(&mut line).await?;
        println!("read line: {}", line);
        Ok::<(), std::io::Error>(())
    };

    let res = task::block_on(read_text);
    println!("read text result: {:?}", res);
}
