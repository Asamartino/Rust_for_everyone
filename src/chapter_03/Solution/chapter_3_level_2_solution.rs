// this function will receive a nested tuple and add 7 to each element
pub fn nested_tuple_increment(nested_tup: ((u32,u32,u32),(u32,u32,u32),(u32,u32,u32))) -> ((u32,u32,u32),(u32,u32,u32),(u32,u32,u32)){
    let ((a1, a2, a3), (b1, b2, b3), (c1, c2, c3)) = nested_tup;
    
    (
        (a1 + 7, a2 + 7, a3 + 7),
        (b1 + 7, b2 + 7, b3 + 7),
        (c1 + 7, c2 + 7, c3 + 7),
    )
}

//this function will take the middle value of an array of u32 and convert it to a floating number
// e.g. [1,2,3] -> 2.0
// you could use shadowing
pub fn array_middle_value(arr: [u32;3]) -> f64 {
    arr[1] as f64
}

// this function will convert the tuple provided as argument into an array
pub fn tuple_to_array(tup: (u32,u32,u32)) -> [u32;3]{
    let (a, b, c) = tup;
    [a, b, c]
}

// this function will convert the nested tuple provided as argument into a nested array
pub fn nested_tuple_to_nested_array(tup:  ((u32,u32,u32),(u32,u32,u32),(u32,u32,u32))) -> [[u32;3];3] {
    let ((a1, a2, a3), (b1, b2, b3), (c1, c2, c3)) = tup;
    [[a1, a2, a3], [b1, b2, b3], [c1, c2, c3]]
}

// this function will create an array of array. The first array will be [1, 2, 3], and each subsequent array will result from multiplying the preceding array by 2, then three, and so forth
// e.g. array_multiples_123() -> [[1,2,3], [2,4,6], [3,6,9], [4,8,12], [5,10,15]]
// tip: use a nested for loop
pub fn array_multiples_123() -> [[u32;3];5]{
    let mut result = [[0; 3]; 5]; 
    for i in 0..5 { 
        for j in 0..3 { 
            result[i][j] = (i + 1) * (j + 1);
        }
    }
    result
}

//this function will sum every element of the array provided together
// e.g. reduce_array([[1,2,3], [4,5,6], [7,8,9], [10,11,12], [13,14,15]]) -> 120
pub fn reduce_array(arr: [[u32;3];5]) -> u32{
    let mut sum = 0; 
    for i in 0..5 { 
        for j in 0..3 {
            sum += arr[i][j]; 
        }
    }
    sum
}

// this functions check if the number provided is a prime number (i.e. number greater than one and that is not obtained by the multiplication of smaller numbers)
// tip: you could use a for loop and the modulo operator
pub fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..(number-1) {
        if number % i == 0 {
            return false; 
        }
    }
    true
}


// this function will create an array containing the first 100 prime numbers i.e. [2,3,5,...,541]
// tip: you could reuse the function created before
pub fn array_100_prime_number() -> [u32;100]{
    let mut prime_array: [u32; 100] = [0; 100]; 
    let mut i = 0; 
    let mut prime_number = 2;

    while i < 100 {
        if is_prime(prime_number) {
            prime_array[i] = prime_number; 
            i += 1; 
        }
        prime_number += 1;
    }

    prime_array
}