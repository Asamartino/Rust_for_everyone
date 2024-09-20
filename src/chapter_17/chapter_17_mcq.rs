//! # Multiple Choice Questions
//!
//! Replace the todo!() macros with your answer e.g.:
//!
//! Question 0
//!
//! Do you want to learn Rust?
//!
//! - a) yes
//! - b) no
//! - c) maybe
//! - d) I don't know
//!
//! ```notest
//! pub fn answer_0(){
//!  'a'
//! }
//! ```
//!
//! There is only one right answer per question
//!
//!
//! The below command will open the docs in a browser after building them
//! ```sh
//! cargo doc --open
//! ```

/// ## Question 1
///
/// What is encapsulation?
///
/// - a) The bundling of data into a single unit, while preventing direct access to all it’s inner elements
/// - b) The action of wrapping your data with another type
/// - c) The bundling of both data and methods into a single unit, while preventing direct access to some of it’s inner elements
/// - d) Preventing direct access to data
pub fn answer_01() -> char {
    todo!()
}

/// ## Question 2
///
/// What does inheritance means in programming?
///
/// - a) A concept in which an object acquires the properties and functionality of another object, enabling it to reuse and adapt the parent class’s code as necessary.
/// - b) The ability to process objects differently depending on their data type or class
/// - c) A concept where one data inherits the data from a variable that has been dropped
/// - d) The ability of a variable to hold the memory address of another variable
pub fn answer_02() -> char {
    todo!()
}

/// ## Question 3
///
/// what is polymorphism and does it exists in Rust?
///
/// - a) Polymorphism refers to code that is capable of operating on data of various types. It exists in Rust through the use of generics and traits
/// - b) Polymorphism refers to data having multiple forms It exists in Rust through the use of ownership
/// - c) Polymorphism refers to data having multiple forms. It doesn't exist in Rust
/// - d) Polymorphism refers to code that can process data belonging to multiple type. It exists in Rust through the use of trait object
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// What is a trait object?
///
/// - a) a variable that has been reduced to a specific trait, rather than recognizing its full identity and complexity
/// - b) a bounded automatic dispatch
/// - c) points to an instance of a type that implements a set of trait
/// - d) a way to define shared behavior across different types
pub fn answer_04() -> char {
    todo!()
}

/// ## Question 5
///
/// What is a dynamic dispatch?
///
/// - a) occurs when the compiler knows what method you are calling at compile time
/// - b) occurs when the specific method to be called is not known at compile time, and the resolution is deferred until runtime. There is no runtime cost
/// - c) happens when the compiler know what method you are calling at compile time and at runtime
/// - d) occurs when the specific method to be called is not known at compile time, and the resolution is deferred until runtime, which introduces a runtime cost
pub fn answer_05() -> char {
    todo!()
}

/// ## Question 6
///
/// When is a trait object safe?
///
/// - a) when the compiler doesn't emits any error
/// - b) when none of the methods return Self and there are no generic type parameters defined within the trait
/// - c) when none of the methods return Self
/// - d) when none of the methods have a generic type parameters
pub fn answer_06() -> char {
    todo!()
}

/// ## Question 7
///
/// Why should trait object be object safe?
///
/// - a) because Rust is designed with safety as a core principle
/// - b) because otherwise Rust compiler might be overwhelmed
/// - c) because once you use a trait object, Rust loses knowledge of the concrete type implementing that trait
/// - d) because it might create dangling references
pub fn answer_07() -> char {
    todo!()
}

/// ## Question 8
///
/// What does the Send trait do?
///
/// - a)
/// - b)
/// - c)
/// - d)
pub fn answer_08() -> char {
    todo!()
}

/// ## Question 9
///
/// What does the Sync trait do?
///
/// - a)
/// - b)
/// - c)
/// - d)
pub fn answer_09() -> char {
    todo!()
}

/// ## Question 10
///
/// Send and Sync are automatically derived traits.
/// This means that, if a type consists solely of Send or Sync components, it will be Send or Sync as well.
/// Implementing these traits manually would involve using unsafe Rust
///
/// Is the above sentence correct?
///
/// - a)
/// - b)
/// - c)
/// - d)
pub fn answer_10() -> char {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sanity_check(f: &dyn Fn() -> char) {
        assert!(
            "abcd".contains(f()),
            "{}",
            "You have not returned an answer a, b, c, d."
        )
    }

    #[test]
    fn answer_01_sanity_check() {
        sanity_check(&answer_01)
    }

    #[test]
    fn answer_02_sanity_check() {
        sanity_check(&answer_02)
    }

    #[test]
    fn answer_03_sanity_check() {
        sanity_check(&answer_03)
    }

    #[test]
    fn answer_04_sanity_check() {
        sanity_check(&answer_04)
    }

    #[test]
    fn answer_05_sanity_check() {
        sanity_check(&answer_05)
    }

    #[test]
    fn answer_06_sanity_check() {
        sanity_check(&answer_06)
    }

    #[test]
    fn answer_07_sanity_check() {
        sanity_check(&answer_07)
    }

    #[test]
    fn answer_08_sanity_check() {
        sanity_check(&answer_08)
    }

    #[test]
    fn answer_09_sanity_check() {
        sanity_check(&answer_09)
    }

    #[test]
    fn answer_10_sanity_check() {
        sanity_check(&answer_10)
    }

    #[test]
    fn answer_11_sanity_check() {
        sanity_check(&answer_11)
    }
}
