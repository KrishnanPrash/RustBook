# RustBook
Working through examples in the Rust Book

## Notes

### Chapter 1

- `Cargo.lock` keeps track of all your dependencies for your rust project. Never have to manually alter.
- Time taken for `cargo build` >>> Time taken for `cargo check`
  - `build` produces an executable at the end.
  - `check` is for iterative development to check for compilation.

### Chapter 2

- Set of items that are brought into every rust program is called the prelude [[Link](https://doc.rust-lang.org/std/prelude/index.html#prelude-contents)]
- `Cargo.toml` dependencies use semantic versioning, which means:  
  - > 0.8.5 is actually shorthand for ^0.8.5, which means any version that is at least 0.8.5 but below 0.9.0.
- After building for the first time, a frozen set of versions is written to `Cargo.lock`, so on future fresh builds, if `Cargo.lock` is present we get a reproducible build.
- `cargo update` will ignore `Cargo.lock` and find the most updated packages that are in complaince with your `Cargo.toml`

## Readings

### Article 1: To panic or not to panic [[Link](https://www.ncameron.org/blog/to-panic-or-not-to-panic/)]

- "You cannot, and should not pretend to, write completely panic-free Rust. But you must always design for panics consciously"
- Panics represent programmer bugs or broken invariants or undefined behavior, not runtime recoverable conditions.
  - `Result` for recoverable cases
  - `panic!` or `unwrap()` for impossible logic paths
- Can intercept/catch panics at multiple levels:
  - At thread level, we can use: `std::panic::catch_unwind( potential_panic_func() )`
  - At process level, add `catch_unwind(rest_of_code())`, so any future code called by process is covered.
  - __Q__: What happens to `catch_unwind(...)` when you call fork() later in the code?
    - The Parent Process and Child Process are completely isolated. So, the child will have it's own version of the panic and the child process crashing/panicing will never make it's way back to the parent `catch_unwind()`
- Example of panic catching code:

```rust
use std::panic;

fn start_application() {
    println!("Application initializing...");

    // Simulate subsystem that may panic
    perform_critical_task();

    println!("Application shutdown cleanly.");
}

fn perform_critical_task() {
    // Unexpected internal bug
    panic!("Subsystem failure detected!");
}

fn main() {
    // Process-level panic guard: top-level boundary
    let result = panic::catch_unwind(|| {
        start_application();
    });

    match result {
        Ok(_) => println!("Program exited normally."),
        Err(_) => {
            eprintln!("Panic captured at process level — shutting down gracefully.");
            // Optional recovery actions (log, cleanup, restart logic)
        }
    }

    println!("Main process recovered — exiting cleanly.");
}
```

- In `cargo.toml` we can define two different panic behaviors. `panic = unwind` or `panic = abort`
  - `panic = unwind`:
    - iteratively unwinds stack
    - calls destructors for all in-scope objects (__Look into `Drop`__)
    - Optionally lets upper layers catch panic via: `std::panic::catch_unwind()`
    - Leads to larger binaries (because we include unwinding tables and destructors)
  - `panic = abort`:
    - catastrophic failure
    - process terminates immediately
    - `catch_unwind()` will not work, even if included
    - Ideally used, when process is monitored externally (eg: Docker, Kubernetes)
    - Smaller Binaries

- Future Reading:
  - [[Panic Recovery in Rust-based
Embedded Systems](https://plos-workshop.org/2023/docs/slides/zhong.pdf)]
    - Q: What are Landing Pads in the context of unwinding panics.
  - [[Panics in Rust and How to Track Them](https://blog.aheymans.xyz/post/don_t_panic_rust/)]
    - Q: How does the linker fit in the context of panics.
