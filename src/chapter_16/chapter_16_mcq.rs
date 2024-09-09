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
/// What is a thread in Rust?
///
/// - a) a bug that happen in a deadlock scena
/// - b) a unit of execution within a program that allows you to run multiple independent parts concurrently
/// - c) a type of hair style
/// - d) another name for the M:N model
pub fn answer_01() -> char {
    todo!()
}

/// ## Question 2
///
/// What does blocking a thread means?
///
/// - a) that the thread is prevented from continuing its work
/// - b) that the thread is prevented from continuing its work or exiting
/// - c) that the thread is prevented from exiting
/// - d) that the thread immediatly exits
pub fn answer_02() -> char {
    todo!()
}

/// ## Question 3
///
/// How does the `move` keyword affect the behavior of closures when used in conjunction with thread::spawn in Rust?
///
/// - a) it allows the closure to move faster
/// - b) it force the thread to move its data from the stack to the heap
/// - c) it force the closure to take ownership of the values it uses
/// - d) it enables to closure to not run at the same time than the thread
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// Whn does a channel is said to be closed
///
/// - a) when the transmitter is dropped
/// - b) when the receiver is dropped
/// - c) when either the transmitter or the receiver is dropped
/// - d) when it feels drained
pub fn answer_04() -> char {
    todo!()
}

/// ## Question 5
///
/// What is a Mutex and what does it do?
///
/// - a) Mutex is an abbreviation for mutual exclusive and it grants access to the data to just one thread at a time
/// - b) Mutex is an abbreviation for mutual exclusion and it grants access to the data to just one thread at a time
/// - c) Mutex is an abbreviation for mutual exclusion and it grants access to the data to multiple thread at once
/// - d) Mutex is an abbreviation for mutual exclusive and it grants access to the date to multiple thread at once
pub fn answer_05() -> char {
    todo!()
}

/// ## Question 6
///
/// When could you need to use the dereference operator *?
///
/// - a)
/// - b)
/// - c)
/// - d)
pub fn answer_06() -> char {
    todo!()
}

/// ## Question 7
///
/// Select the wrong answer to the question: Why do you need the Deref trait?
///
/// - a)
/// - b)
/// - c)
/// - d)
pub fn answer_07() -> char {
    todo!()
}

/// ## Question 8
///
/// What is Deref coercion?
///
/// - a)
/// - b)
/// - c)
/// - d)
pub fn answer_08() -> char {
    todo!()
}

/// ## Question 9
///
/// Is the below sentence correct?
/// The number of times that Deref::deref needs to be inserted will be resolved at compile time, so there is no runtime penalty for using deref coercion.
///
/// - a)
/// - b)
/// - c)
/// - d)
pub fn answer_09() -> char {
    todo!()
}

/// ## Question 10
///
/// What does the Drop trait allow you to do?
///
/// - a)
/// - b)
/// - c)
/// - d)
pub fn answer_10() -> char {
    todo!()
}

/// ## Question 11
///
/// What will the following code produce in the terminal?
/// ```notest
///     struct CustomSmartPointer{
///            number: i32,
///     }
///     impl Drop for CustomSmartPointer{
///         fn drop(&mut self){
///             print!("Dropping {} ", self.number);
///         }
///     }
///     fn main(){
///         let _one = CustomSmartPointer{number: 1};
///         let _two = CustomSmartPointer{number: 2};
///     }
/// ```
///
/// - a)
/// - b)
/// - c)
/// - d)
pub fn answer_11() -> char {
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
