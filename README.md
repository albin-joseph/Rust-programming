# Rust
*Rust programming language basics*

🦀 **What is Rust Programming?**
Rust is a modern, open-source, systems programming language focused on:

- Performance (as fast as C/C++)
- Safety (especially memory safety without needing a garbage collector)
- Concurrency (built to avoid data races)
- It was created by Graydon Hoare and sponsored by Mozilla.

**Why Rust is popular:**

- Prevents common bugs at compile time (e.g. null pointers, buffer overflows)
- Loved for CLI tools, WebAssembly, embedded systems, game dev, and backend services
- Used by companies like Microsoft, Amazon, Dropbox, and Cloudflare

**Create a New Project**

```
cargo new hello_world
cd hello_world
```
**This creates:**

```
hello_world/
├── Cargo.toml        # Project config
└── src/
    └── main.rs       # Main source file

```

**Open src/main.rs — it already has:**
```
fn main() {
    println!("Hello, world!");
}
```

**Run the Program**
```
cargo run
```

**Output:**
```
   Compiling hello_world v0.1.0
    Finished dev [unoptimized + debuginfo] target(s)
     Running `target/debug/hello_world`
Hello, world!
```

> We can also manually create project and compile using rustc and compiled output can run. Please refer: https://doc.rust-lang.org/book/ch01-02-hello-world.html

**Cargo Package Commands**
- Create new project
    ```
    cargo new <project_name>
    ```
- Run the project
    ```
    cd <project_name>
    cargo build
    cargo run
    ```
- Release build creation
    ```
    cd <project_name>
    cargo build --release
    ```

# Data Types
## Rust Data Types Reference

Rust is a statically typed language where every value has a specific type known at compile time. Rust's type system is divided into two main categories: **Scalar Types** and **Compound Types**.

## Scalar Types (Primitive)

Scalar types represent single values and are the building blocks of Rust's type system.

### Integer Types

Rust provides both signed and unsigned integers in various bit sizes:

**Signed Integers** (can hold negative values):
- `i8`: -128 to 127
- `i16`: -32,768 to 32,767  
- `i32`: -2,147,483,648 to 2,147,483,647 (default)
- `i64`: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
- `i128`: -170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687,303,715,884,105,727
- `isize`: pointer-sized signed integer (32-bit on 32-bit systems, 64-bit on 64-bit systems)

**Unsigned Integers** (only positive values):
- `u8`: 0 to 255
- `u16`: 0 to 65,535
- `u32`: 0 to 4,294,967,295
- `u64`: 0 to 18,446,744,073,709,551,615
- `u128`: 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455
- `usize`: pointer-sized unsigned integer

```rust
let x: i32 = -42;        // 32-bit signed integer
let y: u64 = 1000;       // 64-bit unsigned integer
let z = 100;             // defaults to i32
```

### Floating-Point Types

Rust has two floating-point types for numbers with decimal points:

- `f32`: 32-bit floating point (single precision)
- `f64`: 64-bit floating point (double precision, default)

```rust
let pi: f64 = 3.14159;   // 64-bit float
let e: f32 = 2.718;      // 32-bit float
let default_float = 2.0; // defaults to f64
```

### Boolean Type

The `bool` type represents logical values:

```rust
let is_active: bool = true;
let is_complete = false;     // type inferred as bool
```

### Character Type

The `char` type represents a Unicode scalar value (4 bytes):

```rust
let letter: char = 'A';
let emoji: char = '😊';
let unicode: char = 'ñ';
```

## Compound Types

Compound types group multiple values into a single type.

### Arrays

Arrays are fixed-size collections of elements of the same type, allocated on the stack:

**Key Properties:**
- **Homogeneous**: All elements must be the same type
- **Fixed size**: Length is known at compile time
- **Stack allocated**: Fast access but limited size

```rust
// Declaration with explicit type and size
let numbers: [i32; 5] = [1, 2, 3, 4, 5];

// Array with repeated values
let zeros: [i32; 3] = [0; 3];  // [0, 0, 0]

// Type inference
let fruits = ["apple", "banana", "orange"];  // [&str; 3]

// Accessing elements
let first = numbers[0];  // 1
```

### Tuples

Tuples group together values of different types into a single compound type:

**Key Properties:**
- **Heterogeneous**: Can contain different types
- **Fixed size**: Number of elements known at compile time
- **Ordered**: Elements accessed by position

