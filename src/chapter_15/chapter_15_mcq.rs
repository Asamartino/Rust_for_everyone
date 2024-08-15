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
/// What is a pointer?
///
/// - a) a variable that stores another variable value
/// - b) a variable that stores a memory address, directing/pointing to other data
/// - c) a data point
/// - d) a computer way of emphasizing something on your screen
pub fn answer_01() -> char {
    todo!()
}


/// ## Question 2
///
/// What is a smart pointer?
///
/// - a) a smart way to launch the ball in bowling
/// - b) a pointer having less information so it can process it faster
/// - c) a reference that directs you to a specific location
/// - d) a variable that stores a memory address and have added metadata and functionalities
pub fn answer_02() -> char {
    todo!()
}

/// ## Question 3
///
/// Compared to a struct what trait does a smart pointers typically implements?
/// - a) Deref and Drop
/// - b) Deref and From
/// - c) Into and From
/// - d) Drop and From
pub fn answer_03() -> char {
    todo!()
}

/// ## Question 4
///
/// Complete the sentence: When a Box<T> gets out of scope, ...
///
/// - a) the deallocation happens only for the box (stored on the stack)
/// - b) the deallocation happens only for the data it points to (stored on the heap)
/// - c) the deallocation happens for the box (stored on the stack) and the data it points to (stored on the heap)
/// - d) the deallocation happens only when the program finishes running and the operating system reclaims the memory that was allocated during its execution
pub fn answer_04() -> char {
    todo!()
}

/// ## Question 5
///
/// Is this sentence correct: A pointer size doesn't change based on the amount of data it's point to?
///
/// - a) No, the pointer size is fixed within a particular addressable range but will vary depending on the range
/// - b) No, the pointer size change with each change on the amount of data it's point to 
/// - c) No, that only occurs with smart pointers
/// - d) Yes 
pub fn answer_05() -> char {
    todo!()
}

/// ## Question 6
///
/// When could you need to use the dereference operator *?
///
/// - a) when you need to access the data pointed by the reference  
/// - b) when you need to multiply two values
/// - c) when you need to build a smart pointer
/// - d) when you need to implement the Deref trait
pub fn answer_06() -> char {
    todo!()
}

/// ## Question 7
///
/// Select the wrong answer to the question: Why do you need the Deref trait?
/// 
/// - a) Otherwise, the compiler can only dereference &references 
/// - b) To provide seamless access to the underlying data that the smart pointer manages
/// - c) To move the ownership of the data from the smart pointer to a regular reference
/// - d) To grant the compiler the capacity to use the deref method to get an &reference, that can be easily dereferenced
pub fn answer_07() -> char {
    todo!()
}

/// ## Question 8
///
/// What is Deref coercion?
/// 
/// - a) it converts a type of reference into another type reference to a type that Deref can convert the original type into.
/// - b) it allows the compiler to automatically convert a reference to a type into a reference to another type, provided that the original type implements the Deref trait
/// - c) it coerce a reference to implement Deref
/// - d) it converts a reference to a type that implements Deref into a dereference
pub fn answer_08() -> char {
    todo!()
}

/// ## Question 9
///
/// Is the below sentence correct?
/// The number of times that Deref::deref needs to be inserted will be resolved at compile time, so there is no runtime penalty for using deref coercion. 
/// 
/// - a) No, there is a limit set to 100 to the number of times that Deref::deref needs to be inserted to avoid an infinity loop
/// - b) No, there is a runtime penalty for using the deref method
/// - c) No, the number of times that Deref::deref needs to be inserted will be resolved at runtime.
/// - d) Yes
pub fn answer_09() -> char {
    todo!()
}

/// ## Question 10
///
/// What does the Drop trait allow you to do? 
/// 
/// - a) to omit the dereference operator
/// - b) to define a custom behavior when a value is about to be dropped
/// - c) to customize what happens when a value come into scope
/// - d) to specify when a value goes out of scope
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
/// - a) Dropping one Dropping two
/// - b) Dropping Two Dropping one
/// - c) A compilation error as the Drop trait is not included in the prelude and needs to be imported
/// - d) Nothing, as the drop method needs to be call explicitly
pub fn answer_11() -> char {
    todo!()
}

/// ## Question 12
/// Is the below sentence correct:
///     By using the std::mem::drop function you can clean up a value early. However you need to be carful from accidentally cleaning up value still in use.
/// - a) Yes, this is why you should use it only when you are confident in your understanding of its implications and are certain about your actions
/// - b) No, as the std::mem::drop function is similar to the drop method in the Drop trait
/// - c) No, as the ownership system will make sure that the drop method can only be called once the value is no longer needed 
/// - d) Yes, this makes cleanup convenient and safe
pub fn answer_12() -> char {
    todo!()
}

/// ## Question 13
///
/// What will the following code produce in the terminal?
/// ```notest
///     (0..5).map(|x| println!("{x}"));
/// ```
/// - a) 
/// - b)
/// - c) 
/// - d) 
pub fn answer_13() -> char {
    todo!()
}

/// ## Question 14
///
/// What does the below code produce?
/// ```notest
///     let infinite_iterator = 0..;
/// ```
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
