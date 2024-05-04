// before completing the below exercises have a look at the documentation: https://doc.rust-lang.org/std/result/enum.Result.html

// complete the below function that returns true if the result is Ok.
// you could use the is_ok method: https://doc.rust-lang.org/std/result/enum.Result.html#method.is_ok
pub fn result_is_ok(res: Result<u32, &str>) -> bool {
    todo!();
}

// complete the below function that returns true if the result is false.
// you could use the is_err method: https://doc.rust-lang.org/std/result/enum.Result.html#method.is_err
pub fn result_is_false(res: Result<u32, &str>) -> bool {
    todo!();
}

// complete the below function that returns true if the ok value is bigger than 100 and false otherwise
// you could use the is_ok_and method: https://doc.rust-lang.org/std/result/enum.Result.html#method.is_ok_and
pub fn result_is_ok_and_bigger_than_10(res: Result<u32, &str>) -> bool {
    todo!();
}

// converts the input to an option
// you could use the ok method: https://doc.rust-lang.org/std/result/enum.Result.html#method.ok
pub fn result_to_option(res: Result<u32, &str>) -> Option<u32> {
    todo!();
}

// complete the below function that converts and Option to a Result
// if the option contains None it should return Err(0)
// you could use the ok_or method: https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or
pub fn option_u32_to_result_u32(option: Option<u32>) -> Result<u32, u32> {
    todo!();
}

// returns the contained Ok value or 0 if res contains and error
// you could use the unwrapped_or method: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or
pub fn result_to_u32(res: Result<u32, &str>) -> u32 {
    todo!();
}

// complete the below function that should return the element at the specified index
// if index is out of bound it return Err("Index out of bounds")
pub fn safe_get_element_by_index(a: &[u32], index: usize) -> Result<&u32, &'static str> {
    todo!();
}

// complete the below function that should return the square root of a number
// it returns Err("can't compute sqrt of negative number") if the number is negative
// use the sqrt method: https://doc.rust-lang.org/std/primitive.f64.html#method.sqrt
pub fn safe_sqrt(n: f64) -> Result<f64, &'static str> {
    todo!();
}

// complete the below function that parse an str to a u32 by using the method parse: https://doc.rust-lang.org/std/primitive.str.html#method.parse
pub fn parse_to_u32(s: &str) -> Result<u32, std::num::ParseIntError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_pop() {}
}
