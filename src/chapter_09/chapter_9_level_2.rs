/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Exercise n°2                                    //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// complete the below function that converts Result<Result<u32, &'static str>, &'static str> to Result<u32, &'static str>
pub fn result_flatten(res: Result<Result<u32, &'static str>, &'static str>) -> Result<u32, &'static str>{
    todo!()
}

// transpose a result from an option into an option of a result
// e.g. result_transpose(Ok(Some(5))) -> Some(Ok(5)), 
// e.g. result_transpose(Ok(None)) -> None,
// e.g. result_transpose(Err(_)) -> Some(Err(_)),
pub fn result_transpose(res: Result<Option<u32>,&'static str>) ->  Option<Result<u32, &'static str>>{
    todo!()
}


// complete the below function that returns an different error message if:
// - the password is less than 8 characters long
// - the password do not contains any digit
// - the password do not contains any letter
// fn check_password_strength(password: &str) -> Result<(), &'static str> {
//     if password.len() < 8 {
//         return Err("Password must be at least 8 characters long");
//     }
//     if !password.chars().any(char::is_ascii_digit) {
//         return Err("Password must contain at least one digit");
//     }
//     if !password.chars().any(char::is_alphabetic) {
//         return Err("Password must contain at least one letter");
//     }
//     Ok(())
// }

// Show how to use debbuger
use std::fs::File;
use std::io::Write;

// look at the different method before or while during the below exercises: https://doc.rust-lang.org/std/fs/struct.File.html
// complete the below function that create a file called stan.txt using the create function: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
// and write the following to it:  b"Dear Slim, I wrote you but you still ain't calling"
pub fn stan_letter() -> Result<(), std::io::Error> {
    todo!()
}

//create an api fetching the price of bitcoin from coingecko
// https://api.coingecko.com/api/v3/coins/bitcoin
// market_data -> current_price -> usd



// Change main function to produce result as output :)


// use serde_json;

// #[derive(Serialize)]
// struct Person {
//     name: String,
//     age: u32,
// }

// fn serialize_to_json(person: &Person) -> Result<String, serde_json::Error> {
//     serde_json::to_string(person)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_rectangle() {}
}
