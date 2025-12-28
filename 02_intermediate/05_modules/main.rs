// Modules & Crates in Rust
// =========================
// Organizing code into modules and packages

// ================================
// MODULE BASICS
// ================================

// Define a module with `mod`
mod restaurant {
    // Public function
    pub fn greet() {
        println!("Welcome to our restaurant!");
    }
    
    // Nested module
    pub mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("Adding to waitlist...");
            }
            
            pub fn seat_at_table() {
                println!("Seating at table...");
            }
        }
        
        pub mod serving {
            pub fn take_order() {
                println!("Taking order...");
            }
            
            pub fn serve_order() {
                println!("Serving order...");
            }
        }
    }
    
    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,  // Private field
        }
        
        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
        
        pub enum Appetizer {
            Soup,
            Salad,
        }
        
        fn fix_incorrect_order() {
            cook_order();
            // Access parent module with super
            super::front_of_house::serving::serve_order();
        }
        
        fn cook_order() {
            println!("Cooking...");
        }
    }
    
    pub fn eat_at_restaurant() {
        // Absolute path
        crate::restaurant::front_of_house::hosting::add_to_waitlist();
        
        // Relative path
        front_of_house::serving::take_order();
        
        // Order breakfast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");  // Public field
        println!("I'd like {} toast please", meal.toast);
        
        // Enum variants are public if enum is public
        let _order1 = back_of_house::Appetizer::Soup;
        let _order2 = back_of_house::Appetizer::Salad;
    }
}

// ================================
// USE KEYWORD
// ================================

// Bring items into scope with `use`
use restaurant::front_of_house::hosting;
use restaurant::front_of_house::serving::take_order;

// Re-exporting with `pub use`
pub use restaurant::front_of_house::serving;

// Nested paths
// use std::{cmp::Ordering, io};
// use std::io::{self, Write};

// Glob operator (use sparingly)
// use std::collections::*;

// ================================
// EXTERNAL CRATES
// ================================

// In Cargo.toml:
// [dependencies]
// rand = "0.8"
//
// Then: use rand::Rng;

fn main() {
    println!("=== Module Examples ===\n");
    
    // Call function from restaurant module
    restaurant::greet();
    
    // Using the `use` statement
    hosting::add_to_waitlist();
    take_order();
    
    // Through re-export
    serving::serve_order();
    
    // Full restaurant flow
    println!("\n=== Full Restaurant Flow ===");
    restaurant::eat_at_restaurant();
    
    // ================================
    // MODULE ORGANIZATION EXAMPLE
    // ================================
    
    println!("\n=== Module Organization ===");
    println!("
Project structure for larger projects:

my_project/
├── Cargo.toml
├── src/
│   ├── main.rs          # Binary entry point
│   ├── lib.rs           # Library root
│   ├── garden/
│   │   ├── mod.rs       # garden module
│   │   └── vegetables.rs
│   └── house/
│       ├── mod.rs       # house module
│       ├── kitchen.rs
│       └── bedroom.rs
    ");
    
    // ================================
    // VISIBILITY RULES
    // ================================
    
    println!("\n=== Visibility Rules ===");
    println!("1. Everything is private by default");
    println!("2. Use `pub` to make items public");
    println!("3. Child modules can access parent's private items");
    println!("4. Parent modules cannot access child's private items");
    println!("5. pub(crate) - visible within the crate only");
    println!("6. pub(super) - visible to parent module only");
}

// Alternative: Separate files
// src/restaurant.rs or src/restaurant/mod.rs
// would contain the restaurant module code

// EXERCISES:
// 1. Buat module `math` dengan sub-module `basic` (add, subtract) dan `advanced` (power, sqrt)
// 2. Gunakan `pub use` untuk re-export agar user tidak perlu tahu struktur internal
// 3. Buat module hierarchy untuk e-commerce: Product, Cart, Order
