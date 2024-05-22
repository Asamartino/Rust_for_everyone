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
/// Why do we need to write tests?
/// - a) to ensure that our code behave as expected
/// - b) to increase the performance of our code
/// - c) to help the Rust compiler catching type mistakes
/// - d) to prove the absence of bug in our code
pub fn answer_01() -> char {
    todo!()
}

/// ## Question 2
///
/// How many test can you write per file?
/// - a) 10
/// - b) 100
/// - c) 1
/// - d) as many as you want
pub fn answer_02() -> char {
    todo!()
}

/// ## Question
/// 
/// Is the below sentence correct?
/// Rust does all the borrower-checking and type checking at compile time. However, the Rust compiler can't assess if our code behave as we intend.
/// By adding tests, we can test our code to make sure it behaves as planned.
/// a) Yes
/// b) No, because the borrower-checking and type checking is done at runtime.
/// c) Yes, because adding test is the only way to show the absence of bug. 
/// d) No, because the Rust compiler also asses it our code behave as we intend. Rust is designed with a high degree of correctness.
pub fn answer_0000() -> char {
    todo!()
}

// question on cargo test binary pg 202

/// ## Question 3
///
/// For what reason do we need to write `use super::*` at the beginning of our test module.
/// - a) to import the inner module into scope
/// - b) to import the outer module into scope
/// - c) to import the in-between module into scope
/// - d) to import the outer and inner module into scope 
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// The `assert_ne!` macro is equivalent to the `assert!` macro combined with an expression using the != operator
/// Is the above sentence true?
/// - a) Yes
/// - b) No, the `assert_ne!` macro is equivalent to the `assert!` macro combined with an expression using the == operator
/// - c) No, the `assert_eq!` macro is equivalent to the `assert!` macro combined with an expression using the != operator
/// - d) No, the `assert_ne!` macro is equivalent to the `assert_eq!` macro combined with an expression using the == operator
pub fn answer_04() -> char {
    todo!()
}

/// ## Question 5
///
/// With the `assert_eq!` macro and the `asset_ne!` macro what is the correct order for the value we expect and the value that the code produce?
/// - a) left: value we expect, right: value that the code produce
/// - b) left: value that the code produce, right: value we expect
/// - c) it doesn't matter
/// - d) left: value we expect, right: value we expect
pub fn answer_05() -> char {
    todo!()
}

/// ## Question 5
///
/// Why is assert_eq! more convenient than assert! ?
/// - a) because it saves time and resources making a positive impact on the planet
/// - b) because it also check for ownerships errors
/// - c) because it is specially optimized to be executed by the Rust compiler
/// - d) because it will also print both value if the test fails which makes it easier to debug


/// ## Question 6
///
/// Values being compared with the `assert_eq!` macro and the  `asset_ne!` macro must implement which traits? 
/// - a) Debug and PartialEq
/// - b) Eq and PartialSol
/// - c) Debug and PartialSol
/// - d) Eq and PartialCmp
pub fn answer_06() -> char {
    todo!()
}

/// ## Question 7
///
/// Is the below sentence correct:
///  A should_panic test can panic due to a different reason than the one we anticipated, making it imprecise.
/// - a) Yes
/// - b) No, because a should_panic test is precise
/// - c) Yes, and by using the substring method of should_panic we can make it more precise
/// - d) No, because it only panic for the reason we specified
pub fn answer_07() -> char {
    todo!()
}

/// ## Question 8
///
/// The `asset_ne!` macro use case
/// - a) 
/// - b) 
/// - c) 
/// - d)
pub fn answer_08() -> char {
    todo!()
}

/// ## Question 9
///
/// binary test 
/// - a) 
/// - b) 
/// - c) 
/// - d)
pub fn answer_09() -> char {
    todo!()
}

/// ## Question 10
///
/// When you are running multiple tests, they run in parallel by default
/// Is the above sentence correct?
/// // look test_threads flag
/// - a) Yes,
/// - b) No, they run in sequence by default
/// - c) No, 
/// - d) Yes, an
pub fn answer_10() -> char {
    todo!()
}


/// ## Question 11
///
/// By default, if we call a println! in a test and that test fails we won't see the println! output
/// Is the above sentence correct
/// - a) Yes, 
/// - b) No. By default, if we call a println! in a test and that test passes we won't see the println! output
/// - c) No. By default, if we call a println! in a test and that test passes we won't see the println! output
/// - d) Yes, because the output capture is enable by default
pub fn answer_11() -> char {
    todo!()
}

/// ## Question 12
///
/// question with ignore multiple test 
/// - a) 
/// - b) 
/// - c) 
/// - d)
pub fn answer_12() -> char {
    todo!()
}

/// ## Question 13
/// 
/// What is the purpose of a unit test
/// - a) 
/// - b) 
/// - c) 
/// - d)
pub fn answer_13() -> char {
    todo!()
}

/// ## Question 14
/// 
/// Is the below sentence correct?
/// By default, due to Rust privacy rules you can't test private functions
/// - a) Yes, and to enable it you should use the --private flag 
/// - b) No, Rust privacy rules allow you to test private functions
/// - c) No, Rust privacy rules only applies in integration tests
/// - d) Yes, and to enable it you should use the --no-private flag
pub fn answer_14() -> char {
    todo!()
}

/// ## Question 15
/// 
/// What is the purpose of integration tests?
/// - a) 
/// - b) 
/// - c) 
/// - d)
pub fn answer_15() -> char {
    todo!()
}

/// ## Question 16
///
/// - a) 
/// - b) 
/// - c) 
/// - d)
pub fn answer_16() -> char {
    todo!()
}

/// ## Question 17
///
/// - a) 
/// - b) 
/// - c) 
/// - d)
pub fn answer_17() -> char {
    todo!()
}

/// ## Question 18
///
/// - a) 
/// - b) 
/// - c) 
/// - d)
pub fn answer_18() -> char {
    todo!()
}

/// ## Question 19
///
/// - a) 
/// - b) 
/// - c) 
/// - d)


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
