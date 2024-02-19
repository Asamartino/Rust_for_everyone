//! # Multiple Choice Questions
//!
//!
//!
/////////////////////////////////////////////////////////////////////////////////////////////////
//////                         Variables, mutability and shadowing                         //////
/////////////////////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Data Types                                      //////
/////////////////////////////////////////////////////////////////////////////////////////////////


//this function will take the middle value of an array of number and convert it to a floating number
// e.g. [1,2,3] -> 2.0
// you could use shadowing
pub fn array_middle_value(arr: [u32;3]) -> f64 {
    todo!()
}

// convert tuple to array and round the floating number to closest integer

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Else if                                         //////
/////////////////////////////////////////////////////////////////////////////////////////////////

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
// combine array and for loop to reverse the array and add 5 to every element

// this functions check if the number provided is a prime number (i.e. number greater than one and that is not obtained by the multiplaction of smaller numbers)
// tip: you could use a for loop and the modulo operator
pub fn is_prime(number: u32) -> bool {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_middle_value() {
        let arr: [u32;3]= [4,5,6];
        let response: f64 = 5.0;
        assert_eq!(response, array_middle_value(arr));
    }
}