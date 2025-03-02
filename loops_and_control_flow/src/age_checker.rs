use std::io;

pub fn ask_for_age() {
    loop {
        println!("Please enter your age:");

        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).expect("Failed to read line");

        // Trim any extra spaces or newline characters
        let age_input = age_input.trim();

        // Try to parse the input to a u32 (unsigned 32-bit integer)
        match age_input.parse::<u32>() {
            Ok(age) if age >= 18 => {
                println!("Age {} accepted. You are allowed to continue!", age);
                break;
            }
            Ok(_) => {
                println!("Age is less than 18. Please try again.");
            }
            Err(_) => {
                println!("Invalid input! Please enter a valid number.");
            }
        }
    }
}