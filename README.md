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
  ```

  - The `drop(email)` will cause a compile-time error because of the explicit lifetime stated in User with `'a`.
  - User needs `'a` lifetime because the struct must not outlive the referenced data.

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

  ```rust
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

- `where` clause (readibility improvement)
  - Meant to visually separate the shape of function & constraints necessary for function to be valid.

  ```rust
  pub fn some_function<T: Display + Clone, U: Clone + Debug>(
      t: &T,
      u: &U,
  ) -> String {
        // ...
      }

  pub fn some_function<T, U>(t: &T, u: &U) -> String
  where
      T: Display + Clone,
      U: Clone + Debug,
      {
        // ...
      }
  ```

- Lifetimes
  - Each reference has a lifetime
  - Borrow Checker validates relationships between lifetimes
  - Lifetimes DO NOT AFFECT RUNTIME
  - IT IS A COMPILER CONSTRUCT

  ```rust
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
  // THE RETURN VALUE MUST BE VALID FOR ATLEAST AS LONG AS BOTH INPUT REFERENCES
  ```

  - Lifetime Epsilon Rules
     1. Each reference parameter gets its own lifetime parameter
     2. If one input lifetime, it is assigned to all output lifetimes
     3. If a method has `&self` or `&mut self`, the lifetime of self is assigned to all output lifetimes.
  - Static Lifetime
    - Refers to data that lives for the entire duration of the program.
    - Data is stored in program's binary.

    ```rust
    let s: &'static str = "I have a static lifetime.";
    ```

  - Combining Generics + Traits + Lifetimes

  ```rust
  fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
  where
    T: Display,
    { ... }
  // T is a generic type parameter that requires implementation of Display
  // Function uses lifetime parameter 'a
  // ann must implement traits specified by T
  // Return reference is bound by 'a, independent of T lifetime.
  ```
  
### Chapter 11 - Writing Automated Tests

- Using `Result<T, E>` in Tests
  - Don't have to return anything, or they can return `Result<T,E>`
  - If test returns `Ok(...)` -> pass, returns `Err(...)` -> fail.
- Force Single Threaded Tests: `cargo test -- --test-threads=1`
- Show Test Output: `cargo test -- --nocapture`
- Unit Tests live in same file as source code
- Integration Tests live at `my_project_root/tests`  

```rust
  // ... source code beguins...
  
  // This is a doc test that rust can compile and run when we do `cargo test`

  /// Adds one.
  /// ```
  /// assert_eq!(my_crate::add_one(1), 2);
  /// ```
  
  
  // ... source code ends ...

  #[cfg(test)]
  mod tests {
    fn test_...() {

    }
  }
  ```

### Chapter 12 - An I/O Project: Building a Command Line Program

- To provide cmd line arguments, you can do -- arg1 arg2
  - `arg1` and `arg2` get ingested by your program instead of `cargo`
- `0sString` can be used for cases where input is invalid Unicode.

### Chapter 13 - Functional Language Features: iterators and Closures

- Closures:
  - Like lambdas from Python/C++
  - Stronger Ownership Semantics + Stricter Type Rules
  - Closures can capture variables in 3 ways:
    - `&T` Immutable Borrow
    - `&mut T` - Mutable Borrow
    - `T` - Take Ownership

  ```rust
  let v = vec![1,2,3];

  let print_v = || println!("{:?}", v); // IMMUTABLE BORROW
  let mut push_v = || v.push(4); // MUTABLE BORROW
  thread::spawn(move || println!("{:?}", v)); // TAKE OWNERSHIP 
  // Last one is required when there are multiple threads, prevents race conditions.
  ```

- Iterators
  - All iterators need to implement the `Iterator` trait:

  ```rust
  fn next(&mut self) -> Option<Self::Item> {}
  ```

  - Rust iterators do nothing until consumed.
  - Categories:
    - Consumers: `sum()`, `collect()`, `for`
    - Adapters : `map`, `filter`, `enumerate`, transform but do not consume data.
  - Adapters will create a pipeline of actions that need to be done when consumed.
  - Iterator chains will perform same/better than hand-written loops.
  - Example:

  ```rust
  v.iter().map(|x| x + 1).filter(|x| x % 2 == 0).collect::<Vec<_>>();
  ```

### Chapter 14 - More about Cargo and Crates.io

- TBD

### Chapter 15 - Smart Pointers

- 2 important traits for Smart Pointers:
  - `Deref`: makes smart pointers act like references, enabling `*`
  - `Drop`: runs custom logic when smart pointer goes out of scope

- Common Smart Pointers in Rust std lib:
  - `Box<T>` - Heap Allocation
  - `Rc<T>` - Reference Counting + Shared Ownership
  - `Ref<T>/RefMut<T>` - Runtime checked borrows enabling interior mutability

- `Box<T>`: Heap Allocation
  - `T` needs to be a type with a known compile-time size.

  - ```rust
    fn main() {
      let b = Box::new(5);
      println!("b = {b}");
    }
    ```

  - b stores a pointer on stack, 5 lives on heap. When dropped heap and pointer is freed.
  - Recursion + `Box<T>`

    - ```rust
      enum List {
        Cons(i32, List),
        Nil,
      }

      // This will not compile, because Rust Compiler will try to figure out size of List and do something like:
      // List::Cons = size_of::<i32> + size_of::<List> which endlessly loops
      // Instead we could do:

      enum List {
        Cons(i32, Box<List>),
        Nil,
      }

      // The size of Box<List> is known, so now
      // List::Cons = size_of::<i32> + size_of::<Box<List>> <- size_of::<pointer>
      ```
  
- `Deref`
  - Needed to make smart pointers behave like references

  - ```rust
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
      fn new(x: T) -> MyBox<T> {
          MyBox(x)
      }
    }

    
    // Deref will return a reference, not the owned value
    impl<T> Deref for MyBox<T> {
      type Target = T;
      fn deref(&self) -> &Self::Target {
          &self.0
          }
    }
    // Then we can do something like *mybox, which internally does: *(mybox.deref())
    // Additionally, deref coercion, will automatically convert `&T` to `&U` if `T: Deref<Target = U>`

    fn hello(name: &str) {
      println!("Hello, {name}!");
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // Finally, for cleanup:
    // We cannot explicitly call the destructor:
    m.drop();

    // Instead, we rely on this to consume the value and force cleanup:
    std::mem::drop(m);
    ```

- `RC<T>` = Reference Counter Smart Pointer (`shared_ptr<T>` C++ equivalent)
  - Rust default ownership is one owner per value, but sometimes you need multiple owners of the same heap value
  - ONLY USE IN SINGLE-THREADED (NOT ATOMIC)
  - CORE PROPERTIES:
    - Internal Value = Strong Count (rc)
    - If rc = 0, data is dropped.
    - If `clone()` called on pointer, rc++
    - If `drop()` called on pointer, rc--
- `RefCell<T>`
  - Allows mutation through an immutable reference
  - Borrow-Checking happens at runtime (failure will cause panic), instead of compile time (failure will cause compile-time error)
  - `RefCell<T>` key functions:
    - `borrow()` -> Immtable, returns `Ref<T>`
    - `borrow_mut()` -> Mutable, returns `RefMut<T>`
  - `Rc<RefCell<T>>` can cause runtime panic, because reference counting allows multiple references and ref cell allows for mutability. So `RefCell<T>` resolves to a mutable reference with `borrow_mut()`, then `Rc<T>` can create multiple mutable references to the same spot in the heap.
  - `Weak<T>`:
    - To prevent cycles, we can define weak refs
    - `Rc<T>` clones will upgrade strong count and denote ownership
    - `Weak<T>` clones will NOT incrememnt strong count and denote non-owning references.
  
### Chapter 16 - Fearles Concurrency

- The Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread.
- Don't communicate between threads, by sharing memory, communicate by channels. Each channel has two parts: transmitter and receiver

  ```rust
  use std::sync::mpsc;
  use std::thread;

  fn main() {
      let (tx, rx) = mpsc::channel();

      thread::spawn(move || {
          let val = String::from("hi");
          tx.send(val).unwrap();
          // Send takes ownership of val
          // So if we try doing something like:
          // println!("Value: {val}")
          // it will not compile
      });

      let received = rx.recv().unwrap();
      println!("Got: {received}");
  }  
  ```

- `recv()` is blocking, `try_recv()` is a non-blocking call that returns `Result<T, E>`
- For Mutexs:

  ```rust
  use std::sync::Mutex;

  fn main() {
      let m = Mutex::new(5);

      {
          let mut num = m.lock().unwrap();  // blocks until lock acquired
          *num = 6;                         // num: MutexGuard<i32> (IMPLEMENTS DEREF)
      }                                     // lock released here

      println!("{:?}", m);
  }
  ```

- `Arc<T>`: Atomic Reference Counting
  - Updates to this pointer are atomic and instantly reflected across threads.

  - ```rust
      use std::sync::{Arc, Mutex};
      use std::thread;

      fn main() {
          let counter = Arc::new(Mutex::new(0));
          let mut handles = vec![];

          for _ in 0..10 {
              let counter = Arc::clone(&counter);     // cheap atomic increment
              let h = thread::spawn(move || {
                  let mut num = counter.lock().unwrap();
                  *num += 1;
              });
              handles.push(h);
          }

          for h in handles {
              h.join().unwrap();
          }

          println!("Result: {}", *counter.lock().unwrap());
      }
    ```

- Rust Concurrency relies on data types implementing the two following marker traits: `Send` and `Sync`
  - `Send`: Can I move a value into a new thread, is that move safe?
  - `Sync`: Is `&T` safe to share across multiple threads at the same time?

### Chapter 17 - Async Programming

- Parallelism: Multiple cores doing work simultaneously. Hardware.
- Concurrency: Switching between multiple tasks logically at once. Software.
- Async Rust is concurrency first.
- Rust Futures are:
  - Lazy: they don’t run until you .await.
  - State machines: the compiler transforms async code into an enum-like type with states.
  - Polled: the runtime repeatedly asks the future “are you ready yet?”
- `async fn` compiles into normal function returning `impl Future<Output = T>`
- `await` suspends future, yields control to the runtime and later. `async` function is a state machine with a "resume here" every time you use `await`.
- Examples:

  ```rust
  async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await?;   // async HTTP GET
    let body = response.text().await?;      // await body download

    // Parse HTML and find <title>
    let doc = Html::parse_document(&body);
    let selector = Selector::parse("title").ok()?;
    let el = doc.select(&selector).next()?;
    Some(el.inner_html())
  }

  async fn fastest_title(url1: &str, url2: &str) -> Option<String> {
    let f1 = page_title(url1);
    let f2 = page_title(url2);

    match trpl::race(f1, f2).await {
        Either::Left(title) => title, // url1
        Either::Right(title) => title, // url2
    }
  }


  let fut = async {
    let x = compute().await;
    x + 1
  };

  // fut is a Future. Not running until awaited:
  let result = fut.await;
  ```

- An async runtime is a queue of futures where each future is polled to see if it is `Ready` or `Pending`.
- Example of Async Tasks:

  ```rust
  let handle = trpl::spawn_task(async {
                      /* work */
                  });
  handle.await.unwrap();
  ```

- Joining Futures:

  ```rust
  let fut1 = async { ... };
  let fut2 = async { ... };

  trpl::join(fut1, fut2).await; // Deterministic Interleaving
  ```

  - Joining Futures:

  ```rust
  trpl::join!(a, b, c); // If # of futures known at compile-time
  // If # of futures unknwon at compile-time
  use std::pin::Pin;
  let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![
    Box::pin(f1), // Pinning prevents a value from being moved in memory once it's been pinned.
    Box::pin(f2),
  ];

  trpl::join_all!(futures);
  ```

- Racing Futures:
  - poll multiple futures and return whoever is `Ready` first, cancel the others: `trpl::race(fut1, fut2).await;`

- Streams = Async Iterators
  - `Iterator::next()` -> Sync, `StreamExt::next()` -> Async
  - Streams are used when data arrives over time
  - Example: 
  ```rust
  use trpl::StreamExt;

  fn main() {
      trpl::run(async {
          let values = [1,2,3,4,5,6,7,8,9,10];
          let iter = values.iter().map(|n| n * 2);
          let mut stream = trpl::stream_from_iter(iter);

          while let Some(value) = stream.next().await {
              println!("The value was: {value}");
          }
      });
  }
  ```

### Chapter 18 - Object Oriented Programming

- Static Dispatch: Compile-Time with generics (`T: Trait`)
- Dynamic Dispatch: Vtable pointer at runtime (`dyn Trait`)
- Example:
```rust

trait Shape {
    fn area(&self) -> f64;
}

// Static Dispatch
fn print_area<T: Shape>(shape: &T) {
    println!("{}", shape.area());
}


// Dynamic Dispatch
fn print_area_dyn(shape: &dyn Shape) {
    println!("{}", shape.area());
}


fn main() {
  // Static Dispatch
  print_area(&Circle { r: 10.0 });
  print_area(&Square { s: 3.0 });
  // Rust will generate two calls internally at compile-time:
  // print_area_for_Circle(...)
  // print_area_for_Square(...)


  // Dynamic Dispatch
  print_area_dyn(Box::new(Circle { r: 10.0 }));
  print_area_dyn(Box::new(Square { s: 3.0 }));
  // In this case, shape is a FAT POINTER
  // FAT POINTERS = (data_ptr, vtable_ptr)
  // Steps for dispatch:
  // 1. load vtable pointer
  // 2. Lookup offset of 'area'
  // 3. jump to function impl
}
```

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
