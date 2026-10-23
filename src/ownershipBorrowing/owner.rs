// # Rust Ownership, Borrowing, Moving, and Cloning

// ## 1. Borrowing and References

// **Borrowing** allows you to access a value without taking ownership of it. You can borrow a value by creating a reference to it using `&`.

// We can pass a reference to a function so that the function can use the value without becoming its owner. When the function finishes, the original owner can still use the value.

// ### Example: Borrowing a String

// ```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn print_name(name: &String) {
//     println!("{name}");
// }

// fn main() {
//     let name = String::from("Vishal");

//     print_name(&name);

//     println!("{name}");
// }

// ```

// **Explanation:**
// - `s1` owns the `String`.
// - `&s1` passes a reference to the string without transferring ownership.
// - `s: &String` means the function borrows a reference to a `String`.
// - `s.len()` returns the length of the string.
// - `s1` remains valid after the function call.

// ## 2. Moving Ownership

// In Rust, assigning a `String` to another variable moves ownership by default.

// A `String` stores its data on the heap and maintains metadata, such as a pointer, length, and capacity. When we assign one `String` variable to another, Rust moves ownership instead of automatically copying the heap-allocated characters.

// After the move, the original variable can no longer be used.

// ### Example: Moving a String

// ```rust
// fn main() {
//     let a = String::from("Hello");
//     let b = a;

//     println!("{}", b);

//     // println!("{}", a); // Compile-time error
// }
// ```

// **Explanation:**
// - `a` initially owns the string `"Hello"`.
// - `let b = a;` moves ownership from `a` to `b`.
// - `b` becomes the owner of the string.
// - `a` is no longer valid and cannot be used.

// Rust prevents both variables from independently owning and freeing the same heap allocation. This helps prevent memory-safety problems, such as double-free errors.

// ## 3. Cloning a String

// If you need two independent copies of a `String`, use the `.clone()` method.

// Cloning creates a separate `String` with its own heap-allocated character data, allowing both variables to remain valid.

// ### Example: Using `.clone()`

// ```rust
// fn main() {
//     let first = String::from("vishal");
//     let second = first.clone();

//     println!("{}", first);
//     println!("{}", second);
// }
// ```

// **Output:**

// ```text
// vishal
// vishal
// ```

// **Explanation:**
// - `first` owns the original string.
// - `first.clone()` creates a separate copy of the string.
// - `second` owns the cloned string.
// - Both variables can be used independently.

// Cloning a `String` requires additional time and memory because its contents are copied.

// ## 4. Moving vs. Cloning vs. Borrowing

// | Concept | Meaning | Syntax |
// |---|---|---|
// | Move | Transfers ownership to another variable | `let b = a;` |
// | Clone | Creates a separate owned copy | `let b = a.clone();` |
// | Borrow | Accesses a value without taking ownership | `let b = &a;` |

// ## 5. Practice Exercise

// Try the following steps:

// 1. Run `cargo run`.
// 2. Remove `.clone()` and observe the compiler error.
// 3. Explain why `second` works but `first` does not after ownership is moved.
// 4. Restore `.clone()` so both `println!` statements work.

// ## 6. Backend Development Takeaway

// Avoid cloning data unnecessarily. If you only need to read a value, borrowing it through a reference is often more efficient than creating a new copy.

// **Remember:**
// - **Move:** Transfer ownership.
// - **Borrow:** Access without taking ownership.
// - **Clone:** Create a separate owned copy.

// 5. Remember these rules
// - Move: transfers ownership; the old owner usually cannot be used afterward.
// - Clone: creates a separate owned copy when the type supports it.
// - Borrow with &: lets you read without taking ownership.
// - Borrow with &mut: lets you modify through a mutable reference.
// - Ownership
