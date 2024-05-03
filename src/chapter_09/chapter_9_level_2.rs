/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Exercise n°2                                    //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// use flatten method
// use transpose method

// Show how to use debbuger
use std::fs::File;
use std::io::Write;

// look at the different method before or while during the below exercises: https://doc.rust-lang.org/std/fs/struct.File.html
// complete the below function that create a file called stan.txt using the create function: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
// and write the following to it:  b"Dear Slim, I wrote you but you still ain't calling"
pub fn stan_letter() -> Result<(), std::io::Error> {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_rectangle() {}
}
