## Async Programming

### spawn_local

```rust
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
```

- `spawn_local` takes a future and adds it to a pool that block_on will try polling whenever the future it's blocking on isn't ready.
- **asynchronous tasks and threads is that switching from one async task to another happens only at await expressions.**

### Async Block

```rust
let serve_one = async {
    let listener = net::TcpListener::bind("localhost:8087").await?;
    let (mut socket, addr) = listener.accept().await?;
    Ok(())
};
```

- you can start the block with `async move` to take ownership of the captured values, rather than just holding references to them.

```rust
pub async fn many_requests(requests: Vec<(String, u16, String)>)
                           -> Vec<std::io::Result<String>>
{
    use async_std::task;

    let mut handles = vec![];
    for (host, port, path) in requests {
        handles.push(task::spawn_local(async move {
            cheapo_request(&host, port, &path).await
        }));
    }
    ...
}
```

- Async blocks provide a concise way to separate out a section of code you’d like to run asynchronously.

```rust
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
```
