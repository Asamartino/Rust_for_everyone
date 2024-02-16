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
// note that you can't change the type variable if you declare it as mutable
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
pub fn return_array_len_5() -> [u32; 5] {
    todo!()
}

// This function return an array by taking the first, third, and fifth element of the parameter (array of lenght 5) given to to this function
pub fn return_array_len_5(my_array: [u32; 5]) -> [u32; 3] {
    // get the first element, third element and fifth element
    // create a new array and return it
    todo!()
}

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Else if                                         //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// This function return true if the number is even and false otherwise
// Complete this function using else if conditions
pub fn is_even(number: u32) -> bool {
    // to determine if a number is even you can use % (the modulo operator)
    todo!()
}

// this function returns the maximum values between the two numbers provided. If the numbers are equal this function return the number
// Complete this function using else if conditions
pub fn return_maximum(n1: u32, n2: u32) -> u32 {
    todo!()
}

// this function returns the maximum values between the two numbers provided. If the numbers are equal this function return the number
// This time complete this fnction using if on the right side of a let statment
pub fn return_maximum_if_let(n1: u32, n2: u32) -> u32 {
    todo!()
}

// //conditional with array
// // this function print if the number is divisible by 2, 3 and 5, the first part of this function is provide
// pub fn print_divisible_by_two_three_five(number: u32){
//     if number % 2 == 0{
//         println!("the number is divisble by two")
//     }
// }

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Loops                                           //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// In rust there is three types of loops:
//      - loop: creates an infinite loop that can be stoped by using a break statement
//      - while: will run till the condition is true
//      - for: is a more concise a safe way

// Complete the below function by incrementing number by 1 using loop and breaking it once the number 100 is reached.
pub fn return_hunderd() -> u32 {
    let mut number = 0;
    todo!()
    // uncomment the code below and complete the loop
    // loop {
    //     // complete this part of the loop
    //     // add an if clause that return break if the number 100 is reached
    // }
    // number
}

// Complete the code below by using a while loop that will add 1 to number and index, 25 times in total
pub fn increment_by_25(number: u32) -> (u32, u32) {
    let mut response = number;
    let mut index = 0;
    todo!()
    // (response,index)
}

// Complete the below function, that will add all numbers together in the array provided using a for loop
pub fn add_elements_together(arr: [u32; 10]) -> u32 {
    let mut response = 0;
    // put your for loop here
    todo!()
    // response
}
