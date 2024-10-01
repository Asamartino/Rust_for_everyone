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
/// When do you need to use the unsafe keyword with a raw pointer?
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
/// Does the presence of unsafe code within a function require the entire function to be marked as unsafe?
/// 
/// - a) Yes
/// - b) No
pub fn answer_06() -> char {
    todo!()
}

/// ## Question 7
///
/// When should you use the #[no_mangle] annotation?
/// 
/// - a) When you need to ensure that the compiler does not modify the name of a function or variable during compilation. This is particularly useful when interfacing with other languages.
/// - b) When two functions are so simialar that the compiler confuses them 
/// - c) Whenever variables are crippled by the compiler
/// - d) Whenever you have a data race
pub fn answer_07() -> char {
    todo!()
}

/// ## Question 8
///
/// Complete the sentence: In Rust, ... variables are called static variables
///
/// - a) dynamic
/// - b) global
/// - c) constant
/// - d) immutable
pub fn answer_08() -> char {
    todo!()
}

/// ## Question 9
///
/// Static variables and constants are both used to define values that do not change, but they have important differences in their characteristics and usage.
/// Which sentence below is incorrect:
///
/// - a) Constants variables could lead to data races
/// - b) Static variables are variables that are stored in a fixed memory location for the entire duration of the program. 
/// - c) Constants represent a value, not a memory address.
/// - d) Static variable are a possibly mutable variable with 'static lifetime.
pub fn answer_09() -> char {
    todo!()
}

/// ## Question 10
///
/// Complete this sentence: A trait is unsafe when ... of its methods has/have some invariant that the compiler can't verify 
///
///
/// - a) all
/// - b) exactly three
/// - c) at least one
/// - d) none
pub fn answer_10() -> char {
    todo!()
}
/// ## Question 11
///
/// what does lifetime subtyping do?
///
///
/// - a) Lifetime subtyping refers to the ranking of lifetimes, which can either be: primary or secondary.
/// - b) Lifetime subtyping is the principle that lifetime should satisfy the expectations of the compiler
/// - c) Lifetime subtyping is a relationship between lifetimes that allows one lifetime to be considered a subtype of another, hence one should outlive another.
/// - d) Lieftime subtyping is type of parser 
pub fn answer_11() -> char {
    todo!()
}

/// ## Question 12
///
/// What are lifetime bounds?
///
/// - a) Specifies the minimum lifetime a reference must live to remain valid
/// - b) Specifies the maximum lifetime a reference must live to remain valid
/// - c) Lifetime that have a moral duty to remain valid
/// - d) The upper limit of the amount of lifetime you can write per program
pub fn answer_12() -> char {
    todo!()
}

/// ## Question 12
///
/// The default lifetime of a trait object is ...?
///
/// - a) 'dynamic
/// - b) 'a
/// - c) 'static
/// - d) 'lexical
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
