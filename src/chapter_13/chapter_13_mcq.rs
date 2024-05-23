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
/// What is a closure in Rust?
/// - a) closures are anonymous functions, that can capture values from the scope in which they are called
/// - b) a way to capture and protect variable making them safe
/// - c) closures are known functions, that can capture values from the scope in which they are called
/// - d) closures are anonymous functions, that can not capture values from the scope in which they are called
pub fn answer_01() -> char {
    todo!()
}

/// ## Question 2
///
/// What is a good use case for a closure in Rust?
/// - a) to make your code look more professional
/// - b) to encapsulate all the environment variable
/// - c) to decapsulate all teh environment variable
/// - d) to write a function in one place and only execute it if needed
pub fn answer_02() -> char {
    todo!()
}

/// ## Question 3
///
/// What does adding contain 
/// ```notest
/// let adding = |num1, num2| num1+num2;
/// ```
/// 
/// - a) the result of num1+num2
/// - b) the definition of an anonymous function 
/// - c) the definition of a known function
/// - d) the result of the absolute value of num1 and num2 added together
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// Complete the below sentence:
/// Closure do not require you to annotate the types of the the parameters. ...
/// 
/// - a) The compiler infers the type every time the closure is called. Thus, the same closure could be used with different types.
/// - b) 
/// - c) However, once inferred they are fixed and we could get a type error by using the closure with another type.
/// - d) However, you can only call a closure once.
pub fn answer_04() -> char {
    todo!()
}

// What is referred to memoization
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

    #[test]
    fn answer_12_sanity_check() {
        sanity_check(&answer_12)
    }

    #[test]
    fn answer_13_sanity_check() {
        sanity_check(&answer_13)
    }

    #[test]
    fn answer_14_sanity_check() {
        sanity_check(&answer_14)
    }

    #[test]
    fn answer_15_sanity_check() {
        sanity_check(&answer_15)
    }

    #[test]
    fn answer_16_sanity_check() {
        sanity_check(&answer_16)
    }

    #[test]
    fn answer_17_sanity_check() {
        sanity_check(&answer_17)
    }

    #[test]
    fn answer_18_sanity_check() {
        sanity_check(&answer_18)
    }
}
