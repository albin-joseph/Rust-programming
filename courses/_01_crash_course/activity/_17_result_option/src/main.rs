
// Topic: Result & the question mark operator

// Summary:
// This small program simulates unlocking a door using digital keycards
// backed by a database. Many errors can occur when working with a database,
// making the question mark operator the perfect thing to use to keep
// the code manageable.

// Requirements:
// * Write the body of the `authorize` function. The steps to authorize a user
//   are:
//   1. Connect to the database
//   2. Find the employee with the `find_employee` database function
//   3. Get a keycard with the `get_keycard` database function
//   4. Determine if the keycard's `access_level` is sufficient, using the
//      `required_access_level` function implemented on `ProtectedLocation`.
//      * Higher `access_level` values grant more access to `ProtectedLocations`.
//        1000 can access 1000 and lower. 800 can access 500 but not 1000, ...
// * Run the program after writing your `authorize` function. Expected output:
//   Ok(Allow)
//   Ok(Deny)

fn main() {
    println!("Result & Option demonstaring activity");
}
