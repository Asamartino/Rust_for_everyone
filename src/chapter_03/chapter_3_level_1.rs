/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Data Types                                      //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// this function receive a tuple and return a tuple with the first element being the tuple received as argument
// e.g. tuple_of_tuple(x,y,z) -> ((x,y,z), y, z)
pub fn tuple_of_tuple(tup: (u32, u32, u32)) -> ((u32, u32, u32), u32, u32) {
    todo!()
}

// this function receive a tuple with the first argument being a tuple and return the second element of the first tuple
// e.g tuple_1_2((1,2,3),4,5) -> 2
pub fn tuple_1_2(tup: ((u32, u32, u32), u32, u32)) -> u32 {
    todo!()
}

// reverse the tuple provided and return the new tuple, f.i. (1,2,3,4) -> (4,3,2,1)
pub fn reverse_tuple(tup: (u32, u32, u32, u32)) -> (u32, u32, u32, u32) {
    todo!()
}

// reverse the tuple provided using destructuring and return the new tuple using shadowing, , f.i. (1,2,3) -> (3,2,1)
pub fn reverse_tuple_destructuring(tup: (u32, u32, u32)) -> (u32, u32, u32) {
    let response = tup;
    todo!()
}

// this function return an array where every element of the array provided as argument is incremented by 10
pub fn array_add_10(arr: [u32; 5]) -> [u32; 5] {
    todo!()
}

// this function returns the following array: [[1,2,3], [4,5,6], [7,8,9]]
pub fn array_cube() -> [[u32; 3]; 3] {
    todo!()
}

// this function returns the second element of the second array
// e.g. array_cube_2_2([[1,2,3], [4,5,6], [7,8,9]]) -> 5
pub fn array_cube_2_2(arr: [[u32; 3]; 3]) -> u32 {
    todo!()
}

// reverse the array provided, f.i. [1,2,3,4,5] -> [5,4,3,2,1]
pub fn reverse_array(arr: [u32; 5]) -> [u32; 5] {
    todo!()
}

// this function will take two arrays and add the corresponding element together
// e.g. add_array([1,2,3],[10,10,10]) -> [11,12,13]
pub fn add_array(arr1: [u32; 3], arr2: [u32; 3]) -> [u32; 3] {
    todo!()
}

// this function will take two arrays and substract the corresponding element together
// e.g. subtract_array([1,2,3], [10,10,10]) -> [-9,-8,-7]
pub fn substract_array(arr1: [u32; 3], arr2: [u32; 3]) -> [i32; 3] {
    todo!()
}

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Else if                                         //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// this function return the maximum number using multiple else if conditions
pub fn return_max(num1: u32, num2: u32, num3: u32) -> u32 {
    todo!()
}

// this function return a tuple of bools depending if the number is divisible by two, three or four
// divisible_by_2_3_4(4) -> (true, false,true)
// divisible_by_2_3_4(3) -> (false, true, false)
// divisible_by_2_3_4(24) -> (true, true, true)
// divisible_by_2_3_4(7) -> (false, false, false)
pub fn divisble_by_2_3_4(num: u32) -> (bool, bool, bool) {
    todo!()
}

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Loops                                           //////
/////////////////////////////////////////////////////////////////////////////////////////////////


// Complete the code below by using a while loop that will add 1 to number and index, 25 times in total
// note that this function returns a tuple of the number incremented by 25 and the index
pub fn increment_by_25(number: u32) -> (u32, u32) {
    todo!()
    // uncoment the code below
    // let mut response = number;
    // let mut index = 0;
    // add your code here
    // (response,index)
}

// Complete the below function that will add all numbers together in the array provided using a for loop
pub fn add_elements_together(arr: [u32; 10]) -> u32 {
    todo!()
    // uncoment the code below
    // let mut response = 0;
    // put your for loop here
    // response
}

// this function return an array where every element of the array provided as argument is incremented by 10 using a for loop
pub fn for_array_add_10(arr: [u32; 5]) -> [u32; 5] {
    todo!()
}

