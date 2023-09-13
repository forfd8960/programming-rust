# Rust Iterators

## The Iterator and IntoIterator traits, which are the foundation of Rust’s iterators.

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    ... // many default methods
}
```

- An iterator is any value that implements the std::iter::Iterator trait:
- `Item` is the type of value the iterator produces.
- The next method either returns `Some(v)`, where v is the iterator’s next value, or returns None to indicate the end of the sequence

## The three stages of a typical iterator pipeline: creating an iterator from some sort of value source; adapting one sort of iterator into another by selecting or processing values as they go by; and then consuming the values the iterator produces.

## How to implement iterators for your own types
