fn main() {
    let fib_iter = fibonacci();
    let fibs = fib_iter.into_iter().take(5).collect::<Vec<usize>>();

    // [1, 1, 2, 3, 5]
    println!("{:?}", fibs);
}

fn fibonacci() -> impl IntoIterator<Item = usize> {
    let mut state = (0, 1);
    std::iter::from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}
