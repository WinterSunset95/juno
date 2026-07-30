I write random

- `Result<(), Box<dyn std::error::Error>>`: Rust forces us to handle errors explicitly using the `Result` enum. A `Result<T, E>` is basically a box that can contain one of two things:
    - `Ok(T)`: The operation succeeded, with the data type of the return value being `T`.
    - `Err(E)`: The operation failed with the error of type `E`.
    
    So for this specific return type:
    - `()`: This is a "unit type". Rust's version of `void`.
    - `dyn std::error::Error`: `dyn` stands for dynamic. Because our `main` function does multiple things, it can fail in a dozen different ways. We do not want to specify just one error type. `dyn Error` means we can return any error type that implements the standard `Error` trait.
    - `Box<...>`: Because we are returning a _dynamic_ error, the compiler doesn't know how many bytes it takes up at compile time. In Rust, we can't return unsized things on the stack, so we use `Box`, which is technically a smart pointer that allocates the error on the heap.
- The `?` operator: The golang version of this would be
    ```go
    loop, err := EventLoop.try_new()
    if err != nil {
        return err
    }
    ```
    It evaluates the `Result` returned by the statement. If the result is `Ok()`, it unwraps the value and assigns it to the correct variable. If the result is `Err(e)`, the `?` operator instantly aborts the current function and returns that error back to the caller.
- Borrow Checker `&`: At the core of Rust's ownership model, every piece of data has exactly one owner. We can also think of `&` as "pass by reference", except the "value" that the referencer receives is read-only/immutable.
- Mutable Borrow Checker `&mut`: Exactly as the name suggests. It's a "pass by reference" or "borrow" where the "borrower" can actually mutate the original data.
- Closures `| ... | { }`: This is rust's syntax for an *anonymous function* (also known as lambdas or closures).
    In typescript, we write:    `(event, metadata, state) => { ... }`
    In rust, we write:          `|event, metadata, state| { ... }`
