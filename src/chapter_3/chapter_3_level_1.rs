/////////////////////////////////////////////////////////////////////////////////////////////////
//////                         Variables, mutability and shadowing                         //////
/////////////////////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Data Types                                      //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// reverse the tuple provided and return the new tuple, f.i. (1,2,3,4) -> (4,3,2,1)
pub fn reverse_tuple(tup: (u32, u32, u32, u32)) -> (u32, u32, u32, u32) {
    todo!()
}

// reverse the tuple provided using destructuring and return the new tuple using shadowing, , f.i. (1,2,3,4) -> (4,3,2,1)
pub fn reverse_tuple_destructuring(tup: (u32, u32, u32)) -> (u32, u32, u32) {
    let response = tup;
    todo!()
}

// this function return an array where every element of the array provided as argument is incremented by 10
pub fn array_add_10(arr: [u32; 5]) -> [u32; 5] {
    todo!()
}

// reverse the array provided and return the result, f.i. [1,2,3,4,5] -> [5,4,3,2,1]
pub fn reverse_array(arr: [u32; 5]) -> [u32; 5] {
    todo!()
}

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Else if                                         //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// //conditional with array

// this function returns the  print if the number is divisible by 2, 3, 4, 5, the first part of this function is provide
pub fn is_divisible_by_two_three_five(number: u32) -> u32 {
    if number % 2 == 0 {
        println!("the number is divisble by two")
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Loops                                           //////
/////////////////////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;

    // Data Types
    #[test]
    fn test_reverse_tuple() {
        let tup: (u32,u32,u32,u32) = (4,5,6,7);
        assert_eq!(tup.rev(), reverse_tuple(tup));
    }

    #[test]
    fn test_reverse_tuple_destructuring() {
        let tup: (u32,u32,u32,u32) = (4,5,6,7);
        assert_eq!(tup.rev(), reverse_tuple(tup));
    }

    #[test]
    fn test_array_add_10() {
        let arr: [u32;5] = [0,1,2,3,4];
        let response: [u32;5] = [10,11,12,13,14];
        assert_eq!(response,_array_add_10(arr));
    }

}
