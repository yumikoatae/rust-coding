use std::io;

// Define an enum to represent the different transaction types
pub enum Transaction {
    Deposit(f64),
    Withdrawal(f64),
    Transfer(f64, String), // Transfer has an amount and a recipient
}

// Function to process a transaction based on the transaction type
pub fn process_transaction(transaction: Transaction) {
    match transaction {
        // Deposit: Just print the deposit amount
        Transaction::Deposit(amount) => {
            println!("Depositing ${:.2}", amount);
        }
        // Withdrawal: Print withdrawal amount and handle withdrawal
        Transaction::Withdrawal(amount) => {
            if amount > 0.0 {
                println!("Withdrawing ${:.2}", amount);
            } else {
                println!("Invalid withdrawal amount.");
            }
        }
        // Transfer: Print the transfer amount and the recipient
        Transaction::Transfer(amount, recipient) => {
            if amount > 0.0 {
                println!("Transferring ${:.2} to {}", amount, recipient);
            } else {
                println!("Invalid transfer amount.");
            }
        }
    }
}

// Function to get the transaction type from the user
pub fn get_transaction_input() -> Transaction {
    println!("Enter the type of transaction (deposit, withdrawal, transfer):");

    let mut transaction_type = String::new();
    io::stdin().read_line(&mut transaction_type).expect("Failed to read line");

    match transaction_type.trim().to_lowercase().as_str() {
        "deposit" => {
            println!("Enter the amount to deposit:");
            let mut amount_input = String::new();
            io::stdin().read_line(&mut amount_input).expect("Failed to read line");
            let amount: f64 = amount_input.trim().parse().unwrap_or(0.0);
            Transaction::Deposit(amount)
        }
        "withdrawal" => {
            println!("Enter the amount to withdraw:");
            let mut amount_input = String::new();
            io::stdin().read_line(&mut amount_input).expect("Failed to read line");
            let amount: f64 = amount_input.trim().parse().unwrap_or(0.0);
            Transaction::Withdrawal(amount)
        }
        "transfer" => {
            println!("Enter the amount to transfer:");
            let mut amount_input = String::new();
            io::stdin().read_line(&mut amount_input).expect("Failed to read line");
            let amount: f64 = amount_input.trim().parse().unwrap_or(0.0);

            println!("Enter the recipient for the transfer:");
            let mut recipient_input = String::new();
            io::stdin().read_line(&mut recipient_input).expect("Failed to read line");
            let recipient = recipient_input.trim().to_string();

            Transaction::Transfer(amount, recipient)
        }
        _ => {
            println!("Invalid transaction type.");
            Transaction::Deposit(0.0) // Default to deposit if invalid
        }
    }
}