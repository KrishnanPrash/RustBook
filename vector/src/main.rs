fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // Two ways to grab elements
    // 1: Using & and []
    let third: &i32 = &v[2];
    // 2: Using get()
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let element = &v[1000] // WILL CAUSE CODE TO PANIC

    // Cannot have mutable and immutable references in the same scope
    // So because we have referece stored in third, cannot do:
    // v.push(6); // This will cause a compile error.

    // Iterating over Values in Vector in Immutable fashion
    for i in &v {
        println!("{i}")
    }

    // Explicitly end the borrow before using v mutably
    drop(third);

    // Iterate over values in Mutable Vector
    for i in &mut v {
        *i += 10;
        println!("{i}")
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
}
