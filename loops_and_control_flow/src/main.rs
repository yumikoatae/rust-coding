//mod age_checker; 
//mod phone_validator;
//mod transaction_processor;
//use transaction_processor::{get_transaction_input, process_transaction};
//mod cart_calculator;
mod order_state;

fn main() {
    //age_checker::ask_for_age();
    //phone_validator::ask_for_phone_number();
    //let transaction = get_transaction_input();
    //process_transaction(transaction);
    //cart_calculator::calculate_cart_total();

    let order = order_state::OrderState::Processing;
    order_state::print_order_status(order);

}