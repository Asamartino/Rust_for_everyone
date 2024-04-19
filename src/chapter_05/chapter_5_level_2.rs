/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Exercise n°2                                    //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// The below struct represent a more complex bank account. Complete the methods of the BankAccount structure
// The field frozen represent if the account is frozen or not:
//      - if frozen the account can not transfer money but can receive money
//      - if frozen is false the account can transfer and receive money
// once you complete this exercise try doing from a blank file to get use of writing the struct functions, etc.

struct Balance {
    usd: f64,
    eur: f64,
}
struct BankAccount {
    account_name: String,
    balance: Balance,
    frozen: bool
}

const USD_EUR_RATE: f64 = 0.9;
const BANK_FLAT_FEE: f64 = 2.5;

// complete the below function that should return the first word find in a sentence.
// Assume the sentence is only constituted of the 26 letters of the modern English alphabet and the words are separated by whitespace
// if the sentence has no whitespace this function should return the entire word
pub fn return_first_word(s: &str) -> &str {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Variables, mutability and shadowing
    #[test]
    fn test_return_first_word() {
        let s = String::from("the quick brown fox jumps over the lazy dog");
        assert_eq!("the", return_first_word(&s));
    }
    #[test]
    fn test_return_first_word1() {
        let s = String::from("IAmAVeryVeryVeryReallyVeryVeryLongWord");
        assert_eq!(&s, return_first_word(&s));
    }
}