```rust
// Mixed types in a tuple
let person: (String, i32, bool) = ("Alice".to_string(), 25, true);

// Destructuring
let (name, age, is_student) = person;

// Accessing by index
let name = person.0;
let age = person.1;

// Unit tuple (empty tuple)
let unit: () = ();
```

### Slices

Slices are dynamically-sized views into contiguous sequences of elements:

**Key Properties:**
- **Dynamic size**: Size determined at runtime
- **Reference type**: Borrows data, doesn't own it
- **Contiguous memory**: Elements stored next to each other

```rust
let array = [1, 2, 3, 4, 5];

// Slice of the entire array
let slice: &[i32] = &array;

// Partial slice
let partial: &[i32] = &array[1..4];  // [2, 3, 4]

// String slices
let text = "Hello, world!";
let hello: &str = &text[0..5];  // "Hello"
```

## Additional Compound Types

### Vectors

Vectors are growable arrays allocated on the heap:

```rust
let mut numbers: Vec<i32> = Vec::new();
numbers.push(1);
numbers.push(2);

// Or with macro
let fruits = vec!["apple", "banana", "orange"];
```

### Strings

Rust has two main string types:

```rust
let string_literal: &str = "Hello";        // String slice
let owned_string: String = String::from("Hello");  // Owned string
```

## Memory Allocation Summary

- **Stack allocated**: Scalar types, arrays, tuples (when containing stack types)
- **Heap allocated**: Vectors, owned Strings, and other dynamically-sized types
- **Reference types**: Slices borrow memory and don't own the data

## Type Inference and Annotations

Rust can often infer types, but explicit annotations improve code clarity:

```rust
let x = 42;           // Inferred as i32
let y: u64 = 42;      // Explicit type annotation
let z = 42u64;        // Type suffix
```
    

## Functions

Functions in Rust are declared with the `fn` keyword and emphasize memory safety through ownership rules.

## Basic Syntax

```rust
fn function_name(parameter: Type) -> ReturnType {
    // function body
    expression_or_return_statement
}
```

## Function Examples

### Simple Function (No Return Value)
```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Usage
greet("World");
```

### Function with Return Value
```rust
fn add(x: i32, y: i32) -> i32 {
    x + y  // implicit return (no semicolon)
}

// Alternative with explicit return
fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}

// Usage
let result = add(5, 3);  // result = 8
```

### Multiple Return Values (Tuples)
```rust
fn calculate(x: i32) -> (i32, i32, i32) {
    (x + 1, x * 2, x * x)
}

// Usage
let (incremented, doubled, squared) = calculate(4);
```

### Generic Functions
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Usage with different types
let numbers = vec![34, 50, 25, 100, 65];
let result = largest(&numbers);

let chars = vec!['y', 'm', 'a', 'q'];
let result = largest(&chars);
```

## Ownership in Functions

### Taking Ownership
```rust
fn take_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and is dropped
```

### Borrowing (Immutable Reference)
```rust
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but refers to something else, so nothing happens
```

### Mutable Borrowing
```rust
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Usage
let mut s = String::from("hello");
change(&mut s);
```

### Returning References
```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

## Advanced Function Features

### Function Pointers
```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Usage
let answer = do_twice(add_one, 5); // answer = 12
```

### Closures
```rust
let add_one = |x: i32| x + 1;
let result = add_one(5); // result = 6

// Capturing environment
let x = 4;
let equal_to_x = |z| z == x;
let y = 4;
assert!(equal_to_x(y));
```

### Associated Functions (Static Methods)
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (like static method)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Usage
let sq = Rectangle::square(3);  // Associated function
let area = sq.area();           // Method
```

## Key Rules & Best Practices

### Return Types
- If no return type specified, function returns unit type `()`
- Last expression without semicolon is returned
- Use `return` keyword for early returns

### Naming Convention
- Use `snake_case` for function names
- Be descriptive: `calculate_tax()` not `calc()`

### Error Handling
```rust
fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(x / y)
    }
}

// Usage
match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(error) => println!("Error: {}", error),
}
```

### Documentation
```rust
/// Calculates the area of a rectangle.
/// 
/// # Examples
/// 
/// ```
/// let area = calculate_area(5, 10);
/// assert_eq!(area, 50);
/// ```
fn calculate_area(width: u32, height: u32) -> u32 {
    width * height
}
```

## Memory Safety Notes

- **No null pointer dereferences**: Use `Option<T>` instead of null
- **No dangling pointers**: Borrow checker ensures references are valid
- **No buffer overflows**: Array bounds are checked at runtime
- **No use after free**: Ownership system prevents this at compile time

## Quick Tips

1. **Prefer borrowing over taking ownership** when possible
2. **Use `&str` instead of `String`** for function parameters when you only need to read
3. **Return `Result<T, E>`** for functions that can fail
4. **Use generics** to write flexible, reusable functions
5. **Document public functions** with `///` comments

