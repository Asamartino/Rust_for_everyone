//! # Multiple Choice Questions
//!
//! ** As this section introduce several conceme concepts This portion of the exam is to be completed without accessing the internet, books, the rust
//! compiler, or any other resources.**
//!
//!  Replace the todo!() macros with your answer e.g.:
//! 
//! Question 0
//! 
//! Do you want to learn Rust?
//! 
//! - a) yes
//! - b) no
//! - c) what is Rust?
//! - d) I don't know
//! 
//! ```notest
//! pub fn answer_0(){
//!  'a'
//! }
//! ```
//! 
//! There is only one right answer for the multiple choice questions
//! ```sh
//! cargo doc --open
//! ```

/// ## Question 1
///
/// By default variables are:
///
/// - a) mutable
/// - b) immutable
/// - c) methamorphasable
/// - d) made of cheese
pub fn answer_1() -> char {
    todo!()
}

/// ## Question 2
///
/// Consider this code:
///
/// ```notest
/// fn main() {
/// let x = 42;
/// x = 0;
/// }
/// ```
/// What needs to change in order for the above code to compile?
///
/// - a) mut fn main() {
/// - b) let mut x = 42;
/// - c) mut x = 0;
/// - d) nothing, the above code compiles
pub fn answer_2() -> char {
    todo!()
}

/// ## Question 3
///
/// Consider the below code:
///
/// ```notest
/// fn main() {
/// let x = 42;
/// let x = 43;
/// }
/// ```
/// Does the above code compiles, and why?
///
/// - a) yes, because the variable x is mutable
/// - b) no, because by default variables are immutables
/// - c) no, because you can not defined twice a variable with the same name
/// - d) yes, because the first variable is shadowed by the second
pub fn answer_3() -> char {
    todo!()
}

// advantages of shadowing: few transformation and then variable is immutable + change the type of the value

/// ## Question 4
///
/// Rust has two primitive compound types: tuples and arrays. Which statement below is true?
///
/// - a) every element in a tuples and an array must have the same types
/// - b) every element in a tuples and an array can have different types
/// - c) the elements in a tuple can have different type while every element in an array must have the same type
/// - d) the elements in an arrayy can have different type while every element in an tuple must have the same type
pub fn answer_4() -> char {
    todo!()
}

// exemple of vector with array

/// ## Question 5
///
/// Rust has two primitive compound types: tuples and arrays. Which statement below is true?
///
/// - a) every element in a tuple and an array must have the same types
/// - b) every element in a tuple and an array can have different types
/// - c) the elements in a tuple can have different type while every element in an array must have the same type
/// - d) the elements in an arrayy can have different type while every element in an tuple must have the same type
pub fn answer_5() -> char {
    todo!()
}

/// ## Question 6
///
/// What is the difference between a statement and an expression?
///
/// - a) expression return a value while statement do not return values
/// - b) statement return a value while expression do not return values
/// - c) a statement is a synonyme for an expression
/// - d) A statement is a whisper, and an expression is a shout in the programming world.
pub fn answer_6() -> char {
    todo!()
}
// turn this expresion into a statement

/// ## Question 7
///
/// Consider the below code:
///
/// ```notest
/// let forty_two = if true {
///     42
/// } else {
///     "42"
/// }
//
/// ```
/// Does the above code compiles, and why?
///
/// - a) yes, because if is an expression and thus can be use on the right side of a let statement
/// - b) no, you can't use if on the right side of a let statement
/// - c) yes,because if is a statement and thus can be use on the right side of a let statement
/// - d) no, because the value on each arm of the if must have the same type
pub fn answer_7() -> char {
    todo!()
}

/// ## Question 8
///
/// What are the three kinds of loops in Rust?
///
/// - a) for, while and do while
/// - b) loop, do while and while
/// - c) loop, while and for
/// - d) for, while and do while
pub fn answer_8() -> char {
    todo!()
}

/// ## Question 9
///
/// One difference between shadowing a variable rather than making it mutable is that :
///
/// - a) shadowing allow you to also change the type of the variable, which is not possible with a mutable variable
/// - b) mutability allow you to also change the type of the variable, which is not possible with a shadowed variable
/// - c) there is no difference
/// - d) you can only shadowed a variable once then it becomes immutable


/// ## Question 10
///
/// Which Rust loop would you use if you first need to check a condtion before entering the loop
///
/// - a) for
/// - b) while 
/// - c) loop
/// - d) do while