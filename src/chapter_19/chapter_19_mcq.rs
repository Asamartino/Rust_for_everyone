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
//!

// added question String and Vec<T> are smart pointers see pg 306

/// ## Question 1
///
/// What is unsafe Rust?
///
/// - a) It enables four features that Rust’s borrow checker won't verify.
/// - b) It enables four features that the compiler won't verify for memory safety.
/// - c) It enables four features that Rust’s safety mechanisms won’t verify.
/// - d) As it names suggest it is unsafe so it should be used.
pub fn answer_01() -> char {
    todo!()
}

/// ## Question 2
///
/// What is not a use case of using unsafe Rust?
///
/// - a) To access or modify a static variable
/// - b) To do low-level systems programming
/// - c) To interface with another language where Rust's safety guarantees do not hold.
/// - d) To build up safe abstraction that the borrow checker doesn't understand
pub fn answer_02() -> char {
    todo!()
}

/// ## Question 3
///
/// Does unsafe Rust turn off the borrow checker?
///
/// - a) Yes
/// - b) No, but it does disable other Rust's safety checks
/// - c) No, it also doesn't disable other Rust's safety checks
/// - d) No, it turn off the borrow cracker
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// Complete this sentence: When using unsafe code it is recommended to ...
///
/// - a) Keep unsafe sections brief and allow unrestricted access to it (that is why it is unsafe)
/// - b) Maximize the size of the unsafe block and encapsulate it in safe abstractions that expose a safe API
/// - c) Maximize the size of the unsafe block and allow unrestricted access to it (that is why it is unsafe)
/// - d) Keep unsafe sections brief and encapsulate it in safe abstractions that expose a safe API
pub fn answer_04() -> char {
    todo!()
}

/// ## Question 5
///
/// When do you need to use unsafe with a raw pointer?
///
/// - a) When you created the raw pointer
/// - b) When you dereference the raw pointer
/// - c) When you reference the raw pointer
/// - d) Whenever you like, the machine should listen and obey
pub fn answer_05() -> char {
    todo!()
}

/// ## Question
///
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
///
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
///
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
/// Consider the below code:
///
///
/// - a)
/// - b)
/// - c)
/// - d)
pub fn answer_10() -> char {
    todo!()
}
/// ## Question 11
///
/// Does the below code compile and what What would be the output?
///
///
/// - a)
/// - b)
/// - c)
/// - d)
pub fn answer_11() -> char {
    todo!()
}

/// ## Question 12
///
/// Does the below code compile and why?
///
///
/// - a)
/// - b)
/// - c)
/// - d)
pub fn answer_12() -> char {
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
        sanity_check(&answer_10)
    }

    #[test]
    fn answer_12_sanity_check() {
        sanity_check(&answer_10)
    }
}
