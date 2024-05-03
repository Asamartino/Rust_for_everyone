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
/// When should you use an Enums in Rust?
///
/// - a) Whenever you want.
/// - b) Only when you run out of other options.
/// - c) To define a type by enumerating all possible values.
/// - d) To define a type with an infinite amount of potential values.
pub fn answer_01() -> char {
    todo!()
}

/// ## Question 2
///
/// What is refer to as the variants of an enum?
///
/// - a) The possible value of an enum.
/// - b) The name of the enum.
/// - c) The method associated with the enum
/// - d) Enum have no variants, the correct term is fields
pub fn answer_02() -> char {
    todo!()
}

/// ## Question 3
///
/// Which answer below is correct regarding: why is the Option type used in Rust?
///  
/// - a) The compiler ensures that we check for both the presence and absence of the input before processing it further. This assists in identifying a common pitfall with null: mistakenly assuming that a variable holds a non-null value when it is null.
/// - b) Because, to allow for the possibility of a null value, you need to deliberately opt-in by assigning the type of that value as Option<T>. Consequently, whenever you interact with this value, you are compelled to handle the possibility of it being null in your code explicitly. Thus, limiting null pervasiveness.
/// - c) Because it indicates the presence or absence of a value, thus denoting its optional nature. By articulating this notion within the type system, the compiler can ascertain if you've adequately addressed all the expected scenarios.
/// - d) All of the answers are correct.
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// Is this sentence correct:
/// "The Option<T> enum is incredibly handy; it's even part of the prelude, so there's no need to explicitly import it into scope.
/// Moreover, its variants are also in the prelude. You can utilize Some and None directly without needing to prefix them with Option::"
///
/// - a) No, Option<T> it's not part of the prelude, so you need to explicitly import it into scope.
/// - b) Yes
/// - c) No, you can't utilize Some and None directly, you need to prefix Some and None with Option::
/// - d) No, Option<T> is part of the prelude but its variants are not, so you need to imports them into scope.
pub fn answer_04() -> char {
    todo!()
}

/// ## Question 5
///
/// Which sentence below is incorrect regarding match?
///
/// - a) It offers exhaustive pattern matching, ensuring all possible cases are handled explicitly.
/// - b) It enables the comparison of a value against various patterns, executing specific corresponding code based on the matched pattern.
/// - c) Allow you to bind parts of the value that match the pattern, enabling the extraction of values from enum variants.
/// - d) It is similar to an if statement, but the compiler ensures that we check for both the true and false input before processing further.
pub fn answer_05() -> char {
    todo!()
}

/// ## Question 6
///
/// Consider the code:
///
/// ```notest
/// let score = 10;
/// match score {
///     0 => println!("It's zero!"),
///     1 => println!("It's one!"),
///     2..=15=> println!("It's between 2 and 15!"),
///     10..=20 => println!("It's between 10 or 20!"),
///     _ => println!("It's something else!"),
/// }
/// ```
/// Does the above code compiles, and what does it print to the console?
///
/// - a) Yes, and it only prints "It's between 2 and 15!". 
/// - b) Yes, and it prints "It's between 2 and 15!" and then "It's between 10 or 20!".
/// - c) Yes, and it prints "It's one!".
/// - d) No, it doesn't compile.
///
pub fn answer_06() -> char {
    todo!()
}

/// ## Question 7
///
///
/// /// Consider the code:
///
/// ```notest
/// fn plus_two(value: Option<u32>) -> Option<u32>{
///     match value{
///         None => None,
///         Some(x) => Some(x+2)
///     }
/// }
///
/// let number_of_socks = plus_two(5);
/// ```
/// Does the above code compiles, what values does number_of_socks hold?
///
/// - a) Yes, number_of_socks hold the value 7.
/// - b) No, because the plus_two function expects an Option<u32>
/// - c) Yes, number_of_socks hold the value Option<7>.
/// - d) Yes, number_of_socks hold the value Some(7).
pub fn answer_07() -> char {
    todo!()
}

/// ## Question 8
///
/// Does the following sentence is correct ?
///
/// "Matches are exhaustive. Moreover, the first arm that matches the value being matched will be executed, and no other arms will be evaluated"
///
/// - a) Yes
/// - b) No, because every arm that match the pattern will be evaluated
/// - c) No, match is not exhaustive
/// - d) No, only the last arm will be executed
pub fn answer_08() -> char {
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

}
