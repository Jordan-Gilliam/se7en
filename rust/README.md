# Notes

### cargo

> Cargo is Rust’s build system and package manager
>
> Cargo expects your source files to live inside the _src_ directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. There’s a place for everything, and everything is in its place.

**usefull cargo commands**

`cargo new [project_name]`

`cargo build`

> This command creates an executable file in _target/debug/[project_name_

- You can run the debug executable using `./`
- example: `./target/debug/exe`

`cargo run`

> We just built a project with `cargo build` and ran it with `./target/debug/hello_cargo`, but we can also use `cargo run` to compile the code and then run the resulting executable all in one command:

✅ \$$$profit$$\$

`cargo check`

> **Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:**

- Why would you not want an executable? Often, `cargo check` is much faster than `cargo build`, because it skips the step of producing an executable. If you’re continually checking your work while writing the code, using `cargo check` will speed up the process! As such, many Rustaceans run `cargo check` periodically as they write their program to make sure it compiles. Then they run `cargo build` when they’re ready to use the executable.

`cargo build --release`

> This command will create an executable in _target/release_ instead of _target/debug_. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run `cargo build --release` and benchmark with the executable in _target/release_.

---

## Syntax

### variables

> In Rust, variables are immutable by default.

`let mut food = String::new();`

- creates a mutable variable called **food**
- sets **food** variable to a **new string instance** [UTF-8 encoded bit of text]

- **The `::` syntax** in the `::new` line indicates that `new` is an _associated function_ of the `String` type. An associated function is implemented on a type, in this case `String`, rather than on a particular instance of a `String`. Some languages call this a **_static method_.**

### println

> `println!` calls a Rust macro. If it called a function instead, it would be entered as `println` (without the `!`). using a `!` means that you’re calling a macro instead of a normal function.

### std io lib

#### std::io;

Or

#### std::io::stdin

or **std::io::stdin** inline **without importing** std lib

> By default, Rust brings only a few types into the scope of every program in [the _prelude_](https://doc.rust-lang.org/std/prelude/index.html). If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a `use` statement. Using the `std::io` library provides you with a number of useful features, including the ability to accept user input.

## Quick Ref

### Comments

````rust
// This is a comment. Line comments look like this...
// and extend multiple lines like this.

/// Documentation comments look like this and support markdown notation.
/// # Examples
///
/// ```
/// let five = 5
/// ```
````

### Functions

```rust
// Functions
// `i32` is the type for 32-bit signed integers
fn add2(x: i32, y: i32) -> i32 {
    // Implicit return (no semicolon)
    x + y
}
```

### Basics

```rust

///////////////
// 1. Basics //
///////////////

#[allow(dead_code)]
// Functions
// `i32` is the type for 32-bit signed integers
fn add2(x: i32, y: i32) -> i32 {
    // Implicit return (no semicolon)
    x + y
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
// Main function
fn main() {
    // Numbers //

    // Immutable bindings
    let x: i32 = 1;

    // Integer/float suffixes
    let y: i32 = 13i32;
    let f: f64 = 1.3f64;

    // Type inference
    // Most of the time, the Rust compiler can infer what type a variable is, so
    // you don’t have to write an explicit type annotation.
    // Throughout this tutorial, types are explicitly annotated in many places,
    // but only for demonstrative purposes. Type inference can handle this for
    // you most of the time.
    let implicit_x = 1;
    let implicit_f = 1.3;

    // Arithmetic
    let sum = x + y + 13;

    // Mutable variable
    let mut mutable = 1;
    mutable = 4;
    mutable += 2;

    // Strings //

    // String literals
    let x: &str = "hello world!";

    // Printing
    println!("{} {}", f, x); // 1.3 hello world

    // A `String` – a heap-allocated string
    let s: String = "hello world".to_string();

    // A string slice – an immutable view into another string
    // The string buffer can be statically allocated like in a string literal
    // or contained in another object (in this case, `s`)
    let s_slice: &str = &s;

    println!("{} {}", s, s_slice); // hello world hello world

    // Vectors/arrays //

    // A fixed-size array
    let four_ints: [i32; 4] = [1, 2, 3, 4];

    // A dynamic array (vector)
    let mut vector: Vec<i32> = vec![1, 2, 3, 4];
    vector.push(5);

    // A slice – an immutable view into a vector or array
    // This is much like a string slice, but for vectors
    let slice: &[i32] = &vector;

    // Use `{:?}` to print something debug-style
    println!("{:?} {:?}", vector, slice); // [1, 2, 3, 4, 5] [1, 2, 3, 4, 5]

    // Tuples //

    // A tuple is a fixed-size set of values of possibly different types
    let x: (i32, &str, f64) = (1, "hello", 3.4);

    // Destructuring `let`
    let (a, b, c) = x;
    println!("{} {} {}", a, b, c); // 1 hello 3.4

    // Indexing
    println!("{}", x.1); // hello

    //////////////
    // 2. Types //
    //////////////

    // Struct
    struct Point {
        x: i32,
        y: i32,
    }

    let origin: Point = Point { x: 0, y: 0 };

    // A struct with unnamed fields, called a ‘tuple struct’
    struct Point2(i32, i32);

    let origin2 = Point2(0, 0);

    // Basic C-like enum
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    let up = Direction::Up;

    // Enum with fields
    enum OptionalI32 {
        AnI32(i32),
        Nothing,
    }

    let two: OptionalI32 = OptionalI32::AnI32(2);
    let nothing = OptionalI32::Nothing;

    // Generics //

    struct Foo<T> { bar: T }

    // This is defined in the standard library as `Option`
    enum Optional<T> {
        SomeVal(T),
        NoVal,
    }

    // Methods //

    impl<T> Foo<T> {
        // Methods take an explicit `self` parameter
        fn bar(&self) -> &T { // self is borrowed
            &self.bar
        }
        fn bar_mut(&mut self) -> &mut T { // self is mutably borrowed
            &mut self.bar
        }
        fn into_bar(self) -> T { // here self is consumed
            self.bar
        }
    }

    let a_foo = Foo { bar: 1 };
    println!("{}", a_foo.bar()); // 1

    // Traits (known as interfaces or typeclasses in other languages) //

    trait Frobnicate<T> {
        fn frobnicate(self) -> Option<T>;
    }

    impl<T> Frobnicate<T> for Foo<T> {
        fn frobnicate(self) -> Option<T> {
            Some(self.bar)
        }
    }

    let another_foo = Foo { bar: 1 };
    println!("{:?}", another_foo.frobnicate()); // Some(1)

    /////////////////////////
    // 3. Pattern matching //
    /////////////////////////

    let foo = OptionalI32::AnI32(1);
    match foo {
        OptionalI32::AnI32(n) => println!("it’s an i32: {}", n),
        OptionalI32::Nothing  => println!("it’s nothing!"),
    }

    // Advanced pattern matching
    struct FooBar { x: i32, y: OptionalI32 }
    let bar = FooBar { x: 15, y: OptionalI32::AnI32(32) };

    match bar {
        FooBar { x: 0, y: OptionalI32::AnI32(0) } =>
            println!("The numbers are zero!"),
        FooBar { x: n, y: OptionalI32::AnI32(m) } if n == m =>
            println!("The numbers are the same"),
        FooBar { x: n, y: OptionalI32::AnI32(m) } =>
            println!("Different numbers: {} {}", n, m),
        FooBar { x: _, y: OptionalI32::Nothing } =>
            println!("The second number is Nothing!"),
    }

    /////////////////////
    // 4. Control flow //
    /////////////////////

    // `for` loops/iteration
    let array = [1, 2, 3];
    for i in array.iter() {
        println!("{}", i);
    }

    // Ranges
    for i in 0u32..10 {
        print!("{} ", i);
    }
    println!("");
    // prints `0 1 2 3 4 5 6 7 8 9 `

    // `if`
    if 1 == 1 {
        println!("Maths is working!");
    } else {
        println!("Oh no...");
    }

    // `if` as expression
    let value = if true {
        "good"
    } else {
        "bad"
    };

    // `while` loop
    while 1 == 1 {
        println!("The universe is operating normally.");
        // break statement gets out of the while loop.
        //  It avoids useless iterations.
        break
    }

    // Infinite loop
    loop {
        println!("Hello!");
        // break statement gets out of the loop
        break
    }

    /////////////////////////////////
    // 5. Memory safety & pointers //
    /////////////////////////////////

    // Owned pointer – only one thing can ‘own’ this pointer at a time
    // This means that when the `Box` leaves its scope, it can be automatically deallocated safely.
    let mut mine: Box<i32> = Box::new(3);
    *mine = 5; // dereference
    // Here, `now_its_mine` takes ownership of `mine`. In other words, `mine` is moved.
    let mut now_its_mine = mine;
    *now_its_mine += 2;

    println!("{}", now_its_mine); // 7
    // println!("{}", mine); // this would not compile because `now_its_mine` now owns the pointer

    // Reference – an immutable pointer that refers to other data
    // When a reference is taken to a value, we say that the value has been ‘borrowed’.
    // While a value is borrowed immutably, it cannot be mutated or moved.
    // A borrow is active until the last use of the borrowing variable.
    let mut var = 4;
    var = 3;
    let ref_var: &i32 = &var;

    println!("{}", var); // Unlike `mine`, `var` can still be used
    println!("{}", *ref_var);
    // var = 5; // this would not compile because `var` is borrowed
    // *ref_var = 6; // this would not either, because `ref_var` is an immutable reference
    ref_var; // no-op, but counts as a use and keeps the borrow active
    var = 2; // ref_var is no longer used after the line above, so the borrow has ended

    // Mutable reference
    // While a value is mutably borrowed, it cannot be accessed at all.
    let mut var2 = 4;
    let ref_var2: &mut i32 = &mut var2;
    *ref_var2 += 2;         // '*' is used to point to the mutably borrowed var2

    println!("{}", *ref_var2); // 6 , // var2 would not compile.
    // ref_var2 is of type &mut i32, so stores a reference to an i32, not the value.
    // var2 = 2; // this would not compile because `var2` is borrowed.
    ref_var2; // no-op, but counts as a use and keeps the borrow active until here
}
```
