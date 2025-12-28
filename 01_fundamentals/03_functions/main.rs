// Functions in Rust
// ==================

fn main() {
    // Call functions
    greet();
    greet_person("Rifai");
    
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);

    let (area, perimeter) = rectangle_stats(10, 5);
    println!("Rectangle: area={}, perimeter={}", area, perimeter);

    // Expression vs Statement
    let y = {
        let x = 3;
        x + 1  // No semicolon = expression (returns value)
    };
    println!("y = {}", y); // 4

    // Early return
    let result = check_positive(-5);
    println!("Is positive: {}", result);

    // Higher-order functions preview
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
}

// ================================
// BASIC FUNCTION
// ================================

fn greet() {
    println!("Hello, World!");
}

// ================================
// FUNCTION WITH PARAMETERS
// ================================

fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// Multiple parameters
fn print_labeled_measurement(value: i32, unit: &str) {
    println!("The measurement is: {} {}", value, unit);
}

// ================================
// FUNCTION WITH RETURN VALUE
// ================================

fn add(a: i32, b: i32) -> i32 {
    a + b  // implicit return (no semicolon)
}

// Explicit return
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

// Multiple returns using tuple
fn rectangle_stats(width: i32, height: i32) -> (i32, i32) {
    let area = width * height;
    let perimeter = 2 * (width + height);
    (area, perimeter)
}

// ================================
// EARLY RETURN
// ================================

fn check_positive(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    true
}

// ================================
// GENERIC FUNCTIONS (Preview)
// ================================

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// EXERCISES:
// 1. Buat fungsi `is_even(n: i32) -> bool`
// 2. Buat fungsi `factorial(n: u32) -> u32`
// 3. Buat fungsi `fibonacci(n: u32) -> u32`
// 4. Buat fungsi `celsius_to_fahrenheit(c: f64) -> f64`
