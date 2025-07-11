# RUST CRASH COURSE

## Fundamentals | Data Types
- Memory only stores binary data
    - Anything can be represented in binary
- Program determines what the binary represents
- Basic types that are universally useful are provided by the language
- Basic Types
    - Boolean
        - true, flase
    - Int
        - 1, 2, 4
    - Double / Float
        - 10.0, 10.50
    - Character
        - 'A', 'Z', 'a'
    - String
        - "Albin", "abcd"

## Fundamentals | Variables
- Assign data to a temporary memory location
    - Allows programmer to easily work with memory
- Can be set to any value & type
- Immutable by default, but can be mutable
    - Immutable: cannot be changed
    - Mutable: can be changed
- Variables make it easier to work with data
- Variables can be assigned to any value
    - This include other variables
- Immutable by default

## Fundamentals | Functions
- A way to encapsulate program functionality
- Optionally accept data
- Optionally return data
- Utilized for code organization
    - Also makes code easier to read
```
        fn <function name>(<argument name>:<argument types>,...) -> <return type> {
            <expressions>
        }
```

- Can be executed by "calling" the function
- Parameters determine what data a function can work with
- Optionally "returns" data
    - Data sent back from the function

## Fundamentals | println macro
- Macros expand into additional code
- println "Prints" (displays) information to the terminal
- Useful for debugging
- Macros use an exclamation point to call/invoke
- Generate additional Rust code
- Data can be printed using println!:
    - {1:?}
    - {varname:?}
## Fundamentals | Control flow using "if"
- Code executed line-by-line
- Actions are performed & control flow may change
    - Specific conditions can change control flow
        - "if"
        - "else"
        - "else if"
- This can be changed using "if"
- Try to always include "else", unless there truly is no alternative case

## Fundamentals | Repetition using loops
- Called "looping" or "iteration"
- Multiple types of loops
    - "loop" - infinite loop
    - "while" - conditional loop
- Repetition can be performed using loops using
    - While loop
    - Infinite loop
- Both types of loops can exit using "break"

## Fundamentals | Match expressions
- Add logic to programs
- Similar to if..elese
- Exhaustive
    - All options must be accounted
- Use underscore (_) to match "anything else"

>**Match Vs else..if**
>- match will be checked by the compiler
>- If a new possibility is added, you will be notified when this occurs
>- else..if is not checked by compiler
>- If a new possibility is added, your code may contain a bug

## Fundamentals | Repetition using while
- loop declared with the keyword while
- which check the condition is satisfying, after that execute the espressions

## Fundamentals | Enumeration
- Data that can be one of multiple different possibilities
    -  Each possibility is called a "variant"
- Provides information about your program to the compiler
    - More robust programs
- Make program code to more readable
- enum variants can optionally contain data
- The data can be another enum
- Can mix plain identifiers and data-containing variants within the same enum
- More than one piece of data can be associated with a variant

## Fundamentals | Structure
- A type that contains multiple pieces of data
    -  All or nothing - cannot have some pieces of data and not others
- Each piece of data is called a "field"
- Makes working with data easier
    - Similar data can be grouped together
- Filed can be accessed using a dot (.)

## Fundamentals | Tuples
- A type of "record"
- Store data anonymously
    - No need to name fields
- Useful to return pairs of data from functions
    - Use struct when more that 2 or 3 variables
- Can be "destructured" easily into variables
- Usefule when destructing

## Fundamentals | Expressions
- Rust is an expression-based language
    - Most things are evaluated and return some value
- Expression values coalesce to a single point
    - Can be used for nesting logic
- Expression allow nested logic

## Fundamentals | Intermediate Memory
- Basic memory refresh
    - Memory is stored using binary
        - Bits: 0 or 1
    - Computer optimized for bytes
        - 1 byte == 8 contiguous bits
    - Fully contiguous
- Addresses
    - All data in memory has an "address"
        - Used to locate data
        - Always the same - only data changes
    - Usually don't utilize addresses directly
        - Variables handle most of the work
- Offsets
    - Items can be located at an address using an "offset"
    - Offsets begin at o
    - Represent the number of bytes away from the original address
        - Normally deal with indexes instead

## Fundamentals | Ownership
- Programs must track memory
    - If they fail to do so, a "leak" occurs
    - Memory must be managed in some way to prevent leaks
- Rust utilizes an "ownership" model to manage memory
    - The "owner" of memory is responsible for cleaning up the memory
    - This occurs automatically at the end of the scope
- Memory can either be "moved" or "borrowed"
    - Default behavior is to "move" memory to a new owner
    - Use an ampersand (&) to allow code to "borrow"

## Fundamentals | Impl
- impl blocks can be split into multiple blocks for the same struct.
- You can also use generics inside impl.

## Fundamentals | Type Annotations
- Required for function signatures
- Types are usually inferred
- Can also be specified in code
    - Explicit type annoatations
- Type annotations are mostly optional within function bodies
    -  Occasionally required if compiler cannot infer the type
- Can be specified when using let bindings

## Data Structures | Vector
- Multiple pieces of data
    - Must be the same type
- Used for lists of information
- Can add, remove, and traverse the entries
- The vec! macro can be used to make vectors
- Use for.. in to iterate through items of a vector

## Data Structures | String
- Two commonly used types of strings
    - String - owned
    - &str - borrowed String slice
    - Strings are automatically borrowed
- Use to_owned() or String:: from() to create an owned copy of a string slice
- Must use an owned String to store
- Use &str when passing to a function