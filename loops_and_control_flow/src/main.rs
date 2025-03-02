//mod age_checker; 
//mod phone_validator;
mod transaction_processor;
use transaction_processor::{get_transaction_input, process_transaction};

fn main() {
    //age_checker::ask_for_age();
    //phone_validator::ask_for_phone_number();
    let transaction = get_transaction_input();
    process_transaction(transaction);
}