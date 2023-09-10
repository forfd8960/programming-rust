## When to use which

**When to use Trait Object**

- Trait objects are the right choice whenever you need a collection of values of mixed types.

- Another possible reason to use trait objects is to reduce the total amount of compiled code

```rust
trait Vegetable {

}

// Each Box<dyn Vegetable> can own any type of vegetable, but the box itself has a constant size—two pointers—suitable for storing in a vector.
struct Salad<T: Vegetable> {
    veggies: Vec<Box<dyn Vegetable>>
}

```

**When to use Generic**

- The first advantage is speed. Note the absence of the dyn keyword in generic function signatures. Because you specify the types at compile time, either explicitly or through type inference, the compiler knows exactly which write method to call.

**Compare that to the behavior with trait objects. Rust never knows what type of value a trait object points to until run time**

- The second advantage of generics is that not every trait can support trait objects.

- The third advantage of generics is that it’s easy to bound a generic type parameter with several traits at once.

```rust
use std::fmt::Debug;
use std::hash::Hash;


fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {}
```
