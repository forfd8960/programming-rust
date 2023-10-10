use async_std::{
    io::{ReadExt, WriteExt},
    net, task,
};
use std::{future::Future, io};

/*
resp: Ok("HTTP/1.1 301 Moved Permanently\r\nContent-Length: 0\r\nLocation: https://github.com/forfd8960\r\n\r\n")
*/
fn main() {
    let join_handle = task::spawn_local(send_request("github.com", 80, "/forfd8960"));
    let response = task::block_on(join_handle);
    println!("resp: {:?}", response);
}

fn send_request<'a>(
    host: &'a str,
    port: u16,
    path: &'a str,
) -> impl Future<Output = io::Result<String>> + 'a {
    async move {
        let mut socket = net::TcpStream::connect((host, port)).await?;
        let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
        socket.write_all(request.as_bytes()).await?;
        socket.shutdown(net::Shutdown::Write)?;

        let mut response = String::new();
        socket.read_to_string(&mut response).await?;

        Ok(response)
    }
}