// reverse the array provided, f.i. [1,2,3,4,5] -> [5,4,3,2,1] using a for loop.
pub fn for_reverse_array(arr: [u32; 5]) -> [u32; 5] {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    // Data Types
    #[test]
    fn test_tuple_of_tuple() {
        let tup: (u32, u32, u32) = (1, 2, 3);
        let response: ((u32, u32, u32), u32, u32) = ((1, 2, 3), 2, 3);
        assert_eq!(response, tuple_of_tuple(tup));
    }

    #[test]
    fn test_tuple_1_2() {
        let answer: u32 = 5;
        let tup: ((u32, u32, u32), u32, u32) = ((4, answer, 6), 7, 8);
        assert_eq!(answer, tuple_1_2(tup));
    }

    #[test]
    fn test_reverse_tuple() {
        let tup: (u32, u32, u32, u32) = (4, 5, 6, 7);
        let response: (u32, u32, u32, u32) = (7, 6, 5, 4);
        assert_eq!(response, reverse_tuple(tup));
    }

    #[test]
    fn test_reverse_tuple_destructuring() {
        let tup: (u32, u32, u32, u32) = (4, 5, 6, 7);
        let response: (u32, u32, u32, u32) = (7, 6, 5, 4);
        assert_eq!(response, reverse_tuple(tup));
    }

    #[test]
    fn test_array_add_10() {
        let arr: [u32; 5] = [0, 1, 2, 3, 4];
        let response: [u32; 5] = [10, 11, 12, 13, 14];
        assert_eq!(response, array_add_10(arr));
    }

    #[test]
    fn test_array_cube() {
        let response: [[u32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        assert_eq!(response, array_cube());
    }

    #[test]
    fn test_array_cube_2_2() {
        let response: u32 = 5;
        assert_eq!(response, array_cube_2_2(array_cube()));
    }

    #[test]
    fn test_reverse_array() {
        let arr: [u32; 5] = [6, 7, 8, 9, 10];
        let response: [u32; 5] = [10, 9, 8, 7, 6];
        assert_eq!(response, reverse_array(arr));
    }

    #[test]
    fn test_add_array() {
        let arr1: [u32; 3] = [1, 2, 3];
        let arr2: [u32; 3] = [10, 10, 10];
        let response: [u32; 3] = [11, 12, 13];
        assert_eq!(response, add_array(arr1, arr2));
    }

    #[test]
    fn test_substract_array() {
        let arr1: [u32; 3] = [1, 2, 3];
        let arr2: [u32; 3] = [10, 10, 10];
        let response: [i32; 3] = [-9, -8, -7];
        assert_eq!(response, substract_array(arr1, arr2));
    }

    #[test]
    fn test_return_max() {
        let num1: u32 = 100;
        let num2: u32 = 50;
        let num3: u32 = 2500;

        assert_eq!(num3, return_max(num1, num2, num3));
    }

    #[test]
    fn test_return_max_2_similar_value() {
        let num1: u32 = 100;
        let num2: u32 = 250;
        let num3: u32 = 250;

        assert_eq!(num2, return_max(num1, num2, num3));
    }

    #[test]
    fn test_divisible_by_2_3_4_2() {
        let num: u32 = 2;
        let response: (bool, bool, bool) = (true, false, false);
        assert_eq!(response, divisble_by_2_3_4(num));
    }

    #[test]
    fn test_divisible_by_2_3_4_3() {
        let num: u32 = 3;
        let response: (bool, bool, bool) = (false, true, false);
        assert_eq!(response, divisble_by_2_3_4(num));
    }

    #[test]
    fn test_divisible_by_2_3_4_4() {
        let num: u32 = 4;
        let response: (bool, bool, bool) = (true, false, true);
        assert_eq!(response, divisble_by_2_3_4(num));
    }

    #[test]
    fn test_divisible_by_2_3_4_5() {
        let num: u32 = 5;
        let response: (bool, bool, bool) = (false, false, false);
        assert_eq!(response, divisble_by_2_3_4(num));
    }

    #[test]
    fn test_divisible_by_2_3_4_24() {
        let num: u32 = 24;
        let response: (bool, bool, bool) = (true, true, true);
        assert_eq!(response, divisble_by_2_3_4(num));
    }

    #[test]
    fn test_for_array_add_10() {
        let arr: [u32; 5] = [0, 1, 2, 3, 4];
        let response: [u32; 5] = [10, 11, 12, 13, 14];
        assert_eq!(response, for_array_add_10(arr));
    }

    #[test]
    fn test_for_reverse_array() {
        let arr: [u32; 5] = [6, 7, 8, 9, 10];
        let response: [u32; 5] = [10, 9, 8, 7, 6];
        assert_eq!(response, for_reverse_array(arr));
    }

    #[test]
    fn test_increment_by_25() {
        let num: u32 = 50;
        let increment = 25;
        assert_eq!((num + increment, increment), increment_by_25(num));
    }

    #[test]
    fn test_add_elements_together() {
        let arr: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let response: u32 = arr.iter().sum();
        assert_eq!(response, add_elements_together(arr));
    }
}
