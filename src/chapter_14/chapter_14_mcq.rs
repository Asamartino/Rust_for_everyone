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
/// Is the below sentence correct:
///     Cargo can only be use to build, run, and test our code
///
/// - a) Yes
/// - b) No, Cargo is only used to build and test our code
/// - c) No, Cargo is Rust's package manager
/// - d) No, Cargo is only used to ship production code.
pub fn answer_01() -> char {
    todo!()
}

/// ## Question 2
///
/// What is Rust release profiles?
/// - a) the time generally determined by Rust's coding degradation
/// - b) the professional profiles of the main developers working on Rust
/// - c) the different version of cargo
/// - d) adaptable configurations that enable developers to manage different compilation options more effectively.
pub fn answer_02() -> char {
    todo!()
}

/// ## Question 3
///
/// Rust includes a special type of comment that produces HTML documentation. What are they called?
///
/// - a) documentation comment
/// - b) HTML comment
/// - c) special comment
/// - d) You don't call them. They will randomly appear in your
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// How can you re-exports items from a module, making them accessible to other modules or crates that use your code?
///
/// - a) using pub use statements
/// - b) using priv use statements
/// - c) using API structures
/// - d) through the use of header files and namespace
pub fn answer_04() -> char {
    todo!()
}

/// ## Question 5
///
/// Once you publish your code to crates.io it is permanent. Why is that?
///
/// - a) Because crates.io is a public blockchain which is an immutable ledger.
/// - b) To ensure the permanent archive of code and ensure uninterrupted functionality for projects relying on them
/// - c) To prevent accidental deletion
/// - d) To incentivize developers to thoroughly review their code before publishing it. So only the best code is published.
pub fn answer_05() -> char {
    todo!()
}

/// ## Question 6
///
/// Is the below sentence correct?
/// There is no limit to the number of crate version you can publish.
///
/// - a) Yes.
/// - b) No, you can't publish more than 1000 versions
/// - c) No, you can only publish a maximum of three crate versions per day.
/// - d) No, you can only publish a new crate version if you delete an existing one.
pub fn answer_06() -> char {
    todo!()
}

/// ## Question 7
///
/// What does yanking refers to in Rust's ecosystem?
///
/// - a) The opposite of yinking, which is adding code
/// - b) deleting code
/// - c) removing a previously published crate’s version from the server’s index
/// - d) a slang for a Rust developer living in the U.S.
pub fn answer_07() -> char {
    todo!()
}

/// ## Question 8
///
/// What is a workspace in Rust?
///
/// - a) A collection of one or more packages, that are managed together.
/// - b) A small library
/// - c) A very big package
/// - d) A dependency to crates.io
pub fn answer_08() -> char {
    todo!()
}

/// ## Question 9
///
/// Is the below sentence correct:
///     Cargo ensure that the crates in a workspace dependent on each other.
///
/// - a) No, Cargo doesn't doesn't assume interdependencies among crates within a workspace, 
/// - b) Yes
/// - c) No, it is ensured by rustC 
/// - d) No, crates automatically depend on each other in a workspace 
pub fn answer_09() -> char {
    todo!()
}

/// ## Question 10
///
/// Does having all crates in a workspace sharing the same dependencies minimize compatibility conflicts?
///
/// - a) Yes, as crates share the same dependencies, they will always be compatible with each other
/// - b) No, because workspace crates do not depend on each other
/// - c) No, it maximize compatibility conflicts
/// - d) Yes, however, the lower bound remains at O(log n)
pub fn answer_10() -> char {
    todo!()
}

/// ## Question 11
///
/// We can defined our own Iterator and use the library. We only need to define the:
///
/// - a) subsequent method
/// - b) now method
/// - c) previous method
/// - d) next method
pub fn answer_11() -> char {
    todo!()
}

/// ## Question 12
///
/// Is the below sentence correct?
/// Iterators are a high level abstraction that impose no additional memory overhead.
///
/// - a) Yes.
/// - b) No, they impose no additional compile overhead.
/// - c) No, they impose no additional runtime overhead.
/// - d) No, iterators are a low level abstraction.
pub fn answer_12() -> char {
    todo!()
}

/// ## Question 13
///
/// What will the following code produce in the terminal?
/// ```notest
///     (0..5).map(|x| println!("{x}"));
/// ```
///
/// - a) 1,2,3,4,5
/// - b) 0,1,2,3,4
/// - c) 0,1,2,3,4,5
/// - d) nothing, because iterators are lazy
pub fn answer_13() -> char {
    todo!()
}

/// ## Question 14
///
/// What does the below code produce?
/// ```notest
///     let infinite_iterator = 0..;
/// ```
///
/// - a) a compilation error
/// - b) an infinite iterator,
/// - c) a finite iterator
/// - d) a limitless closure
pub fn answer_14() -> char {
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

    // #[test]
    // fn answer_11_sanity_check() {
    //     sanity_check(&answer_11)
    // }

    // #[test]
    // fn answer_12_sanity_check() {
    //     sanity_check(&answer_12)
    // }

    // #[test]
    // fn answer_13_sanity_check() {
    //     sanity_check(&answer_13)
    // }

    // #[test]
    // fn answer_14_sanity_check() {
    //     sanity_check(&answer_14)
    // }

    // #[test]
    // fn answer_15_sanity_check() {
    //     sanity_check(&answer_15)
    // }

    // #[test]
    // fn answer_16_sanity_check() {
    //     sanity_check(&answer_16)
    // }

    // #[test]
    // fn answer_17_sanity_check() {
    //     sanity_check(&answer_17)
    // }

    // #[test]
    // fn answer_18_sanity_check() {
    //     sanity_check(&answer_18)
    // }
}
