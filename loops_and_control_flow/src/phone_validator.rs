use std::io;
use regex::Regex;

pub fn ask_for_phone_number() {
    let re = Regex::new(r"^(\(\d{2}\)\s?)?\d{4,5}-\d{4}$").unwrap();
    let mut phone_input = String::new();

    // Use while and Option to validate the input
    while let None = get_valid_phone_number(&re, &mut phone_input) {
        println!("Invalid phone number! Please try again.");
    }

    println!("Phone number accepted!");
}

// Function that returns Option<String> (valid or invalid phone number)
fn get_valid_phone_number(re: &Regex, phone_input: &mut String) -> Option<String> {
    println!("Please enter a phone number (format: (XX) XXXX-XXXX or XXXX-XXXX):");

    phone_input.clear(); 
    io::stdin().read_line(phone_input).expect("Failed to read line");

    let phone_input = phone_input.trim();


    if re.is_match(phone_input) {
        Some(phone_input.to_string()) 
    } else {
        None 
    }
}