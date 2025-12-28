// Generics in Rust
// =================
// Write code that works with multiple types

fn main() {
    // ================================
    // WHY GENERICS?
    // ================================
    // Without generics, we'd need separate functions for each type:
    // fn largest_i32(list: &[i32]) -> i32 { ... }
    // fn largest_f64(list: &[f64]) -> f64 { ... }
    // fn largest_char(list: &[char]) -> char { ... }
    
    // ================================
    // GENERIC FUNCTIONS
    // ================================
    
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("Largest number: {}", result);
    
    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("Largest char: {}", result);
    
    // ================================
    // GENERIC STRUCTS
    // ================================
    
    // Point that works with any numeric type
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 4.0 };
    
    println!("Int point: ({}, {})", int_point.x, int_point.y);
    println!("Float point: ({}, {})", float_point.x, float_point.y);
    
    // Mixed types
    let mixed = MixedPoint { x: 5, y: 4.0 };
    println!("Mixed: ({}, {})", mixed.x, mixed.y);
    
    // ================================
    // GENERIC METHODS
    // ================================
    
    let p = Point { x: 5, y: 10 };
    println!("p.x() = {}", p.x());
    
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3: ({}, {})", p3.x, p3.y);
    
    // ================================
    // GENERIC ENUMS
    // ================================
    // Option<T> and Result<T, E> are generic enums!
    
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("hello");
    let none: Option<i32> = None;
    
    println!("\nOption examples:");
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("none: {:?}", none);
    
    // Result example
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something went wrong");
    
    println!("\nResult examples:");
    println!("success: {:?}", success);
    println!("failure: {:?}", failure);
    
    // ================================
    // TYPE CONSTRAINTS (Trait Bounds)
    // ================================
    
    let container = Container { value: 42 };
    container.print_value();
}

// Generic function
// T: PartialOrd - type must support comparison
// T: Copy - type must be copyable
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// Generic struct with same type
struct Point<T> {
    x: T,
    y: T,
}

// Generic struct with different types
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// Generic methods
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Method that mixes generics
impl<T, U> Point<T> {
    fn mixup<V, W>(self, other: Point<V>) -> MixedPoint<T, V> {
        MixedPoint {
            x: self.x,
            y: other.x,
        }
    }
}

// Container with trait bound
struct Container<T: std::fmt::Display> {
    value: T,
}

impl<T: std::fmt::Display> Container<T> {
    fn print_value(&self) {
        println!("Value: {}", self.value);
    }
}

// EXERCISES:
// 1. Buat generic function `find_first` yang mencari element pertama yang cocok
// 2. Buat generic struct `Pair<T>` dengan method `swap()` yang menukar nilainya
// 3. Buat generic function yang mengembalikan element terkecil dari slice
