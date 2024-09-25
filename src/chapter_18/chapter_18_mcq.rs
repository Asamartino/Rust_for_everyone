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
/// What is a pattern in Rust?
///
/// - a) a regularity between the names of the variables in Rust
/// - b) a unique syntax for matching the structure of memory allocation strategies
/// - c) a unique syntax for matching the structure of types
/// - d) a regular arrangement in the Rust compiler ensuring efficient memory management
pub fn answer_01() -> char {
    todo!()
}

/// ## Question 2
///
/// Complete the sentence: match expressions need to be ...?
///
/// - a) exhausting, meaning that every wrong value takes a lot of compilation time
/// - b) exhaustive, meaning that every possible value in the match expression need to be handled
/// - c) extensive, meaning that most common value in the match expression must be considered
/// - d) exhilarate, meaning that it will make the compiler happy
pub fn answer_02() -> char {
    todo!()
}

/// ## Question 3
///
/// What is one disadvantage of using if let expressions
///
/// - a) exhaustiveness is enforced by the compiler
/// - b) it has less flexibility than a match expression
/// - c) exhaustiveness is not enforced by the compiler
/// - d) it has no disadvantages
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// Complete the sentence: A variable name is basically a simple instance of a ...?
///
/// - a) tuple
/// - b) iterator
/// - c) pattern
/// - d) match arm
pub fn answer_04() -> char {
    todo!()
}

/// ## Question 5
///
/// What happens to the value 3 in the below code?
/// ```notest
///     let (x,y,_) = (1,2,3);
/// ```
///
/// - a) the value 3 will be stored in the tuple (x,y)
/// - b) the value 3 is stored in _
/// - c) the value 3 will be stored in the tuple (x,y,z)
/// - d) the value 3 is ignored and not stored anywhere
pub fn answer_05() -> char {
    todo!()
}

/// ## Question 6
///
/// Will the below code compile and why?
/// ```notest
///     if let x = 42 {
///      println!("The answer to the ultimate question of life, the universe, and everything is: {}", x )
///     }
/// ```
///
/// - a) No because, 42 is not the answer to the ultimate question of life, the universe, and everything
/// - b) No because you use if let with a refutable pattern
/// - c) Yes
/// - d) No, because you use if let with an irrefutable pattern
pub fn answer_06() -> char {
    todo!()
}

/// ## Question 7
///
/// What line will appear on the terminal if the following code is executed?
/// 
/// ```notest
///     let a = Some(1);
///     let b = 2;
///     let c = false;
/// 
///     match a {
///         Some(2) => println!("Ma-ia-hii"),
///         Some(b) => println!("Ma-ia-huu"),
///         c => println!("Ma-ia-hoo"),
///     }
/// ```
///
/// - a) Ma-ia-hii
/// - b) Ma-ia-huu
/// - c) Ma-ia-hoo
/// - d) Ma-ia-haa-haa
pub fn answer_07() -> char {
    todo!()
}

/// ## Question 8
///
/// Is the below sentence correct?
/// The destructuring pattern must correspond to how the enum’s data is structured.
///
/// - a) No, the destructuring pattern doesn't need to correspond to how the enum's data is structured
/// - b) Yes
/// - c) No, the destructuring pattern must correspond to how the struct’s data is structured
/// - d) No, the destructuring pattern must correspond to how the match arm is structured
pub fn answer_08() -> char {
    todo!()
}

/// ## Question 9
///
/// Will the below code compile and why?
/// ```notest
///    let s = Some(String::from("You want"));
///
///    if let Some(_s) = s{
///        println!("Here is the interior variable {}", _s);
///    }
///
///    println!("the variable s is {:?}", s)
/// ```
///
/// - a) Yes, both lines will be printed in the terminal
/// - b) No, because s have been defined as immutable
/// - c) No, because the value s will be shadowed by _s 
/// - d) No, because the value s will be moved into _s preventing it to be used again
pub fn answer_09() -> char {
    todo!()
}

/// ## Question 10
///
/// Will the below code compile and why?
/// ```notest
///    let s = Some(String::from("You want"));
///
///    if let Some(_) = s{
///        println!("s has something");
///    }
///
///    println!("the variable s is {:?}", s)
/// ```
///
/// - a) No, because the value s will be shadowed by _
/// - b) Yes, both lines will be printed in the terminal
/// - c) No, because s have been defined as immutable
/// - d) No, because the value s will be moved into _ preventing it to be used again
pub fn answer_10() -> char {
    todo!()
}


/// ## Question 11
///
/// Why is the ref keyword used in a match pattern?
/// 
/// - a) to refute a pattern
/// - b) to create a reference in a pattern
/// - c) to create a reflection of a match arm
/// - d) to redirect one match arm to another
pub fn answer_11() -> char {
    todo!()
}


/// ## Question 12
///
///  Which operator in Rust allows you to bind a value to a variable while also matching a pattern?
/// 
/// - a) @
/// - b) &
/// - c) |
/// - d) ref
pub fn answer_12() -> char {
    todo!()
}



/// ## Question 13
///
///
/// - a) 
/// - b) 
/// - c) 
/// - d) 
pub fn answer_13() -> char {
    todo!()
}



/// ## Question 14
///
///
/// - a) 
/// - b) 
/// - c) 
/// - d) 
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
}
