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

### Chapter 3 - Common Programming Concepts

- You can shadow a variable by doing:

  ```rust
  let x = 5;
  let x = x + 1;
  // You can change the data type
  let spaces = "     "
  let spaces = spaces.len()
  ```

- We make a "new" variable that wil take it's spot for any future references.
- When compiling/building with `--release` flag, Rust does not include checks for integer overflow. So, if integer overflow happens, Rust will not panic and silently wrap around.
- Tuple Type: Once declared, cannot grow in size.

  ```rust
  let x: (i32, f64, u8) = (500, 6.4, 1);

  let five_hundred = x.0;

  let six_point_four = x.1;

  let one = x.2;
  ```

- Array Syntax:

  ```rust
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  let a = [3; 5]; // [3, 3, 3, 3, 3]
  ```

- `if` in a `let` Statement:

  ```rust
  let condition = true;
  let number = if condition { 5 } else { 6 };
  ```

- Rust supports `loop {}`, `while {}`, `for _ in (lb, ub+1) {}` or `(lb, ub+1).rev()`.

### Chapter 4 - Ownership

- Ownership Rules
  - Each value in Rust has an owner
  - There can only be one owner at a time
  - When the owner goes out of scope, the value will be dropped.
- `Copy` trait: The variables that use it do not move, but are trivially copied, because their size is known at compile-time. Example:

  ```rust
  let x = 5;
  let y = x;
  println!("x = {x}, y = {y}"); // x = 5, y = 5
  ```

- The same struct/type cannot implement the `Drop` + `Copy` trait.
  - `Copy` is a trivial duplication with no cleanup
  - `Drop` means guaranteed cleanup excatly once
- Theoretically if both traits were implemented:
  - The compiler would generate implicit copies
  - Each copy would run `drop()` when it goes out of scope
  - Multiple destructor calls for the same logical resource.
- Referencing = `&`, Dereferencing = `*`
- The act of creating a reference is called borrowing, a reference cannot modify the value.
  - Need to create a mutable reference, with `&mut var` if you want to modify the underlying value.
  - Similar to write & read locks:
    - You can issue as many read locks as long as no write locks have been issued
    - But once a write lock has been issued, cannot give/acquire a read lock, till write lock has been released.
- When a reference falls out of scope, the value being pointed to, is not dropped because the reference does not own the value.

- Slices are non-owning references to contiguous segment of a collection.
- Example: String Slice

  ```rust
  let s = String::from("hello world");
  let hello = &s[0..5];
  let world = &s[6..11];
  ```

- A slice is internally a pointer + length, representing `[start, end)` in bytes.
- For example:

  ```rust
  fn first_word(s: &String) -> &str {
    for (i, &b) in s.as_bytes().iter().enumerate() {
      if b == b' ' { return &s[0..i];}
    }
    &s[..] // s is one word with no spaces found
  }


  fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear() // compile-time error: cannot mutate while slice exists
  }
  ```

### Chapter 5 - Structs

- Structs vs. Tuples:
  1. Clarity: Fields have names, instead of tuples where you have to do `var_tuple.idx`
  2. Ordering: Structs do not care about ordering during instantiation
  3. Type Specificity: Each struct is it's own type, tuples are only distinguished by shape

- Example:

  ```rust
  #[derive(Debug)]
  struct User {active: bool, username: String, email: String, sign_in_count: u64}
  // Instantiation
  let u1 = User {active: true, username: String::from("u"), email: String::from("e"), sign_in_count: 1};

  // Update (use the existing fields from u1 with ..u1)
  let u2 = User {email: String::from("another@email.com"), ..u1}

  // With the debug trait, we can do the following:
  println!("User is {u2:?}");
  dbg!(&u2);

  // Unit Struct = No data but a useful marker for implementing traits
  struct AlwaysEqual;
  ```

- Ownerships in Structs
  - Using `String` fields ensures that struct owns its data. References require explicit lifetimes.
  - Without lifetimes, the compiler cannot verify if the referenced data remains valid.
  - Here is an example without lifetimes:

  ```rust
  struct User<'a> {
    username: &'a str, 
    email: &'a str,
  }

  fn main() {
    let username = String::from("name");
    let email = String::from("name@gmail.com");
    let u = User{username: &username, email: &email};

    drop(email); // Releases memory that u still points to.
    println!("{}", u.username); // Points to freed memory
  }


  - The `drop(email)` will cause a compile-time error because of the explicit lifetime stated in User with `'a`.

### Chapter 6 - Enums and Pattern Matching

- Structs are `AND` types:
  - A `Rectangle` has a width **and** a height.
- Enums are `OR` types:
  - A `Shape` is a `Circle` or a `Rectangle` or a `Polygon`
- Structs group different data types, Enums distinguish
- Whenver an enum is handled, we need to handle each probable type (usually with match)

  ```rust
  enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
  }

  impl IpAddr {

      fn display(&self) {
        match self {
            IpAddr::V4(a, b, c, d) => println!("Standard IPv4: {}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(s) => println!("Standard IPv6: {}", s),        
          }
      }

      fn is_addr6(&self) -> bool {
        match self {
          IpAddr::V6(_) => true,
          _ => false, // W/O this, compile-time error. Must handle every pattern. 
        }
      }
  }

  fn main() {
    let localhost_v4 = IpAddr::V4(127, 0, 0, 1);
    let localhost_v6 = IpAddr::V6(String::from("::1"));

    localhost_v4.display();
    println!("Is lcaolhost V6? {}", lcaolhost_v6.is_addr6());   // true  
  }
  ```

