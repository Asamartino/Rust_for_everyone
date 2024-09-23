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
/// What is a pattern in Rust?
///
/// - a) a regularity between the names of the variables in Rust
/// - b) a unique syntax for matching the structure of memory allocation strategies
/// - c) a unique syntax for matching the structure of types
/// - d) a regular arrangement in the Rust compiler ensuring efficient memory management
pub fn answer_01() -> char {
    todo!()
}

/// ## Question 2
///
/// Complete the sentence: match expressions need to be ...?
///
/// - a) exhausting, meaning that every wrong value takes a lot of compilation time
/// - b) exhaustive, meaning that every possible value in the match expression need to be handled
/// - c) extensive, meaning that most common value in the match expression must be considered
/// - d) exhilarate, meaning that it will make the compiler happy
pub fn answer_02() -> char {
    todo!()
}

/// ## Question 3
///
/// What is one disadvantage of using if let expressions
///
/// - a) exhaustiveness is enforced by the compiler
/// - b) it has less flexibility than a match expression
/// - c) exhaustiveness is not enforced by the compiler
/// - d) it has no disadvantages
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// Complete the sentence: A variable name is basically a simple instance of a ...?
///
/// - a) tuple
/// - b) iterator
/// - c) pattern
/// - d) match arm
pub fn answer_04() -> char {
    todo!()
}

/// ## Question 5
///
/// what happens to the value in the below code?
/// ```notest
/// let (x,y,_) = (1,2,3);
/// ```
///
/// - a) the value 3 is will be stored in the tuple (x,y)
/// - b) the value 3 is stored in _
/// - c) the value 3 is will be stored in the tuple (x,y,z)
/// - d) the value 3 is ignored and not stored anywhere
pub fn answer_05() -> char {
    todo!()
}

/// ## Question 6
///
/// What is an Arc<T>, and when should you use it?
///
/// - a)
/// - b)
/// - c)
/// - d)
pub fn answer_06() -> char {
    todo!()
}

/// ## Question 7
///
/// What is the trade-off with thread safety types?
///
/// - a)
/// - b)
/// - c)
/// - d)
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
