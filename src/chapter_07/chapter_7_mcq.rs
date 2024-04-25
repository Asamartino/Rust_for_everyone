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
/// When should you use an modules in Rust?
///
/// - a) 
/// - b) 
/// - c) 
/// - d) 
pub fn answer_01() -> char {
    todo!()
}

/// ## Question 2
///
/// What is the keyword used to declare a new module?
///
/// - a) new module
/// - b) mod
/// - c) module
/// - d) import
pub fn answer_02() -> char {
    todo!()
}

/// ## Question 3
///
/// Is the below sentence correct:
/// By default, functions, types, constants and modules are public. 
///  
/// - a) Yes.
/// - b) No, it should be written: by default, functions, types, constants and modules are private.
/// - c) No, only function and modules are public.
/// - d) No, only type and constant are public.
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// What command should you use to compile a library crate's code ?
///
/// - a) cargo build
/// - b) cargo fetch
/// - c) cargo run
/// - d) cargo compile
pub fn answer_04() -> char {
    todo!()
}

/// ## Question 5
///
/// Complete thIs the Which sentence below is incorrect regarding match?
///
/// - a) 
/// - b) 
/// - c) 
/// - d) 
pub fn answer_05() -> char {
    todo!()
}

/// ## Question 
///
/// Complete the below sentence: 
/// When there are no submodules for the module `bar`, you should place the declarations for `bar` in a file named:
///
/// - a) main.rs 
/// - b) mod.rs
/// - c) bar.rs
/// - d) lib.rs
///
pub fn answer_06() -> char {
    todo!()
}

/// ## Question 7
///
///
/// Complete the below sentence: 
/// When there are submodules for the module `bar`, you should place the declarations for `bar` in a file named:
/// 
/// - a) bar/main.rs 
/// - b) bar/mod.rs
/// - c) bar/bar.rs
/// - d) bar/lib.rs
pub fn answer_07() -> char {
    todo!()
}

/// ## Question 8
///
/// What keyword should you use to define a function public?
///
/// - a) public
/// - b) Nothing, all functions are public by default
/// - c) pub
/// - d) publick
pub fn answer_08() -> char {
    todo!()
}

/// ## Question 9
///
/// What keyword should you use to define a function private?
/// 
/// - a) private
/// - b) Nothing, all functions are private by default
/// - c) priv
/// - d) you need to prefix the name of the function with an underscore
pub fn answer_09() -> char {
    todo!()
}

/// ## Question 10
///
/// Is the below sentence correct:
/// If an item is public, it can be accessed trhough any of its parent modules
///
/// 
/// - a) Yes
/// - b) No, because you can only have one immutable reference
/// - c) No, because we can't have a mutable reference while also having an immutable one
pub fn answer_10() -> char {
    todo!()
}
/// ## Question 11
///
/// Which of the following are the rules of reference?
///
/// n°1 References must always be valid
/// n°2 At any given time, you can have either but bot both of the following: one mutable reference or any number of immutable references
/// n°3 Reference can be invalid thanks to Rust ownership rules
/// n°4 At any given time, you can have either but bot both of the following: one immutable reference or any number of mutable references
///
/// - a) n°1 and n°2
/// - b) n°1 and n°4
/// - c) n°2 and n°3
/// - d) n°3 and n°4
pub fn answer_11() -> char {
    todo!()
}

/// ## Question 12
///
/// What is the type of s in the below code?
///
/// ```notest
/// let s = "Hello"
/// ```
///
/// - a) String, which is a mutable reference
/// - b) &str, which is an mutable reference
/// - c) String, which is a immutable reference
/// - d) &str, which is an immutable reference
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
