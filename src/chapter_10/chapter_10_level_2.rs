/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Exercise n°2                                    //////
/////////////////////////////////////////////////////////////////////////////////////////////////


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
