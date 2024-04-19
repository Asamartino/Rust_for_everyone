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
/// - d) Yes, number_of_socks hold the value Some<7>.
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
/// - d) No, 
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
