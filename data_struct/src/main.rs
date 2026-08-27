// ==========================================
// 1. TUPLES AND ARRAYS EXAMPLE
// ==========================================

// fn main() {
//     // Tuple: groups different types together — perfect for a fixed pair like (lat, long)
//     let location: (f64, f64) = (19.0760, 72.8777); // Mumbai's coordinates
//     println!("Location -> lat: {}, long: {}", location.0, location.1);

//     // Array: fixed-size, same-type collection — a week always has 7 days
//     let week_temps: [i32; 7] = [30, 32, 31, 29, 33, 34, 28];
//     let mut total = 0;
//     for temp in week_temps {
//         total += temp;
//     }
//     println!(
//         "Average temp this week: {}°C",
//         total / week_temps.len() as i32
//     );
// }

// ==========================================
// 2. VECTORS AND MUTABLE BORROWING EXAMPLE
// ==========================================

fn main() {
    // Vec<T>: growable list — a cart's item count isn't known ahead of time
    let mut cart: Vec<String> = Vec::new();

    add_item(&mut cart, "Milk");
    add_item(&mut cart, "Bread");
    add_item(&mut cart, "Eggs");
    add_item(&mut cart, "Eggs");
    add_item(&mut cart, "Eggs");

    println!("Your cart ({} items):", cart.len());
    for item in &cart {
        println!("- {}", item);
    }
}

// &str: a borrowed, read-only view — we're just reading the item name, not owning it
// fn add_item(cart: &mut Vec<String>, item: &str) {
//     cart.push(item.to_string()); // .to_string() converts &str into an owned String
// }

// ==========================================
// 3. ENUMS, STRUCTS, AND PATTERN MATCHING EXAMPLE
// ==========================================

// enum: a fixed set of possible states — an order can only ever be one of these
// enum OrderStatus {
//     Placed,
//     Shipped,
//     Delivered,
//     Cancelled,
// }

// // struct: groups related data about one order together
// struct Order {
//     id: u32,
//     status: OrderStatus,
// }

// impl Order {
//     // Method attached to Order — like a "switch-case" over the enum
//     fn describe(&self) -> String {
//         match self.status {
//             OrderStatus::Placed => format!("Order #{} has been placed.", self.id),
//             OrderStatus::Shipped => format!("Order #{} is on its way!", self.id),
//             OrderStatus::Delivered => format!("Order #{} was delivered.", self.id),
//             OrderStatus::Cancelled => format!("Order #{} was cancelled.", self.id),
//         }
//     }
// }

// fn main() {
//     let order = Order {
//         id: 1042,
//         status: OrderStatus::Shipped,
//     };
//     println!("{}", order.describe());
// }

use sha2::{Digest, Sha256};

fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn main() {
    let password = "correct horse battery staple";
    let hashed = hash_password(password);
    println!("SHA-256 hash: {}", hashed);

    // Verifying: hash the guess and compare — never store the real password
    let guess = "correct horse battery staple";
    println!("Match: {}", hash_password(guess) == hashed);
}
