// this function will receive a nested tuple and add 7 to each element
pub fn nested_tuple_increment(nested_tup: ((u32,u32,u32),(u32,u32,u32),(u32,u32,u32))) -> ((u32,u32,u32),(u32,u32,u32),(u32,u32,u32)){
    todo!()
}

//this function will take the middle value of an array of number and convert it to a floating number
// e.g. [1,2,3] -> 2.0
// you could use shadowing
pub fn array_middle_value(arr: [u32;3]) -> f64 {
    todo!()
}

// this function will convert the tuple provided as argument into an array
pub fn tuple_to_array(tup: (u32,u32,u32)) -> [u32;3]{
    todo!()
}

// this function will convert the nested tuple provided as argument into a nested array
pub fn nested_tuple_to_nested_array(tup:  ((u32,u32,u32),(u32,u32,u32),(u32,u32,u32))) -> [[u32;3];3] {
    todo!()
}

// this function will create an array of array. The first array will be [1, 2, 3], and each subsequent array will result from multiplying the preceding array by 2, then three, and so forth
// e.g. array_multiples_123() -> [[1,2,3], [2,4,6], [3,6,9], [4,8,12], [5,10,15]]
// tip: use a nested for loop
pub fn array_multiples_123() -> [[u32;3];5]{
    todo!()
}

//this function will sum every element of the array provided together
// e.g. reduce_array([[1,2,3], [4,5,6], [7,8,9], [10,11,12], [13,14,15]]) -> 120
pub fn reduce_array(arr: [[u32;3];5]) -> u32{
    todo!()
}

// this functions check if the number provided is a prime number (i.e. number greater than one and that is not obtained by the multiplaction of smaller numbers)
// tip: you could use a for loop and the modulo operator
pub fn is_prime(number: u32) -> bool {
    todo!()
}


// this function will create an array containing the first 100 prime numbers i.e. [2,3,5,...,541]
// tip: you could reuse the function created before
pub fn array_100_prime_number() -> [u32;100]{
    todo!()
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_tuple_increment() {
        let nested_tup:  ((u32,u32,u32),(u32,u32,u32),(u32,u32,u32))= ((7,7,7),(7,7,7),(7,7,7));
        let response: ((u32,u32,u32),(u32,u32,u32),(u32,u32,u32))= ((14,14,14),(14,14,14),(14,14,14));
        assert_eq!(response, nested_tuple_increment(nested_tup));
    }

    #[test]
    fn test_array_middle_value() {
        let arr: [u32;3]= [4,5,6];
        let response: f64 = 5.0;
        assert_eq!(response, array_middle_value(arr));
    }

    #[test]
    fn test_tuple_to_array() {
        let tup: (u32,u32,u32)= (4,5,6);
        let response: [u32;3]= [4,5,6];
        assert_eq!(response, tuple_to_array(tup));
    }

    #[test]
    fn test_nested_tuple_to_nested_array() {
        let tup: ((u32,u32,u32),(u32,u32,u32),(u32,u32,u32)) = ((1,2,3),(4,5,6),(7,8,9));
        let response: [[u32;3];3]= [[1,2,3], [4,5,6], [7,8,9]];
        assert_eq!(response, nested_tuple_to_nested_array(tup));
    }

    #[test]
    fn test_array_multiples_123() {
        let response: [[u32;3];5]=  [[1,2,3], [2,4,6], [3,6,9], [4,8,12], [5,10,15]];
        assert_eq!(response, array_multiples_123());
    }

    #[test]
    fn test_reduce_array() {
        let arr: [[u32;3];5]=  [[1,2,3], [2,4,6], [3,6,9], [4,8,12], [5,10,15]];
        let response: u32 = 120;
        assert_eq!(response, reduce_array(arr));
    }


    #[test]
    fn test_is_prime_89() {
        let num: u32 = 89;
        assert_eq!(true, is_prime(num));
    }

    #[test]
    fn test_is_prime_42() {
        let num: u32 = 42;
        assert_eq!(false, is_prime(num));
    }


    #[test]
    fn test_array_100_prime_number() {
        let response: [u32;100]= [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 
        163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 
        373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541];
        assert_eq!(response, array_100_prime_number());
    }

    
}