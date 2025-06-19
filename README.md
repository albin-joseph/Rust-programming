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
    