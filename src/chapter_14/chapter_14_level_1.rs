// exercise with move

// iterator over hashmap

// pub fn convert_arr_to_iter(arr: [u32; 5]) -> impl Iterator<Item = u32> {
//     arr.into_iter()
// }

// use function enumarate


// complete the below function that sum up every element togethers using iterators
// hint: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
pub fn total(arr: [u32; 10]) -> u32 {
    todo!()
}

// complete the below function that will return true if any element of this array is > 0
// hint: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
pub fn any_positive_number(arr: [i32; 10]) -> bool {
    todo!()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_positive_number_false() {
        let input = [0, -2, -999, -43, -3, -832, -456, -134, -549, -450];
        assert_eq!(any_positive_number(input), false);
    }

}
