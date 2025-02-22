fn main() {
    let mut account = BankAccount {
        owner: String::from("Kevin Grimaldi"),
        balance: 10000000.00,
    };

    account.check_balance();
    account.withdraw(1000000.67);
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    /**
    Function to withdraw money from the account
     */
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account owned by {}",
            amount, self.owner
        );

        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "The balance of account owned by {} is ${}",
            self.owner, self.balance
        );
    }
}
