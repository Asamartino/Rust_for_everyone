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
pub fn answer_01() -> char {
    todo!()
}

/// ## Question 2
/// Is the below code valid:
/// ```notest
/// struct Coordinates<U> {
///     x: U,
///     y: U,
/// }
/// ```
/// - a) No, because it should be <T> instead of <U>, so the code won't compile.
/// - b) Yes, because we want x and y to have the possibility to be of different type (f.i. x could be of type char and y of type u32).
/// - c) No, because the syntax is not correct: it should be struct<U> Coordinates{...}.
/// - d) Yes, because you can use any identifier as a type parameter name. By convention it is often just a letter.
pub fn answer_02() -> char {
    todo!()
}

/// ## Question 3
///
/// Is the below code valid:
/// ```notest
/// struct Coordinates<T,U> {
///     x: T,
///     y: U,
/// }
/// fn main(){
///  let coord = Coordinates {x: 1.0, y: 2.0};
/// }
/// ```
/// - a) No, because the x and y fields should be of different types.
/// - b) Yes, because x and y could have different types, but in this case they have the same type.
/// - c) No, because the syntax is incorrect it should be struct<T,U> Coordinates {...}.
/// - d) Yes, because x and y need to both have the same type.
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// Is the below sentence correct:
/// There is a small runtime cost when using generics, but it is a small price to pay in comparison of their many advantages (code reusability, abstraction, etc.)
///
/// - a) No, because there is no runtime overhead associated when using generic.
/// - b) Yes.
/// - c) No, because the runtime cost depends on the size and the quality of your code.
/// - d) No, because code reusability is not an advantage of generics.
pub fn answer_04() -> char {
    todo!()
}

/// ## Question 5
///
/// What does Monomorphization means?
///
/// - a) the morphization into a monkey (mono = monkey in spanish).
/// - b) the process of removing a single morphic disorder from the runtime.
/// - c) the process of converting generic code into specific code by inserting particular types used during the runtime.
/// - d) the process of converting generic code into specific code by inserting particular types used during compilation.
pub fn answer_05() -> char {
    todo!()
}

/// ## Question 6
///
/// Is this sentence correct: There is absolutely no cost of using generics.
/// - a) Yes, because there is no compilation cost for using generics.
/// - b) No, because even though there is no runtime cost it can increase compilation time and binary size.
/// - c) Yes, because using generic makes the runtime less costly than without.
/// - d) No, because the runtime cost can be quite substantial.
pub fn answer_06() -> char {
    todo!()
}

/// ## Question 7
///
/// What is a trait in Rust?
/// - a) a way to declare methods that can be implemented by different types
/// - b) a distinguishing quality exhibited by most rustaceans
/// - c) a way to ensure that references are valid as long as they are needed
/// - d) an interface between the code and the rust compiler
pub fn answer_07() -> char {
    todo!()
}

/// ## Question 8
///
/// Is the below sentence correct:
/// A trait can be implement on a type only if either the trait or the type is local to our crate
/// - a) Yes
/// - b) No, because the trait and the type must be local to our crate
/// - c) No, only the trait must be in scope
/// - d) No, only the type must be local to our crate
pub fn answer_08() -> char {
    todo!()
}

/// ## Question 9
///
/// What does the orphan rule ensures in Rust:
/// - a) that you can't break other people's code and vice versa
/// - b) that each variable has one owner at a time
/// - c) that an item can be accessed only by its immediate cousins if it has no parent
/// - d) a way to propagates error to the calling code
pub fn answer_09() -> char {
    todo!()
}

/// ## Question 10
/// Is this sentence correct: 
/// Lifetime are implicit and inferred. Their primary objective is to ensure that a value goes out of scope at the right time.
/// - a) Yes
/// - b) No, their primary objective is to prevent dangling references.
/// - c) No, lifetime are not inferred.
/// - d) No, lifetimes ensure that a value is properly dropped when the owner goes out of scope
pub fn answer_10() -> char {
    todo!()
}

/// ## Question 11
///
/// borrow checker
pub fn answer_11() -> char {
    todo!()
}

/// ## Question 12
///
/// Is this sentence correct: 
/// By annotating a lifetime we change how long our reference live thus helping the borrow checker perform its analysis.
/// - a) 
/// - b) 
/// - c) 
/// - d) 
pub fn answer_12() -> char {
    todo!()
}

/// ## Question 13
/// the liftime annotations are enforce by the borro checker 
/// 
pub fn answer_13() -> char {
    todo!()
}

/// ## Question 14
/// the liftime annotations are enforce by the borro checker 
/// 
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