- Rust doesn't use null, instead it has:

  ```rust
  // Option<T> is a Functor/Monad?
  enum Option<T> { 
    None,    // No Value
    Some(T), // Value Exists
  }
  ```

- `if let` vs `match`:
  ```rust
      match config_max {
        Some(max) => println!("Max is {max}"),
        _ => (),
      }

      // Same functionality, but this does not handle config_max = None case.
      if let Some(max) = config_max {
        println!("Max is {max}");
      }
  ```

### Chapter 7 - Managing Projects with Crates, Packages, and Modules

- TBD

### Chapter 8 - Common Collections

- Define new Vector with `let v: Vec<T> = Vec::new();`
- If we have a reference to any element, we cannot push a new number to the end of the vector. Why?
  - Assuning `let n = v.len();` and `let m = v.capacity();`:
    - If n+1 < m: Pushing a new element to `v` will work fine without affecting the reference.
    - If n+1 >= m: Vector `v` might have to be reallocated to a different contiguous location in memory. **MAKING THE REFERENCE INVALID**. So, Rust's borrow checker does not allow pushing new elements while there is a reference.

### Chapter 9 - Error Handling

- `panic!` is used for unrecoverable errors.
- By default, Rust will unwind the stack, running destructors as it walks up. This is costly, so you can set `panic = "abort"` in `Cargo.toml` (makes binary smaller and skips cleanup)
- Setting `RUST_BACKTRACE=1` shows a backtrace of panic.
- Recoverable Errors

  ```rust
  enum Result<T, E> {
    Ok(T),
    Err(E)
  }


  // How this looks for opening files:
  let f = match File::open("hello.txt") {
    Ok(file) => file,
    Err(error) => panic!("Problem: {error:?}"),
  };

  // If you want to get more specific on the error block:
  let f = match File::open("hello.txt") {
    Ok(file) => file,
    Err(error) => match e.kind() {
            ErrorKind::NotFound => ...,
            _ => panic!("Other error: {e:?}");
    },
  };
  ```

- `unwrap()/unwrap_or_else()/expect()`:
```rust
// WITH Option<T>
let val = Some(3).unwrap();   // 3
let val = None.unwrap();      // panic: "called `Option::unwrap()`..."
let val = None.expect("value missing"); // panic: "value missing"
let val = None.unwrap_or_else(|| 10);  // 10 (NO ARGUMENT TAKEN BECAUSE OPTION DOES NOT HAVE ERROR TYPE)

// WITH Result<T, E>
let x = Ok(3).unwrap();           // 3
let x = Err("err").unwrap();      // panic with generic error message
let x = Err("err").expect("Failed to compute"); // panic: "Failed to compute: err"
let x = Err("err").unwrap_or_else(|e| {
    eprintln!("Got error: {e}");
    10
}); // -> 10   (closure receives the error)
```

- THE `?` OPERATOR
  - With `?` operator, we can propogate any errors caught back to the caller.
  ```
  fn read_username_from_file() -> Result<String, io::Error> {
      let mut f = File::open("hello.txt")?;
      let mut s = String::new();
      f.read_to_string(&mut s)?;
      Ok(s)
  }
  ```
  - You can use `?` on `Result` ONLY IN FUNCTIONS THAT RETURN `Result`
  - You can use `?` on `Option` ONLY IN FUNCTIONS THAT RETURN `Option`
- `Box<dyn Error>` is a catch-all for all kinds of errors.

### Chapter 10 - Generic Types, Traits, and Lifetimes

- TBD

### Chapter 11 - Writing Automated Tests

- TBD

### Chapter 12 - An I/O Project: Building a Command Line Program

- TBD

### Chapter 13 - Functional Language Features: iterators and Closures

- TBD

### Chapter 14 - More about Cargo and Crates.io

- TBD

### Chapter 15 - Smart Pointers

- TBD

### Chapter 16 - Fearles Concurrency

- TBD

### Chapter 17 - Fundamentals of Async Programming

- TBD

### Chapter 18 - Object Oriented Programming

- TBD

### Chapter 19 - Patterns and Matching

- TBD

### Chapter 20 - Advanced Features

- TBD

## Readings

### Article 1: To panic or not to panic [[Link](https://www.ncameron.org/blog/to-panic-or-not-to-panic/)]

- "You cannot, and should not pretend to, write completely panic-free Rust. But you must always design for panics consciously"
- Panics represent programmer bugs or broken invariants or undefined behavior, not runtime recoverable conditions.
  - `Result` for recoverable cases
  - `panic!` or `unwrap()` for impossible logic paths
- Can intercept/catch panics at multiple levels:
  - At thread level, we can use: `std::panic::catch_unwind( potential_panic_func() )`
  - At process level, add `catch_unwind(rest_of_code())`, so any future code called by process is covered.
  - **Q**: What happens to `catch_unwind(...)` when you call fork() later in the code?
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
    - calls destructors for all in-scope objects (**Look into `Drop`**)
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
