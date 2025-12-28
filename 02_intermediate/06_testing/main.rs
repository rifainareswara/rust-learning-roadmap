// Testing in Rust
// ================
// Unit tests, integration tests, and test organization

fn main() {
    println!("=== Testing Demo ===");
    println!("Run tests with: cargo test");
    println!();
    
    // Demo the functions
    let result = add(2, 3);
    println!("add(2, 3) = {}", result);
    
    let rect = Rectangle::new(30, 50);
    println!("Rectangle: {}x{}, area = {}", rect.width, rect.height, rect.area());
    println!("Can hold 10x40? {}", rect.can_hold(&Rectangle::new(10, 40)));
    
    let greeting = greeting("Rustacean");
    println!("Greeting: {}", greeting);
}

// ================================
// FUNCTIONS TO TEST
// ================================

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be >= 1, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be <= 100, got {}", value);
        }
        
        Guess { value }
    }
}

// ================================
// UNIT TESTS
// ================================

#[cfg(test)]
mod tests {
    // Import everything from parent module
    use super::*;
    
    // Basic test
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    // Test another function
    #[test]
    fn test_add_two() {
        assert_eq!(add_two(3), 5);
    }
    
    // Test with assert! macro
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);
        
        assert!(larger.can_hold(&smaller));
    }
    
    // Negative test
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);
        
        assert!(!smaller.can_hold(&larger));
    }
    
    // Test equality
    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(10, 5);
        assert_eq!(rect.area(), 50);
    }
    
    // Test inequality
    #[test]
    fn test_not_equal() {
        let result = add(2, 2);
        assert_ne!(result, 5);
    }
    
    // Custom failure message
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was: {}",
            result
        );
    }
    
    // Test that should panic
    #[test]
    #[should_panic(expected = "Guess value must be <= 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
    
    // Test using Result
    #[test]
    fn it_works_with_result() -> Result<(), String> {
        if add(2, 2) == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    
    // Ignored test (run with cargo test -- --ignored)
    #[test]
    #[ignore]
    fn expensive_test() {
        // This test takes a long time...
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

/*
=== TEST COMMANDS ===

# Run all tests
cargo test

# Run specific test by name
cargo test it_works

# Run tests matching pattern
cargo test greeting

# Run ignored tests
cargo test -- --ignored

# Run all tests including ignored
cargo test -- --include-ignored

# Show output from passing tests
cargo test -- --show-output

# Run tests in one thread (for tests that share state)
cargo test -- --test-threads=1

=== TEST ORGANIZATION ===

Unit tests:
- In the same file as the code
- In #[cfg(test)] module
- Can test private functions

Integration tests:
- In tests/ directory
- Each file is separate crate
- Only tests public API

tests/
├── integration_test.rs
└── common/
    └── mod.rs    # Shared setup code

*/

// EXERCISES:
// 1. Tambahkan test untuk subtract, multiply, divide functions
// 2. Buat test yang check panic dengan #[should_panic]
// 3. Buat integration test di folder tests/
