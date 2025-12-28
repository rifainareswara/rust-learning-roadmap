// Pattern Matching in Rust
// =========================

fn main() {
    // ================================
    // MATCH EXPRESSION
    // ================================
    
    let number = 3;
    
    match number {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        _ => println!("Something else"), // _ is catch-all
    }

    // Match with return value
    let result = match number {
        1 => "satu",
        2 => "dua",
        3 => "tiga",
        _ => "lainnya",
    };
    println!("Result: {}", result);

    // ================================
    // MATCHING ENUMS
    // ================================
    
    let coin = Coin::Quarter;
    let cents = value_in_cents(&coin);
    println!("Value: {} cents", cents);

    // ================================
    // MATCHING Option<T>
    // ================================
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("five + 1 = {:?}", six);
    println!("none + 1 = {:?}", none);

    // ================================
    // MATCH GUARDS
    // ================================
    
    let num = Some(4);
    
    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("Greater or equal to five: {}", x),
        None => println!("No value"),
    }

    // ================================
    // MULTIPLE PATTERNS
    // ================================
    
    let x = 1;
    
    match x {
        1 | 2 => println!("One or two"),
        3..=5 => println!("Three through five"),
        _ => println!("Something else"),
    }

    // ================================
    // DESTRUCTURING
    // ================================
    
    // Tuple
    let point = (3, 5);
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On x-axis at {}", x),
        (0, y) => println!("On y-axis at {}", y),
        (x, y) => println!("At ({}, {})", x, y),
    }

    // Struct
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On x axis at {}", x),
        Point { x: 0, y } => println!("On y axis at {}", y),
        Point { x, y } => println!("At ({}, {})", x, y),
    }

    // ================================
    // IF LET (simplified pattern matching)
    // ================================
    
    let some_value = Some(3);
    
    // Instead of:
    match some_value {
        Some(3) => println!("three!"),
        _ => (),
    }
    
    // Use if let:
    if let Some(3) = some_value {
        println!("three!");
    }

    // if let with else
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("Max is {}", max);
    } else {
        println!("No max configured");
    }

    // ================================
    // WHILE LET
    // ================================
    
    let mut stack = vec![1, 2, 3];
    
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }

    // ================================
    // LET ELSE (Rust 1.65+)
    // ================================
    
    fn get_count(s: &str) -> Option<usize> {
        // Try to parse, return None if failed
        Some(s.len())
    }
    
    let input = "hello";
    let Some(count) = get_count(input) else {
        println!("Failed to get count");
        return;
    };
    println!("Count: {}", count);

    // ================================
    // @ BINDINGS
    // ================================
    
    let msg = Message::Hello { id: 5 };
    
    match msg {
        Message::Hello { id: id_var @ 3..=7 } => {
            println!("Found id in range: {}", id_var)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Hello { id: i32 },
}

// EXERCISES:
// 1. Buat enum `HttpStatus` dengan variants: Ok(200), NotFound(404), InternalError(500)
//    Buat fungsi yang mengembalikan message berdasarkan status
//
// 2. Parse command line dengan pattern matching:
//    "quit" => exit program
//    "hello [name]" => greet name
//    "add [a] [b]" => print sum
//
// 3. Buat state machine menggunakan enum dan match:
//    Traffic Light: Red -> Green -> Yellow -> Red
