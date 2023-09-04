## Reference

- HashMap is not Copy—it can’t be, since it owns a dynamically allocated table.
- So when the program calls show(table), the whole structure gets moved to the function,
- Leaving the variable table uninitialized.

- A reference lets you access a value without affecting its ownership
- If you have a mutable reference to a value, you may both read and modify the value.
- However, you may not have any other references of any sort to that value active at the same time

**You can think of the distinction between shared and mutable references as a way to enforce a multiple readers or single writer rule at compile time**

**As long as there are shared references to a value, not even its owner can modify it; the value is locked down. Nobody can modify table while show is working with it. Similarly, if there is a mutable reference to a value, it has exclusive access to the value; you can’t use the owner at all, until the mutable reference goes away**
