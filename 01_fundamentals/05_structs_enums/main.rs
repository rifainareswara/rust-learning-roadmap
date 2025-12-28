// Structs & Enums in Rust
// ========================

fn main() {
    // ================================
    // STRUCTS
    // ================================
    
    // Create instance
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("user123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("User: {} ({})", user1.username, user1.email);

    // Mutable struct
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("another"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("new@example.com");

    // Struct update syntax
    let user3 = User {
        email: String::from("third@example.com"),
        ..user1 // take rest from user1
    };
    println!("User3: {}", user3.username);

    // ================================
    // TUPLE STRUCTS
    // ================================
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("Black RGB: ({}, {}, {})", black.0, black.1, black.2);

    // ================================
    // UNIT STRUCT
    // ================================
    
    let _unit = AlwaysEqual;

    // ================================
    // METHODS
    // ================================
    
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("Area: {} px²", rect.area());
    println!("Can hold 10x40? {}", rect.can_hold(&Rectangle { width: 10, height: 40 }));
    
    // Associated function (like static method)
    let square = Rectangle::square(25);
    println!("Square area: {}", square.area());

    // ================================
    // ENUMS
    // ================================
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    route(&home);
    route(&loopback);

    // Message enum with different data types
    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 128, 0),
    ];
    
    for msg in &messages {
        msg.call();
    }

    // ================================
    // OPTION<T> - Rust's null alternative
    // ================================
    
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;
    
    // Must handle both cases!
    match some_number {
        Some(n) => println!("Got number: {}", n),
        None => println!("No number"),
    }
    
    // Useful methods
    let x: Option<i32> = Some(5);
    println!("Is some? {}", x.is_some());
    println!("Unwrap or default: {}", x.unwrap_or(0));
    println!("Map: {:?}", x.map(|n| n * 2));

    // ================================
    // RESULT<T, E> - Error handling
    // ================================
    
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

// ================================
// STRUCT DEFINITIONS
// ================================

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit struct
struct AlwaysEqual;

// Struct dengan methods
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Associated function (no self)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// ================================
// ENUM DEFINITIONS
// ================================

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip: &IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6: {}", addr),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: rgb({}, {}, {})", r, g, b),
        }
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// EXERCISES:
// 1. Buat struct `Book` dengan fields: title, author, pages, is_read
//    Tambahkan method `read()` yang mengubah is_read menjadi true
//
// 2. Buat enum `Shape` dengan variants: Circle(radius), Rectangle(width, height)
//    Implementasikan method `area()` untuk keduanya
//
// 3. Buat struct `BankAccount` dengan balance
//    Implementasikan methods: deposit, withdraw (return Result)
