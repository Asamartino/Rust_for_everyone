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
/// - a) Yes, and we can make it more precise by using the optional expected parameter
/// - b) No, because a should_panic test is precise
/// - c) Yes, and we can make it more precise by using the substring method of should_panic
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
/// When you are running multiple tests, they run in parallel by default.
/// Is the above sentence correct?
/// // look test_threads flag
/// - a) Yes
/// - b) No, they run in sequence by default
/// - c) No, you can't run multiple test
/// - d)
pub fn answer_10() -> char {
    todo!()
}

/// ## Question 11
///
/// By default, if we call a println! in a test and that test fails we won't see the println! output
/// Is the above sentence correct
/// - a) Yes, because the output capture is disable by default
/// - b) No, because you will see the println! output
/// - c) No, because you only see the failure message
/// - d) Yes, because The Rust's test library captures anything printed to standard output
pub fn answer_11() -> char {
    todo!()
}

/// ## Question
///
/// Using the command: `cargo test adding` will:
///
/// - a) run all test that start with adding
/// - b) run only the test adding
/// - c) run all test
/// - d) run all test that contain the word adding
pub fn answer_11() -> char {
    todo!()
}

/// ## Question 12
///
/// Using the command: `cargo test -- --ignored` will:
/// - a) run all test that have the #[ignore] line
/// - b) run all test and ignored the failing one
/// - c) run all test that are filtered out from the Rust compiler making the compilation slower but your code more safe
/// - d)
pub fn answer_12() -> char {
    todo!()
}

/// ## Question 13
///
/// What is the purpose of a unit test
/// - a) to test how fast your functions run
/// - b) to do only one thorough test of a function
/// - c) to test one module in isolation at a time or private interfaces
/// - d) to make sure the conversion between the different data types is respected (e.g. converting a u32 to a u16)
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
/// By convention where should you put the unit tests?
/// - a) in the src directory in the file with the code they are testing by creating a separated module named test
/// - b) in the module directory and create a file src.rs
/// - c) in the debug directory by importing the code they are testing and creating a separated module named test
/// - d) nowhere, the Rust compiler will take care of them
pub fn answer_15() -> char {
    todo!()
}

/// ## Question 16
/// 
/// Adding to much test in the test module can slow the compilation time when you run the command `cargo build`.
/// Is the above sentence true?
/// - a) Yes, that is why unit test should be as small as possible
/// - b) No, by adding the annotation `#[cfg(test)]` on the tests module, tests will only be compile and run with the command `cargo test`
/// - c) Yes, but it rarely happens as Rust has very good RAM 
/// - d) No, because tests do not need to be compiled.
pub fn answer_16() -> char {
    todo!()
}

/// ## Question 17
/// 
/// Do Rust privacy rules allow you to test private functions?
/// - a) Yes, but if you don't want to test them feel free to do so
/// - b) Yes, because by ideology is best to test private function
/// - c) No, because by ideology is best to not test private function
/// - d) No, this is why we have privacy rules in the first place
pub fn answer_17() -> char {
    todo!()
}

/// ## Question 18
/// 
/// What is the use of integration tests?
/// - a) to test the correct integration of your private function with the public one
/// - b) to test small module at a time 
/// - c) to test the private function of your library
/// - d) to use many parts of your library and verify that they work correctly together
pub fn answer_18() -> char {
    todo!()
}

/// ## Question 19
/// 
/// Where should you put integration tests?
/// - a) in a test directory at the top level of your project directory
/// - b) in a separate module inside your files
/// - c) wherever you like
/// - d) in an integration directory at the lowest level of your project directory


/// ## Question 20
/// 
/// What is the use of this command: `cargo test --test integration`?
/// - a) to run all the tests that do not contain the word integration
/// - b) to run all the tests that start with integration
/// - c) to run all the tests in an integration file called integration 
/// - d) to run all the tests in the integration folder

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
