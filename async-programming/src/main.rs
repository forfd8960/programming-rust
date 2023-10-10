use async_std::{
    io::{ReadExt, WriteExt},
    net, task,
};

async fn say_hello() {
    println!("Hello, Async Fn");
}

/*

Hello, Async Fn
HTTP/1.1 301 Moved Permanently
Content-Length: 0
Location: https://github.com/


HTTP/1.1 301 Moved Permanently
content-length: 0
location: https://en.wikipedia.org/
server: HAProxy
x-cache: cp5021 int
x-cache-status: int-tls
connection: close*/
fn main() {
    // block_on function blocks until a future's value is ready.
    task::block_on(say_hello());

    let requests = vec![
        ("github.com".to_string(), 80, "/".to_string()),
        ("en.wikipedia.org".to_string(), 80, "/".to_string()),
    ];

    // All this execution takes place on a single thread,
    // the three calls to cheapo_request being interleaved with each other through successive polls of their futures
    let results = task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(resp) => println!("{}", resp),
            Err(e) => eprintln!("error: {}", e),
        }
    }
}

async fn send_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    let mut socket = net::TcpStream::connect((host, port)).await?;
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;

    Ok(response)
}

// spawn_local takes a future and adds it to a pool that block_on will try polling whenever the future it's blocking on isn't ready.
async fn many_requests(requests: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
    let mut handles = vec![];

    for (host, port, path) in requests {
        handles.push(task::spawn_local(send_own_request(host, port, path)));
    }
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }
    results
}

async fn send_own_request(host: String, port: u16, path: String) -> std::io::Result<String> {
    send_request(&host, port, &path).await
}
