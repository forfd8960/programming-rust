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
