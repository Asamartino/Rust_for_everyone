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

////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
/// add question about reference and pointer see pg. 305

/// ## Question 1
///
/// Does Rust have a garbage collector?
///
/// - a) yes
/// - b) no
/// - c) let me consult my crystal ball... oh, it's foggy again... maybe?
pub fn answer_01() -> char {
    todo!()
}

/// ## Question 2
///
/// What is Rust ownership feature?
///
/// - a) a way to manage computer's CPU while running
/// - b) a way to manage computer's memory while running
/// - c) the act, state, or right of Rust to possess some features
pub fn answer_02() -> char {
    todo!()
}

/// ## Question 3
///
/// Is this sentence correct:
/// "Rust managed its memory trough a system of ownership with a set of rules that the compiler checks at compile time. This slow down your program while it's running"
///
/// - a) No, it should be "Rust managed its CPU through...
/// - b) Yes
/// - c) No, the ownership features do no slow down your program while it is running
/// - d) No, the compiler checks the ownership rules at runtime
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// Which of the below is *not* part of Rust's ownership rules:
///
/// - a) When the owner goes out of scope, the value will be dropped
/// - b) Each value in Rust has a variable called its owner
/// - c) There can be multiple owner at a time
/// - d) There can be only one owner at a time
pub fn answer_04() -> char {
    todo!()
}

/// ## Question 5
///
/// Which is faster: accessing data on the heap or accessing data on the stack?
///
/// - a) accessing data on the heap
/// - b) accessing data on the stack
/// - c) neither, there are both equally fast
/// - d) depends on the computer
pub fn answer_05() -> char {
    todo!()
}

/// ## Question 6
///
/// Is this sentence correct: "Ownership exist to manage heap data"
///
/// - a) Yes
/// - b) No
pub fn answer_06() -> char {
    todo!()
}

/// ## Question 7
///
/// Which comment in the below code are correct?
/// ```notest
/// {                              // n°1 s is valid hereinafter
///     let s = "hello world"      // n°2 s is valid hereinafter
///     // do thing with s
/// }                              // n°3 s goes out of scope and is no longer valid
/// ```
///
/// - a) Comment n°1, 2, and 3 are correct
/// - b) Comment n°1 is not correct, comment n°2 and 3 are correct.
/// - c) Comment n°2 is not correct, comment n°1 and 3 are correct.
/// - d) Comment n°3 is not correct, comment n°1 and 2 are correct.
pub fn answer_07() -> char {
    todo!()
}

/// ## Question 8
///
/// Does the below code compiles?
///
/// ```notest
/// {
///     let mut s1 = String::from("hello");
///     let mut s2 = s1;
///     s1.push_str(", world");
/// }
///
/// ```
///
/// - a) Yes
/// - b) No, because s1 is moved into s2 and is therefore no longer valid
/// - c) No, because s1 is stored on the stack
pub fn answer_08() -> char {
    todo!()
}

/// ## Question 9
///
/// Does the below code compile?
///
/// ```notest
///     pub fn main() {
///         let s = String::from("hello");
///         print_string(s);
///
///         let x: u32 = 5;
///         print_u32(x);
///
///         let x = 6;
///         println!("the value of variable x is: {}", x);
///     }
///
///     pub fn print_string(s: String) {
///         println!("the value of the string is: {}", s)
///     }
///     pub fn print_u32(x: u32) {
///         println!("the value of variable x is: {}", x)
///     }
/// ```
/// - a) Yes
/// - b) No, because s is moved into print_string()
/// - c) No, because x is declared twice
pub fn answer_09() -> char {
    todo!()
}

/// ## Question 10
///
/// Does the below code compiles?
///
/// ```notest
///         let mut s = String::from("hello");
///         
///         let s1 = &s;
///         let s2 = &s;
///         let s3 = &mut s;
///         let s4 = s;
///     }
/// ```
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