## Rust Memory Management Snapshot

### 🏠 Ownership
Each value has exactly one owner. When owner goes out of scope, value is dropped.
```rust
let s1 = String::from("hello");
let s2 = s1; // s1 moved to s2, s1 no longer valid
```

### 📝 Borrowing & References
Use values without taking ownership via references (`&` and `&mut`).

#### Immutable Reference
```rust
fn get_length(s: &String) -> usize { s.len() }
let s = String::from("hello");
let len = get_length(&s); // s still valid after call
```

#### Mutable Reference
```rust
fn append_world(s: &mut String) { s.push_str(" world"); }
let mut s = String::from("hello");
append_world(&mut s);
```

### 🔒 Borrowing Rules
- **One mutable reference** OR **any number of immutable references** at a time
- References must always be valid (no dangling pointers)

```rust
let mut s = String::from("hello");
let r1 = &s;     // OK - immutable
let r2 = &s;     // OK - multiple immutable
// let r3 = &mut s; // ERROR - can't mix mutable with immutable
```

### 📊 Copy vs Move Types
- **Copy types** (i32, bool, char): Copied on assignment
- **Move types** (String, Vec): Ownership transferred on assignment

```rust
let x = 5; let y = x;        // x copied, both valid
let s1 = String::from("hi"); // s1 moved, only s2 valid
let s2 = s1;
```

### 🎯 Key Benefits
- **Memory safety** without garbage collection
- **Zero-cost abstractions** - no runtime overhead
- **Compile-time checks** prevent dangling pointers, double-free errors
- **Predictable performance** - know exactly when memory is freed


## Variables

Variables in Rust are immutable by default and must be explicitly declared as mutable when needed. Rust uses static typing with powerful type inference capabilities.

### Variable Declaration

#### Immutable Variables
```rust
let x = 5;
let name = "Alice";
let is_active = true;
```

#### Mutable Variables
```rust
let mut count = 0;
count += 1;

let mut message = String::from("Hello");
message.push_str(", World!");
```

### Type Annotations

While Rust can infer types, you can explicitly specify them:

```rust
let x: i32 = 42;
let pi: f64 = 3.14159;
let letter: char = 'A';
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
```

### Constants

Constants are always immutable and must have explicit type annotations:

```rust
const MAX_USERS: u32 = 1000;
const PI: f64 = 3.14159265359;
```

### Shadowing

Rust allows variable shadowing, creating a new variable with the same name:

```rust
let x = 5;
let x = x + 1;        // x is now 6
let x = x * 2;        // x is now 12

// Shadowing can change the type
let spaces = "   ";
let spaces = spaces.len();  // spaces is now a number
```

### Scope and Ownership

Variables have scope determined by the block they're declared in:

```rust
fn main() {
    let outer = "I'm in the outer scope";
    
    {
        let inner = "I'm in the inner scope";
        println!("{}", outer);  // ✅ Can access outer variable
        println!("{}", inner);  // ✅ Can access inner variable
    }
    
    println!("{}", outer);      // ✅ Still accessible
    // println!("{}", inner);   // ❌ Error: inner is out of scope
}
```

### Common Patterns

#### Destructuring
```rust
let tuple = (1, 2, 3);
let (x, y, z) = tuple;

let array = [1, 2, 3, 4, 5];
let [first, second, ..] = array;
```

#### Option and Result Handling
```rust
let maybe_number: Option<i32> = Some(42);
if let Some(number) = maybe_number {
    println!("Got number: {}", number);
}

let result: Result<i32, &str> = Ok(10);
match result {
    Ok(value) => println!("Success: {}", value),
    Err(error) => println!("Error: {}", error),
}
```

### Best Practices

- **Prefer immutability**: Use `let` by default, only use `mut` when necessary
- **Use descriptive names**: `user_count` instead of `uc`
- **Leverage type inference**: Let Rust infer types when context is clear
- **Use constants for magic numbers**: Define `const` for repeated literal values
- **Consider shadowing**: Use shadowing for type transformations and value updates

### Examples

