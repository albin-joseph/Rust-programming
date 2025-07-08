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

## Fundamentals | Structure
- A type that contains multiple pieces of data
    -  All or nothing - cannot have some pieces of data and not others
- Each piece of data is called a "field"
- Makes working with data easier
    - Similar data can be grouped together
