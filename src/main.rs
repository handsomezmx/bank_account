use std::io;

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: String, balance: f64) -> BankAccount {
        BankAccount { owner, balance }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance < amount {
            Err(String::from("Insufficient funds."))
        } else {
            self.balance -= amount;
            Ok(())
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account = BankAccount::new(String::from("John"), 100.0);

    loop {
        println!("Welcome to your bank account, {}!", account.owner);
        println!("1. Deposit");
        println!("2. Withdraw");
        println!("3. Check balance");
        println!("4. Exit");
        println!("Enter your choice:");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter the deposit amount:");

                let mut amount = String::new();

                io::stdin()
                    .read_line(&mut amount)
                    .expect("Failed to read line.");

                let amount: f64 = match amount.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                account.deposit(amount);
                println!("Deposit successful.");
            }
            2 => {
                println!("Enter the withdrawal amount:");

                let mut amount = String::new();

                io::stdin()
                    .read_line(&mut amount)
                    .expect("Failed to read line.");

                let amount: f64 = match amount.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                match account.withdraw(amount) {
                    Ok(()) => println!("Withdrawal successful."),
                    Err(err) => println!("{}", err),
                };
            }
            3 => println!("Your current balance is {}.", account.get_balance()),
            4 => {
                println!("Thank you for using our bank account!");
                break;
            }
            _ => continue,
        };
    }
}