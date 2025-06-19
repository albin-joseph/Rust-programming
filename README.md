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

#### Data Types

Rust is a static typed language. We have different type primitive and non premitive data types

    - **Primitive Data types **
        - Int
            - Integeres signed and unsigned integers in different ranges
            - signed integers: i8, i16, i32, etc
            - unsigned integers: u8, u16, u32, etc
        - float
        - char