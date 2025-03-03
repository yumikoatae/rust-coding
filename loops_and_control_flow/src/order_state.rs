// Define the enum representing the states of an order
pub enum OrderState {
    New,         // New order
    Processing,  // Order is being processed
    Shipped,     // Order has been shipped
    Delivered,   // Order has been delivered
    Canceled,    // Order was canceled
}

pub fn print_order_status(status: OrderState) {
    match status {
        OrderState::New => println!("The order is new."),
        OrderState::Processing => println!("The order is being processed."),
        OrderState::Shipped => println!("The order has been shipped."),
        OrderState::Delivered => println!("The order has been delivered."),
        OrderState::Canceled => println!("The order has been canceled."),
    }
}