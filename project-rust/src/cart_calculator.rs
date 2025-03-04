struct Product {
    name: String,
    price: f64,
}

fn calculate_total(cart: &Vec<Product>) -> f64 {
    cart.iter().map(|p| p.price).sum()
}

pub fn calculate_cart_total() {
    let cart = vec![
        Product { name: "Laptop".to_string(), price: 3500.00 },
        Product { name: "Mouse".to_string(), price: 150.00 },
        Product { name: "Keyboard".to_string(), price: 200.00 },
    ];
    
    let total = calculate_total(&cart);
    println!("The total purchase amount is: ${:.2}", total);
    
    // The cart can still be accessed
    println!("First item in the cart: {}", cart[0].name);
}