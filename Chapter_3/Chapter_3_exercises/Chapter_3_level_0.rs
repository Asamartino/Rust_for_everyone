//! # Multiple Choice Questions
//!
//!
//!
/////////////////////////////////////////////////////////////////////////////////////////////////
//////                         Variables, mutability and shadowing                         //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// complete the below function by shadowing the x variable. This function should return the value 6.
pub fn return_six_shadowing() -> u32 {
    let x = 5;
    // add your 1-liner code here
    x
}

// complete the below code. This function should return the value 6.
pub fn return_six_mutability() -> u32 {
    let mut x = 5;
    // add your 1-liner code here
    x
}

// complete the below code. This function should return the char '6' by shadowing x
pub fn return_six_char() -> char {
    let x = 5;
    // add your 1-liner code here
    x
}
/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Data Types                                      //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// This function should return a tuple with the value (42, 2.0, 'c')
pub fn return_tuple() -> (u32, f32, char) {
    // 1. create a tuple
    // 2. return the tuple
    todo!()
}

// reverse and destructuring, tuple of tuple

// This function should return the u32 value of a tuple (u64, f64, char, u32, f64, i64)
pub fn return_u32_tuple(tup: (u64, f64, char, u32, f64, i64)) -> u32 {
    // 1. create a tuple
    // 2. return the tuple
    todo!()
}

// This function creates and array of length 5 with u32 values 
pub fn return_array_len_5() -> [u32;5]{
    todo!()
}

// This function return an array by taking the first and third and fifth element of an array of length five 
pub fn return_array_len_5(my_array: [u32;5]) -> [u32;3]{
    // get the first element, third element and fifth element
    // create a new array and return it
    todo!()
}


