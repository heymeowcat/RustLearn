enum Transaction {
    Deposit(f64),
    Withdraw(f64),
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: String, initial_balance: f64) -> Self {
        BankAccount {
            owner,
            balance: initial_balance,
        }
    }

    fn apply_transaction(&mut self, transaction: Transaction) {
        match transaction {
            Transaction::Deposit(amount) => {
                self.balance += amount;
                println!("Deposited Rs.{:.2}", amount);
            }
            Transaction::Withdraw(amount) => {
                if amount > self.balance {
                    println!("Insufficient balance for withdrawal of Rs.{:.2}", amount);
                } else {
                    self.balance -= amount;
                    println!("Withdrew Rs.{:.2}", amount);
                }
            }
        }
    }

    fn display_balance(&self) {
        println!("{}'s account balance: Rs.{:.2}", self.owner, self.balance);
    }
}

fn main() {
    let mut account = BankAccount::new("Vidura".to_string(), 1000.0);
    account.display_balance();

    account.apply_transaction(Transaction::Deposit(200.0));
    account.apply_transaction(Transaction::Withdraw(500.0));
    account.apply_transaction(Transaction::Withdraw(800.0));

    account.display_balance();
}
