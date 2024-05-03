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
/// What is a backtrace ?
///
/// - a) A variant of the Result Type
/// - b) An enumeration of the various function called to reach this stage
/// - c) A small trace that some programmers have in their underwear
/// - d) An other word for unrecoverable error
pub fn answer_01() -> char {
    todo!()
}

/// ## Question 2
///
/// What happen when the panic! macro is executed?
///
/// - a) The program will display an error message, rollback and clean up the stack, and then terminate
/// - b) The program will have sudden uncontrollable anxiety and produce incoherent results.
/// - c) The program will display an error message
/// - d) The program terminate without cleaning up
pub fn answer_02() -> char {
    todo!()
}

/// ## Question 3
///
/// What is the Result Type ?
///
/// - a) An enum
/// - b) A struct
/// - c) A hashmap
/// - d) A priority queue implemented with a binary heap
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// How are recoverable errors handled?
///
/// - a) Using the Result Type
/// - b) Using the panic! macro
/// - c) Using a backtrace
/// - d) Due to the type system, Rust code does not produce errors
pub fn answer_04() -> char {
    todo!()
}

/// ## Question 5
///
/// Is the sentence correct?
///
/// The Result enum and its variants are automatically imported into every Rust program.
///
/// - a) Yes
/// - b) No, because the Result enum is imported in the prelude, which is not automatically imported into every Rust program
/// - c) No, because Result is a struct
/// - d) No, because they must be manually imported
pub fn answer_05() -> char {
    todo!()
}

/// ## Question 6
///
/// How are unrecoverable errors handled?
///
/// - a) with a buffer overread
/// - b) with the Result Type
/// - c) they are not, as the name suggest they are unable to be recovered
/// - d) with the panic! macro that stops execution as soon as one is encountered
pub fn answer_06() -> char {
    todo!()
}

/// ## Question 7
///
/// Unwinding the stack can be quite demanding. What alternative immediately stops the program without cleaning up?
/// It could also be used to make the binary as small as possible.
///
/// - a) abort
/// - b) neglect
/// - c) stop
/// - d) !consuela
pub fn answer_07() -> char {
    todo!()
}

/// ## Question 8
///
/// What does the unwrap method of the Result type do?
///
/// - a) Returns true if the result is Err.
/// - b) It unwraps the contained Ok value.
/// - c) If the Result type is the Ok variant, it returns the inner value of Ok. It panics if the value is an Err.
/// - d) It coerces the Ok variant into Err and panics.
pub fn answer_08() -> char {
    todo!()
}

/// ## Question 9
///
/// Using the unwrap method or the expect method is generally discouraged. Why is that and what are good use cases for those methods.
///
/// - a) Because they can panic. They can be useful while you are prototyping and don't know yet how to handle errors or if some other logic ensure that Result have an Ok value.
/// - b) Because they can panic. They are useful when the program does not need to stop entirely.
/// - c) Because they are deprecated. They can be useful when working with older Rust version.
/// - d) Because they can panic. They should only be used while you are prototyping and don't know yet how to handle errors.
pub fn answer_09() -> char {
    todo!()
}

/// ## Question 10
///
/// When could you return a Result?
///
/// - a) when a function might fail
/// - b) only for File I/O operations
/// - c) only when a function returns no value if successful
/// - d) whenever a function is guaranteed to not raise an error
pub fn answer_10() -> char {
    todo!()
}

/// ## Question 11
///
/// When should your code panic ?
///
/// - a) When you encounter unrecoverable errors and continuing the execution of the program would lead to a inconsistent state.
/// - b) When you encounter recoverable errors and continuing the execution of the program would lead to a bad state.
/// - c) Whenever a Result variant is Err.
/// - d) When you don't have time to handle this problem.
pub fn answer_11() -> char {
    todo!()
}

/// ## Question 12
///
/// The question mark operator `?` placed after a Result value functions almost identically to the match expressions defined to handle Result values in listing 9-6.
/// However, there is a difference between them, namely?  
///
/// - a) ? is a more flexible construct that allows pattern matching on various types
/// - b) error values used with ? will be converted using the from function
/// - c) ok values used with ? will be converted using the into function
/// - d) match simplifies error handling by reducing boilerplate code
pub fn answer_12() -> char {
    todo!()
}

/// ## Question 13
///
/// The question mark operator `?` can be used:
///
/// - a) everywhere
/// - b) only in function that returns Result
/// - c) only inside the main function
/// - d) only when the code in unclear
pub fn answer_13() -> char {
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

    #[test]
    fn answer_12_sanity_check() {
        sanity_check(&answer_12)
    }

    #[test]
    fn answer_13_sanity_check() {
        sanity_check(&answer_13)
    }
}
