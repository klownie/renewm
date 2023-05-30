//! This is the main documentation for my renewm.

/// This function greets the given name.
///
/// # Examples
///
/// ```
/// let greeting = greet("Alice");
/// assert_eq!(greeting, "Hello, Alice!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
