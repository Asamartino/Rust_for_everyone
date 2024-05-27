/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                       Closures                                      //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// rewrite the following closure with as minimum syntax as possible sur
// closure sur plusieurs ligne

// implement an the below function that returns a closure

// recreate the cacher implementation

// iter vs into iter vs itermut
// https://doc.rust-lang.org/std/iter/index.html#implementing-iterator

// exercise with move

// iterator over hashmap

pub fn convert_arr_to_iter(arr: [u32; 5]) -> impl Iterator<Item = u32> {
    arr.into_iter()
}

// use function enumaate

// complete the below function that returns true if max is the maximum value in this vector.
// use the max method of iterator.
fn is_max(vec: Vec<u32>, max: u32) -> bool {
    let max_vec = match vec.iter().max() {
        Some(&x) => x,
        None => return false,
    };
    max_vec == max
}

/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                       Iterators                                     //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// reduce method instead of sum

//exercise dememonstrating that iteroator are lazy

// use of into_iter and iter_mut

//create a struct and create an interator over it

// exercise on filter

// Burger exercise from Wes Bos

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_pop() {}
}
