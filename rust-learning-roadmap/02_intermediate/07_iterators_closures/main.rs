// Iterators & Closures in Rust
// =============================
// Functional programming patterns in Rust

fn main() {
    // ================================
    // CLOSURES
    // ================================
    
    println!("=== Closures ===\n");
    
    // Basic closure
    let add_one = |x: i32| x + 1;
    println!("add_one(5) = {}", add_one(5));
    
    // Closure with type annotations (optional)
    let add_two: fn(i32) -> i32 = |x| x + 2;
    println!("add_two(5) = {}", add_two(5));
    
    // Multi-line closure
    let calculate = |x: i32, y: i32| {
        let sum = x + y;
        let product = x * y;
        (sum, product)
    };
    let (sum, product) = calculate(3, 4);
    println!("calculate(3, 4) = sum: {}, product: {}", sum, product);
    
    // Closures capture environment
    let multiplier = 3;
    let multiply = |x| x * multiplier;  // Captures multiplier
    println!("multiply(5) with multiplier {} = {}", multiplier, multiply(5));
    
    // FnOnce - takes ownership
    let greeting = String::from("Hello");
    let consume_greeting = move || {
        println!("Consuming: {}", greeting);
        // greeting is moved into closure
    };
    consume_greeting();
    // println!("{}", greeting);  // ERROR: greeting was moved
    
    // ================================
    // ITERATORS
    // ================================
    
    println!("\n=== Iterators ===\n");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Basic iteration
    println!("Basic iteration:");
    for num in numbers.iter() {
        print!("{} ", num);
    }
    println!();
    
    // Iterator methods return new iterators
    let doubled: Vec<i32> = numbers.iter()
        .map(|x| x * 2)
        .collect();
    println!("Doubled: {:?}", doubled);
    
    // Filter
    let evens: Vec<&i32> = numbers.iter()
        .filter(|x| *x % 2 == 0)
        .collect();
    println!("Evens: {:?}", evens);
    
    // Chain multiple operations
    let result: Vec<i32> = numbers.iter()
        .filter(|x| *x % 2 == 1)  // Odd numbers
        .map(|x| x * 10)           // Multiply by 10
        .collect();
    println!("Odd * 10: {:?}", result);
    
    // Sum
    let total: i32 = numbers.iter().sum();
    println!("Sum: {}", total);
    
    // Product
    let product: i32 = numbers.iter().product();
    println!("Product: {}", product);
    
    // fold (reduce)
    let sum_squared: i32 = numbers.iter()
        .fold(0, |acc, x| acc + x * x);
    println!("Sum of squares: {}", sum_squared);
    
    // ================================
    // COMMON ITERATOR METHODS
    // ================================
    
    println!("\n=== Common Iterator Methods ===\n");
    
    let fruits = vec!["apple", "banana", "cherry", "date"];
    
    // find
    let found = fruits.iter().find(|x| x.starts_with('c'));
    println!("Find 'c': {:?}", found);
    
    // position
    let pos = fruits.iter().position(|x| *x == "banana");
    println!("Position of 'banana': {:?}", pos);
    
    // any & all
    let has_long = fruits.iter().any(|x| x.len() > 5);
    let all_short = fruits.iter().all(|x| x.len() < 10);
    println!("Has long (>5 chars): {}", has_long);
    println!("All short (<10 chars): {}", all_short);
    
    // count
    let count = fruits.iter().filter(|x| x.len() > 4).count();
    println!("Count of fruits with >4 chars: {}", count);
    
    // take & skip
    let first_two: Vec<_> = fruits.iter().take(2).collect();
    let skip_two: Vec<_> = fruits.iter().skip(2).collect();
    println!("First two: {:?}", first_two);
    println!("Skip two: {:?}", skip_two);
    
    // enumerate
    println!("\nEnumerate:");
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  {} -> {}", index, fruit);
    }
    
    // zip
    let prices = vec![100, 50, 75, 120];
    println!("\nZip (fruit, price):");
    for (fruit, price) in fruits.iter().zip(prices.iter()) {
        println!("  {} = Rp {}", fruit, price);
    }
    
    // ================================
    // CREATING CUSTOM ITERATOR
    // ================================
    
    println!("\n=== Custom Iterator ===\n");
    
    let counter = Counter::new();
    let values: Vec<u32> = counter.collect();
    println!("Counter values: {:?}", values);
    
    // Chain with other iterators
    let result: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("Custom iterator result: {}", result);
}

// ================================
// CUSTOM ITERATOR IMPLEMENTATION
// ================================

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// EXERCISES:
// 1. Buat closure yang menghitung faktorial
// 2. Gunakan iterator untuk mencari kata terpanjang dalam Vec<&str>
// 3. Implement custom iterator yang menghasilkan Fibonacci sequence
