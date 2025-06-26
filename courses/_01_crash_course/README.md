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
    - Also makes code easier to
    ```
        fn <function name>(<argument name>:<argument types>,...) -> <return type> {
            <expressions>
        }
    ```