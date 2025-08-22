/// A savings account
pub struct SavingsAccount {
    balance: i32,
}

impl SavingsAccount {
    /// Creates a 'SavingsAccount' with a balance of 0
    ///
    /// # Example
    ///
    /// '''
    /// use bank::SavingsAccount;
    /// let account = SavingsAccount::new();
    /// assert_eq!(account.get_balance(),0);
    /// '''
    ///
    pub fn new() -> SavingsAccount {
        SavingsAccount { balance: 0 }
    }

    pub fn get_balance(&self) -> i32 { self.balance }

    pub fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            panic!("Can not deposit a negative amount!") // not good practice; just to should #[should_panic]
        }
        self.balance += amount;
    }

    pub fn transfer(&self, acc_number: u32, amount: i32) -> Result<String, String> {
        Ok(format!("Transferred ${amount} to {acc_number}"))
    }
}

#[cfg(test)] // This module will only run when "cargo test" is entered in the terminal
mod tests {
    use super::*;

    // **NOTE** the name of the test function shows up on the terminal when doing "cargo test"
    // so give them descriptive names to know what the test is.
    //
    // There are several test macros: assert_eq!, assert_ne!, assert!

    #[test]
    fn should_have_a_starting_balance_zero() {
        let account = SavingsAccount::new();
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn should_be_able_to_deposit() {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        assert_eq!(account.get_balance(), 100, "Balance should be 100!"); // sees if the two values are equal
        /*
        assert_ne!(account.get_balance(), 0); // sees if the two values are not equal
        assert!(account.get_balance() == 100); // see if the comparison is true.
         */
    }

    #[test]
    #[should_panic] // used when you know the test will panic
    fn should_panic_if_deposit_is_negative() {
        let mut account = SavingsAccount::new();
        account.deposit(-1);
    }

    #[test]
    fn should_transfer_money() -> Result<(), String> {
        let mut account = SavingsAccount::new();
        account.deposit(200);
        account.transfer(300, 400)?; // The question-mark propagate any errors.
        Ok(())
    }
}