fn main() {
    println!("Dummy application!");
}

/// Doc-Comment above some_doing(). The first line should be a summary!
/// Three slashes indicate a doc-comment, the style is defined in [RFC505](https://github.com/rust-lang/rfcs/blob/master/text/0505-api-comment-conventions.md).
///
/// # Some special things
/// Document comments are written in **markdown**!
/// This is like you write _readme.md_ at [Github](http://www.github.com).
///
/// - So we can do lists
/// - or something else
/// - or `some code snippet` here
///
/// | e.G. | A | table |
/// |---|---|---|
/// | Some | table | input |
///
/// # Headers
/// You should use headers for four categories
///
/// - Panics
/// - Errors
/// - Safety
/// - Examples
///
/// # Examples
///
/// This section shows examples. A special highlight: **Cargo** is able to test documentation examples by `cargo test` command. For this the file must be declared as a *lib*. For this look at **automatic_testing** folder of this repository or in file **lib.rs** (is named as *comment-lib.rs*, because `cargo doc` does not work if there is a *main.rs* and a *lib.rs* file in the same project).
///
/// ```
/// let x = 10;
/// let y = 20;
/// println!("Some output of this example {}", x * y);
/// assert_eq!(30, x+y);
/// ```
///
pub fn some_doing() {
    println!("Do something");
}
