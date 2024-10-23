/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Data Types                                      //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// this function receive a tuple and return a tuple with the first element being the tuple received as argument
// e.g. tuple_of_tuple(x,y,z) -> ((x,y,z), y, z)
pub fn tuple_of_tuple(tup: (u32, u32, u32)) -> ((u32, u32, u32), u32, u32) {
    let (x, y, z) = tup; // Destructure the input tuple
    ((x, y, z), y, z)
}

// this function receive a tuple with the first argument being a tuple and return the second element of the first tuple
// e.g tuple_1_2((1,2,3),4,5) -> 2
pub fn tuple_1_2(tup: ((u32, u32, u32), u32, u32)) -> u32 {
    let ((_, answer, _), _, _) = tup;
    answer
}

// reverse the tuple provided and return the new tuple, f.i. (1,2,3,4) -> (4,3,2,1)
pub fn reverse_tuple(tup: (u32, u32, u32, u32)) -> (u32, u32, u32, u32) {
    (tup.3, tup.2, tup.1, tup.0)
}

// reverse the tuple provided using destructuring and return the new tuple using shadowing, , f.i. (1,2,3) -> (3,2,1)
pub fn reverse_tuple_destructuring(tup: (u32, u32, u32)) -> (u32, u32, u32) {
    let (x, y, z) = tup;
    (z, y, x)
}

// this function return an array where every element of the array provided as argument is incremented by 10
pub fn array_add_10(arr: [u32; 5]) -> [u32; 5] {
    [
        arr[0] + 10,
        arr[1] + 10,
        arr[2] + 10,
        arr[3] + 10,
        arr[4] + 10,
    ]
}

// this function returns the following array: [[1,2,3], [4,5,6], [7,8,9]]
pub fn array_cube() -> [[u32; 3]; 3] {
    [[1, 2, 3], 
    [4, 5, 6], 
    [7, 8, 9]]
}

// this function returns the second element of the second array
// e.g. array_cube_2_2([[1,2,3], [4,5,6], [7,8,9]]) -> 5
pub fn array_cube_2_2(arr: [[u32; 3]; 3]) -> u32 {
    arr[1][1]
}

// reverse the array provided, f.i. [1,2,3,4,5] -> [5,4,3,2,1]
pub fn reverse_array(arr: [u32; 5]) -> [u32; 5] {
    [
        arr[4],
        arr[3], 
        arr[2], 
        arr[1], 
        arr[0], 
    ]
}

// this function will take two arrays and add the corresponding element together
// e.g. add_array([1,2,3],[10,10,10]) -> [11,12,13]
pub fn add_array(arr1: [u32; 3], arr2: [u32; 3]) -> [u32; 3] {
    [arr1[0] + arr2[0], arr1[1] + arr2[1], arr1[2] + arr2[2]]
}

// this function will take two arrays and substract the corresponding element together
// e.g. subtract_array([1,2,3], [10,10,10]) -> [-9,-8,-7]
pub fn substract_array(arr1: [i32; 3], arr2: [i32; 3]) -> [i32; 3] {
    [
        arr1[0] - arr2[0],
        arr1[1] - arr2[1],
        arr1[2] - arr2[2],
    ]
}

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Else if                                         //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// this function return the maximum number using multiple else if conditions
pub fn return_max(num1: u32, num2: u32, num3: u32) -> u32 {
    if num1 >= num2 && num1 >= num3 {
        num1
    } else if num2 >= num1 && num2 >= num3 {
        num2
    } else {
        num3
    }
}

// this function return a tuple of bools depending if the number is divisible by two, three or four
// divisible_by_2_3_4(4) -> (true, false,true)
// divisible_by_2_3_4(3) -> (false, true, false)
// divisible_by_2_3_4(24) -> (true, true, true)
// divisible_by_2_3_4(7) -> (false, false, false)
pub fn divisble_by_2_3_4(num: u32) -> (bool, bool, bool) {
    (num % 2 == 0, num % 3 == 0, num % 4 == 0)
}

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Loops                                           //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// Complete the code below by using a while loop that will add 1 to number and index, 25 times in total
// note that this function returns a tuple of the number incremented by 25 and the index
pub fn increment_by_25(number: u32) -> (u32, u32) {
    todo!()
    let mut response = number;
    let mut index = 0;
    while index < 25 {
        response += 1; 
        index += 1;
    }
    (response,index)
}

// Complete the below function that will add all numbers together in the array provided using a for loop
pub fn add_elements_together(arr: [u32; 10]) -> u32 {
    let mut response = 0;
    for element in &arr {
        response += element; 
    }
    response
}

// this function return an array where every element of the array provided as argument is incremented by 10 using a for loop
pub fn for_array_add_10(arr: [u32; 5]) -> [u32; 5] {
    let mut response = [0; 5]; 
    for i in arr {
        result[i] = arr[i] + 10; 
    }
    result 
}

// reverse the array provided, f.i. [1,2,3,4,5] -> [5,4,3,2,1] using a for loop.
pub fn for_reverse_array(arr: [u32; 5]) -> [u32; 5] {
    let mut rev = [0; 5]; 
    for i in arr {
        rev[i] = arr[arr.len() - 1 - i]; 
    }
    rev
}
