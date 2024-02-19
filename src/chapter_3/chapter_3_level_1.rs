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


// reverse the array provided, f.i. [1,2,3,4,5] -> [5,4,3,2,1]
pub fn reverse_array(arr: [u32; 5]) -> [u32; 5] {
    todo!()
}

// this function will take two arrays and add the corresponding element together
// e.g. [1,2,3] + [10,10,10] -> [11,12,13]
pub fn add_array(arr1: [u32;3], arr2: [u32;3]) -> [u32;3]{
    todo!()
}

// this function will take two arrays and substract the corresponding element together
// e.g. [1,2,3] + [10,10,10] -> [-9,-8,-7]
pub fn substract_array(arr1: [u32;3], arr2: [u32;3]) -> [i32;3]{
    todo!()
}

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Else if                                         //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// //conditional with array

// this function returns the  print if the number is divisible by 2, 3, 4, 5, the first part of this function is provide
pub fn is_divisible_by_two_three_five(number: u32) -> u32 {
    // if number % 2 == 0 {
    //     println!("the number is divisble by two")
    // }
    todo!()
}

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Loops                                           //////
/////////////////////////////////////////////////////////////////////////////////////////////////

//sort an array from small to big


// sort an array of u32 from bigger to smaller


// this function return an array where every element of the array provided as argument is incremented by 10. This time use a for loop.
pub fn for_array_add_10(arr: [u32; 5]) -> [u32; 5] {
    todo!()
}

// reverse the array provided, f.i. [1,2,3,4,5] -> [5,4,3,2,1]. This time use a for loop.
pub fn for_reverse_array(arr: [u32; 5]) -> [u32; 5] {
    todo!()
}



#[cfg(test)]
mod tests {
    use super::*;

    // Data Types
    #[test]
    fn test_reverse_tuple() {
        let tup: (u32,u32,u32,u32) = (4,5,6,7);
        let response:  (u32,u32,u32,u32) = (7,6,5,4);
        assert_eq!(response, reverse_tuple(tup));
    }

    #[test]
    fn test_reverse_tuple_destructuring() {
        let tup: (u32,u32,u32,u32) = (4,5,6,7);
        let response:  (u32,u32,u32,u32) = (7,6,5,4);
        assert_eq!(response, reverse_tuple(tup));
    }

    #[test]
    fn test_array_add_10() {
        let arr: [u32;5] = [0,1,2,3,4];
        let response: [u32;5] = [10,11,12,13,14];
        assert_eq!(response, array_add_10(arr));
    }


    #[test]
    fn test_reverse_array() {
        let arr: [u32;5] = [6,7,8,9,10];
        let response: [u32;5] = [10,9,8,7,6];
        assert_eq!(response, reverse_array(arr));
    }

    #[test]
    fn test_add_array() {
        let arr1: [u32;3] = [1,2,3];
        let arr2: [u32;3] = [10,10,10];
        let response: [u32;3] = [11,12,13];
        assert_eq!(response, add_array(arr1,arr2));
    }

    #[test]
    fn test_substract_array() {
        let arr1: [u32;3] = [1,2,3];
        let arr2: [u32;3] = [10,10,10];
        let response: [i32;3] = [-9,-8,-7];
        assert_eq!(response, substract_array(arr1,arr2));
    }


    #[test]
    fn test_for_array_add_10() {
        let arr: [u32;5] = [0,1,2,3,4];
        let response: [u32;5] = [10,11,12,13,14];
        assert_eq!(response, for_array_add_10(arr));
    }


    #[test]
    fn test_for_reverse_array() {
        let arr: [u32;5] = [6,7,8,9,10];
        let response: [u32;5] = [10,9,8,7,6];
        assert_eq!(response, for_reverse_array(arr));
    }
}
