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

/// ## Question 6
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

/// ## Question 13
///
/// The default lifetime of a trait object is ...?
///
/// - a) 'dynamic
/// - b) 'a
/// - c) 'static
/// - d) 'lexical
pub fn answer_13() -> char {
    todo!()
}

/// ## Question 14
///
/// What is an associated type ?
///
/// - a) a type that performs certain activities or services on behalf of another type
/// - b) an entry-level or mid-level position within the type organization
/// - c) another word for generic
/// - d) a feature of traits that allow you to define types that are related to the trait, but are determined by the implementer of the trait
pub fn answer_14() -> char {
    todo!()
}

/// ## Question 15
///
/// Consider the below code:
///
///  ```notest
/// trait Pilot {
///     fn say_hi(&self);
/// }
/// trait Patrick {
///     fn say_hi(&self);
/// }
/// trait Kenobi{
///     fn say_hi(&self);
/// }
/// struct Human;
/// impl Pilot for Human {
///     fn say_hi(&self) {
///         println!("Hello, this is your captain speaking.");
///      }
///  }
/// impl Patrick for Human {
///     fn say_hi(&self) {
///         println!("Hello?");
///     }
/// }
/// impl Human {
///     fn say_hi(&self) {
///         println!("Hi, I'm human");
///     }
/// }
/// impl Kenobi for Human{
///     fn say_hi(&self) {
///         println!("Hello, there");
///     }
/// }
///
/// ```
///
/// Assume you create the variable: let human = Human.
/// How can you call the say_hi method from Human which will result in "Hi, I'm human" in the terminal
///
/// - a) Pilot::say_hi(&human)
/// - b) Patrick::say_hi(&human)
/// - c) Human::say_hi(&human)
/// - d) Kenobi::say_hi(&human)
pub fn answer_15() -> char {
    todo!()
}

/// ## Question 16
///
/// How is defined the fully qualified syntax:
///
/// - a) <Trait as Type>::function(next_arg, ...);
/// - b) <Type as Trait>::function(next_arg, ...);
/// - c) <Trait as Type>::function(receiver_if_method, next_arg, ...);
/// - d) <Type as Trait>::function(receiver_if_method, next_arg, ...);
pub fn answer_16() -> char {
    todo!()
}

/// ## Question 17
///
/// What is a supertrait?
///
/// - a) a very powerful trait (e.g. Display)
/// - b) a trait that another trait depends on, meaning that one trait is a superset of another trait
/// - c) a trait use in a fully qualified syntax
/// - d) a trait very well written
pub fn answer_17() -> char {
    todo!()
}

/// ## Question 18
///
/// What is a new type pattern?
///
/// - a) a new type as the name suggest
/// - b) a design pattern where a new type is created as supertrait of an existing type
/// - c) a design pattern where a new type is created as a wrapper around an existing type
/// - d) a new type defined as unsafe
pub fn answer_18() -> char {
    todo!()
}

/// ## Question 19
///
/// What is the main use case of type alias?
///
/// - a) to give an affectuous name to your favorite variable
/// - b) to conceal the type of a variable so that only the compiler knows it
/// - c) to increase the speed of the compiler
/// - d) to reduce repetition
pub fn answer_19() -> char {
    todo!()
}

/// ## Question 20
///
/// what does the ! type (aka never type) represents ?
///
/// - a) the type of computations which never resolve to any value at all
/// - b) the type of computations that never compiles
/// - c) the type of computations that never ends
/// - d) the type of computations that is not used anymore (comes from legacy Rust)
pub fn answer_20() -> char {
    todo!()
}

/// ## Question 21
///
/// Which of the following does not have a ! value?
///
/// - a) vec![]
/// - b) loop
/// - c) continue
/// - d) panic!
pub fn answer_21() -> char {
    todo!()
}

/// ## Question 22
///
/// What is the golden rule of dynamically size types?
///
/// - a) must always put values of dynamically sized types behind a pointer
/// - b) must always adjust its value during runtime
/// - c) must always determine the type size at compile time
/// - d) must always allocate the same amount of memory to their values
pub fn answer_22() -> char {
    todo!()
}

/// ## Question 23
///
/// What does the special syntax `?Sized` mean?
///
/// - a) that Sized is a function pointer
/// - b) it either unwraps the value and continues execution or return an error
/// - c) is not Sized
/// - d) may or may not be Sized
pub fn answer_23() -> char {
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

    #[test]
    fn answer_19_sanity_check() {
        sanity_check(&answer_19)
    }

    #[test]
    fn answer_20_sanity_check() {
        sanity_check(&answer_20)
    }

    #[test]
    fn answer_21_sanity_check() {
        sanity_check(&answer_21)
    }

    #[test]
    fn answer_22_sanity_check() {
        sanity_check(&answer_22)
    }

    #[test]
    fn answer_23_sanity_check() {
        sanity_check(&answer_23)
    }
}
