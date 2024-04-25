// have a look at the doc of the different methods of vector, before or while solving the below exercises : https://doc.rust-lang.org/std/vec/struct.Vec.html

// complete the below function that should remove the last element from a vector and returns it, or return None if the vector is empty
pub fn vec_pop(vec: &mut Vec<u32>) -> Option<u32> {
    todo!()
}

// complete the below function which should return the length of a vector
pub fn vec_length(vec: &Vec<u32>) -> usize {
    todo!()
}

// complete the below function that should return the sum of the element of a vector
pub fn sum_of_vec(vec: &Vec<u32>) -> u32 {
    todo!()
}

// complete the below function that should return the mean of a vector.
// TIP: you could use the result of vec_sum and divide it by the number of element of a vector
pub fn mean_of_vec(vec: &Vec<u32>) -> u32 {
    todo!()
}

// complete the below function that should append the content of vec2 to vec1 and leaving vec2 empty.
pub fn vec_append(vec1: &mut Vec<u32>, vec2: &mut Vec<u32>) -> Vec<u32> {
    todo!()
}

// complete the below function that should append the content of vec2 to vec1 and not let vec2 empty.
pub fn vec_append_not_empty(vec1: &mut Vec<u32>, vec2: &mut Vec<u32>) -> Vec<u32> {
    todo!()
}

// have a look at the doc of the different methods of String, before or while solving the below exercises: https://doc.rust-lang.org/std/string/struct.String.html

// complete the below function that should convert the input string into lowercase. Assume that the input only contains latin characters
pub fn string_to_lowercase(string: String) -> String {
    todo!()
}

// reversed the string provided as input, e.g. string_reversed("hello") -> "olleh" . Assume that the input only contains latin characters
pub fn string_reversed(string: &str) -> String {
    todo!()
}

// Count the number of vowel (i.e. a,e,i,o,u ) in a string provided as input. Assume that the input only contains latin characters (that can also be uppercase)
pub fn string_count_vowels(input: &String) -> usize {
    todo!()
}

// Have a look at the different methods of hashmap before or while solving the below exercises: https://doc.rust-lang.org/std/collections/struct.HashMap.html

// returns the sum of the values of a hashmap
pub fn hashmap_values_sum(hm: &HashMap<String, u32>) -> u32 {
    todo!()
}

// returns the sum of the keys of a hashmap
pub fn hashmap_keys_sum(hm: &HashMap<u32, u32>) -> u32 {
    todo!()
}

// returns true if the hashmap contains the key
pub fn hashmap_contains_key(hm: &HashMap<u32, u32>, key: u32) -> bool {
    todo!()
}

// remove the key from the hashmap and returns the value of the key if it is present or None
pub fn hashmap_removes_key(hm: &mut HashMap<u32, u32>, key: u32) -> Option<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_pop() {
        let mut vec = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(Some(5), vec_pop(&vec));
    }

    #[test]
    fn test_vec_pop_empty() {
        let mut vec = vec::new();
        assert_eq!(None, vec_pop(&vec));
    }

    #[test]
    fn test_vec_length() {
        let vec = vec![0, 1, 2, 3];
        assert_eq!(4, vec_length(&vec));
    }

    #[test]
    fn test_sum_of_vec() {
        let vec = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(0 + 1 + 2 + 3 + 4 + 5, sum_of_vec(vec));
    }

    #[test]
    fn test_mean_of_vec() {
        let vec = vec![10, 10, 10, 10, 10, 10];
        assert_eq!(10, sum_of_vec(vec));
    }

    #[test]
    fn test_vec_append() {
        let mut vec1 = vec![0, 1, 2, 3, 4, 5];
        let mut vec2 = vec![6, 7, 8, 9];
        assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec_append(vec1, vec2));
        assert_eq!(&[], vec2);
    }
    #[test]
    fn test_vec_append_not_empty() {
        let mut vec1 = vec![0, 1, 2, 3, 4, 5];
        let mut vec2 = vec![6, 7, 8, 9];
        assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec_append(vec1, vec2));
        assert_eq!(&[6, 7, 8, 9], vec2);
    }
}
