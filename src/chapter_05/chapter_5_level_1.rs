/////////////////////////////////////////////////////////////////////////////////////////////////
//////                                     Exercise n°1                                    //////
/////////////////////////////////////////////////////////////////////////////////////////////////

// The below struct represent a simple bank account. Complete the methods of the BankAccount structure
// note that some method require the instance to be mutable while some other do not.
// once you complete this exercise try starting it from a blank file to get use of writing the functions,etc.

struct BankAccount {
    account_name: String,
    balance: f64,
}

// As we are working with float we might run in weird behavior (see more here: https://floating-point-gui.de/basic/)
// Before updating the final balance value we would:
// multiply the final result by PRECISION apply the method .round() on the result and then divid iy by PRECISION
// e.g. let final_answer = (input * PRECISION).round() / PRECISION
const PRECISION: f64 = 100.0;

impl BankAccount {
    // create a new BankAccount using name with a balance of 0.0
    // notice that in this case you can't use the field init shorthand as the parameter has a different name thant the struct field
    fn new_bank_account(name: String) -> BankAccount {
        // todo!()
        BankAccount {
            account_name: name,
            balance: 0.0,
        }
    }

    // deposit a given amount into the BankAccount
    fn deposit(&mut self, amount: f64) {
        // todo!()
        self.balance = (self.balance + amount * PRECISION).round() / PRECISION
    }

    // return the balance of the account
    fn check_balance(&self) -> f64 {
        // todo!()
        self.balance
    }

    // check if the instance can withdraw a given amount
    // return true if the amount can be withdrawn without balance getting lower than 0.0
    fn can_withdraw(&self, amount: f64) -> bool {
        amount <= self.balance
    }

    // withdraw a certain amount
    // this bank do not allow the client to have a balance lower than 0
    // tip: try using can_withdraw to help you out
    fn withdraw(&mut self, amount: f64) {
        // todo!()
        if self.can_withdraw(amount) {
            self.balance -= amount;
        }
    }

    // transfer money from one account to another. As before an account can not go below 0.0
    // you could use the can_withdraw method to help you
    fn transfer_from(&mut self, to: &mut BankAccount, amount: f64) {
        // todo!()
        if self.can_withdraw(amount) {
            self.withdraw(amount);
            to.deposit(amount);
        }
    }

    // increase the balance by a given interest due per period
    fn apply_interest(&mut self, interest_rate: f64) {
        // todo!()
        self.balance = (self.balance * (1.0 + interest_rate) * PRECISION).round() / PRECISION
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bank_account() {
        let bank_account_john_doe = BankAccount::new_bank_account(String::from("John Doe"));

        assert_eq!("John Doe", bank_account_john_doe.account_name);
        assert_eq!(0.0, bank_account_john_doe.balance);
    }

    #[test]
    fn test_check_balance() {
        let bank_account_john_doe = BankAccount::new_bank_account(String::from("John Doe"));

        assert_eq!(0.0, bank_account_john_doe.balance);
    }

    #[test]
    fn test_deposit() {
        let mut bank_account_john_doe = BankAccount::new_bank_account(String::from("John Doe"));
        let value_deposited: f64 = 100.0;
        bank_account_john_doe.deposit(value_deposited);

        assert_eq!(value_deposited, bank_account_john_doe.balance);
    }

    #[test]
    fn test_can_withdraw_successful() {
        let mut bank_account_john_doe = BankAccount::new_bank_account(String::from("John Doe"));
        let value_deposited: f64 = 100.0;
        let value_withdrawn = 50.0;
        bank_account_john_doe.deposit(value_deposited);

        assert_eq!(true, bank_account_john_doe.can_withdraw(value_withdrawn));
    }

    fn test_can_withdraw_fail() {
        let mut bank_account_john_doe = BankAccount::new_bank_account(String::from("John Doe"));
        let value_deposited: f64 = 100.0;
        let value_withdrawn = 1000.0;
        bank_account_john_doe.deposit(value_deposited);

        assert_eq!(false, bank_account_john_doe.can_withdraw(value_withdrawn));
    }

    #[test]
    fn test_withdraw_successful() {
        let mut bank_account_john_doe = BankAccount::new_bank_account(String::from("John Doe"));
        let value_deposited = 100.0;
        let value_withdrawn = 50.0;
        bank_account_john_doe.deposit(value_deposited);
        bank_account_john_doe.withdraw(value_withdrawn);

        assert_eq!(
            value_deposited - value_withdrawn,
            bank_account_john_doe.balance
        );
    }

    #[test]
    fn test_withdraw_failed() {
        let mut bank_account_john_doe = BankAccount::new_bank_account(String::from("John Doe"));
        let value_deposited = 100.0;
        let value_withdrawn = 102.0;
        bank_account_john_doe.deposit(value_deposited);
        bank_account_john_doe.withdraw(value_withdrawn);

        assert_eq!(value_deposited, bank_account_john_doe.balance);
    }

    #[test]
    fn test_transfer_successful() {
        let mut bank_account_john_doe = BankAccount::new_bank_account(String::from("John Doe"));
        let mut bank_account_pepe_roni = BankAccount::new_bank_account(String::from("Pepe Roni"));
        let value_deposited = 100.0;
        let value_transferred = 50.0;
        bank_account_john_doe.deposit(value_deposited);
        bank_account_john_doe.transfer_from(&mut bank_account_pepe_roni, value_transferred);

        assert_eq!(
            value_deposited - value_transferred,
            bank_account_john_doe.balance
        );
        assert_eq!(value_transferred, bank_account_pepe_roni.balance)
    }

    #[test]
    fn test_transfer_failed() {
        let mut bank_account_john_doe = BankAccount::new_bank_account(String::from("John Doe"));
        let mut bank_account_pepe_roni = BankAccount::new_bank_account(String::from("Pepe Roni"));
        let value_deposited = 100.0;
        let value_transferred = 1000.0;
        bank_account_john_doe.deposit(value_deposited);
        bank_account_john_doe.transfer_from(&mut bank_account_pepe_roni, value_transferred);

        assert_eq!(value_deposited, bank_account_john_doe.balance);
        assert_eq!(0.0, bank_account_pepe_roni.balance)
    }

    #[test]
    fn test_apply_interest() {
        let mut bank_account_john_doe = BankAccount::new_bank_account(String::from("John Doe"));
        let value_deposited = 100.0;
        let interest_rate = 0.1;
        bank_account_john_doe.deposit(value_deposited);
        bank_account_john_doe.apply_interest(interest_rate);

        assert_eq!(110.0, bank_account_john_doe.balance)
    }
}
