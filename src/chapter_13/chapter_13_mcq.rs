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
/// What is a closure in Rust?
/// - a) closures are anonymous functions, that can access variables from the scope in which they are defined
/// - b) a way to capture and protect variable making them feel safe
/// - c) closures are known functions, that can capture values from the scope in which they are called
/// - d) closures are anonymous functions, that can not capture values from the scope in which they are called
pub fn answer_01() -> char {
    todo!()
}

/// ## Question 2
///
/// What is a good use case for a closure in Rust?
/// - a) to make your code look more professional
/// - b) to encapsulate all the environment variable
/// - c) to decapsulate all teh environment variable
/// - d) to write a function in one place and only execute it if needed
pub fn answer_02() -> char {
    todo!()
}

/// ## Question 3
///
/// Considering the below code, what does adding contain ?
/// ```notest
/// let adding = |num1, num2| num1+num2;
/// ```
///
/// - a) the result of num1+num2
/// - b) the definition of an anonymous function
/// - c) the definition of a known function
/// - d) the result of the absolute value of num1 and num2 added together
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// Complete the below sentence:
/// Closure do not require you to annotate the types of the the parameters. ...
///
/// - a) The compiler infers the type every time the closure is called. Thus, the same closure could be used with different types.
/// - b)
/// - c) However, once inferred they are fixed and we could get a type error by using the closure with another type.
/// - d) However, you can only call a closure once.
pub fn answer_04() -> char {
    todo!()
}

/// ## Question 5
///
/// Complete the below sentence:
/// Closure do not require you to annotate the types of the the parameters. ...
///
/// - a) The compiler infers the type every time the closure is called. Thus, the same closure could be used with different types.
/// - b) This is known as memoization.
/// - c) However, once inferred they are fixed and we could get a type error by using the closure with another type.
/// - d) This is known as congolexitimization.
pub fn answer_05() -> char {
    todo!()
}

/// ## Question 6
///
/// Is the below sentence correct?
/// Closure can capture a value from its environment. This process cause a memory overhead.
///
/// - a) Yes.
/// - b) No, thanks to Rust optimization it doesn't incur any additional overhead.
/// - c) No, it does not cause a memory overhead but reduce runtime performances.
/// - d) No, it cause a memoization overhead.
pub fn answer_06() -> char {
    todo!()
}

/// ## Question 7
///
/// Considering the below code, which trait are implement by the closure?
/// ```notest
/// let forty_two = 42;
/// let is_equal_42 = |x| x == forty_two;
/// ```
///
/// - a) Fn and FnOnce
/// - b) Fn and FnMut
/// - c) FnMut and FnOnce
/// - d) FnMut and FnTwice
pub fn answer_07() -> char {
    todo!()
}

/// ## Question 8
///
/// In Rust, an iterator:
///
/// - a) enables you to traverse all your files faster
/// - b) is responsible for the logic of iterating over many different kind of sequences.
/// - c) a powerful abstraction that allows for lazy evaluation, enabling efficient processing of large datasets without consuming excessive memory.
/// - d) autonomously optimize code performance by predicting future data access patterns.
pub fn answer_08() -> char {
    todo!()
}

/// ## Question 9
///
/// In Rust, iterators are:
///
/// - a) busy
/// - b) energetic
/// - c) lazy
/// - d) confused
pub fn answer_09() -> char {
    todo!()
}

/// ## Question 10
///
/// What is an iterator adaptor?
///
/// - a) a function that takes an iterator and return another iterator
/// - b) a function that takes an iterator and consumes it
/// - c) a way to turn an iterator into a closure
/// - d) a way to convert US iterators to their EU counterpart
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
