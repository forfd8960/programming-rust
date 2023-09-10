## Trait Object

**A reference to a trait type, like writer, is called a trait object**. Like any other reference, a trait object points to some value, it has a lifetime, and it can be either mut or shared.

What makes a trait object different is that **Rust usually doesn’t know the type of the referent at compile time**

```rust
let mut file = File::create("hello.txt")?;

// Rust automatically converts ordinary references into trait objects when needed.
// This is why we’re able to pass &mut local_file to say_hello in this example:
say_hello(&mut file)?;
```