```rust
fn demonstrate_variables() {
    // Basic variable declaration
    let language = "Rust";
    let version = 1.70;
    let mut popularity_score = 85;
    
    // Type annotations when needed
    let user_ids: Vec<u32> = Vec::new();
    let temperature: f32 = 23.5;
    
    // Constants
    const MAX_CONNECTIONS: usize = 100;
    
    // Shadowing example
    let input = "123";
    let input: i32 = input.parse().expect("Not a number!");
    
    // Mutable operations
    popularity_score += 5;
    
    println!("Language: {}, Version: {}, Score: {}", 
             language, version, popularity_score);
}
```

## Control Flow

Rust provides several control flow constructs that allow you to control the execution path of your programs. All control flow expressions in Rust can return values, making them more powerful than in many other languages.

### Conditional Statements

#### `if` Expressions

```rust
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}
```

#### `if` as Expression

Since `if` is an expression, you can use it to assign values:

```rust
let condition = true;
let number = if condition { 5 } else { 6 };

let x = 10;
let result = if x > 5 {
    "greater than 5"
} else {
    "less than or equal to 5"
};
```

### Loops

#### `loop` - Infinite Loop

```rust
let mut counter = 0;

let result = loop {
    counter += 1;
    
    if counter == 10 {
        break counter * 2;  // Return value from loop
    }
};

println!("The result is {}", result);  // Output: 20
```

#### `while` Loop

```rust
let mut number = 3;

while number != 0 {
    println!("{}!", number);
    number -= 1;
}

println!("LIFTOFF!!!");
```

#### `for` Loop

```rust
// Iterating over a range
for number in 1..4 {
    println!("{}!", number);
}

// Iterating over a collection
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("the value is: {}", element);
}

// Iterating with index
for (index, value) in a.iter().enumerate() {
    println!("Index: {}, Value: {}", index, value);
}
```

### Pattern Matching

#### `match` Expression

The `match` expression is Rust's most powerful control flow construct:

```rust
let dice_roll = 9;

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

#### Matching with Values

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything else"),  // _ is a catch-all pattern
}
```

#### Matching Ranges

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

#### Matching Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}
```

### Conditional Control Flow

#### `if let` Expression

Concise way to match against one pattern:

```rust
let some_u8_value = Some(0u8);

if let Some(3) = some_u8_value {
    println!("three");
} else {
    println!("not three");
}

// More practical example
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

#### `while let` Loop

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### Loop Control

#### `break` and `continue`

```rust
let mut count = 0;

'counting_up: loop {
    println!("count = {}", count);
    let mut remaining = 10;

    loop {
        println!("remaining = {}", remaining);
        if remaining == 9 {
            break;  // Break inner loop
        }
        if count == 2 {
            break 'counting_up;  // Break outer loop with label
        }
        remaining -= 1;
    }

    count += 1;
}
```

#### Returning Values from Loops

```rust
let mut counter = 0;

let result = loop {
    counter += 1;
    
    if counter == 10 {
        break counter * 2;
    }
};

println!("The result is {}", result);
```

### Advanced Patterns

#### Guards in Match

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

#### Multiple Patterns

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything else"),
}
```

#### Destructuring Structs

```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };

match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
}
```

### Best Practices

- **Use `match` for exhaustive pattern matching**: Prefer `match` over multiple `if` statements when dealing with enums
- **Leverage `if let` for single pattern matches**: More concise than `match` when you only care about one case
- **Use meaningful loop labels**: When using nested loops, label them descriptively
- **Prefer `for` loops over `while` for iteration**: More idiomatic and less error-prone
- **Take advantage of expression nature**: Use control flow expressions to assign values directly

### Examples

```rust
fn control_flow_examples() {
    // Complex conditional logic
    let temperature = 25;
    let weather = match temperature {
        t if t < 0 => "freezing",
        0..=15 => "cold",
        16..=25 => "mild",
        26..=35 => "warm",
        _ => "hot",
    };
    println!("It's {} today", weather);
    
    // Processing a collection
    let numbers = vec![1, 2, 3, 4, 5];
    for (index, &number) in numbers.iter().enumerate() {
        match number % 2 {
            0 => println!("Index {}: {} is even", index, number),
            _ => println!("Index {}: {} is odd", index, number),
        }
    }
    
    // Option handling
    let maybe_name = Some("Alice");
    if let Some(name) = maybe_name {
        println!("Hello, {}!", name);
    }
    
    // Loop with early exit
    let mut attempts = 0;
    let target = 7;
    
    let found = loop {
        attempts += 1;
        let guess = attempts * 2 + 1;
        
        if guess == target {
            break true;
        } else if attempts >= 5 {
            break false;
        }
    };
    
    println!("Found target: {}", found);
}
```