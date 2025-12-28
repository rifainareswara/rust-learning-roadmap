// Traits in Rust
// ===============
// Shared behavior across different types (like interfaces)

use std::fmt::Display;

fn main() {
    // ================================
    // BASIC TRAIT USAGE
    // ================================
    
    let article = NewsArticle {
        headline: String::from("Rust 2024 Released!"),
        location: String::from("Jakarta"),
        author: String::from("Rustacean"),
        content: String::from("Rust continues to dominate..."),
    };
    
    let tweet = Tweet {
        username: String::from("@rustlang"),
        content: String::from("Rust is awesome! 🦀"),
        retweets: 1000,
    };
    
    println!("=== Summaries ===");
    println!("Article: {}", article.summarize());
    println!("Tweet: {}", tweet.summarize());
    
    // ================================
    // DEFAULT IMPLEMENTATIONS
    // ================================
    
    println!("\n=== Default vs Custom ===");
    println!("Article author: {}", article.author());
    println!("Tweet author: {}", tweet.author());
    
    // ================================
    // TRAITS AS PARAMETERS
    // ================================
    
    println!("\n=== Trait as Parameter ===");
    notify(&article);
    notify(&tweet);
    
    // ================================
    // MULTIPLE TRAITS (+ syntax)
    // ================================
    
    notify_with_display(&article);
    
    // ================================
    // RETURNING TRAITS
    // ================================
    
    let item = create_summarizable();
    println!("\n=== Returned Trait ===");
    println!("{}", item.summarize());
    
    // ================================
    // DERIVE TRAITS
    // ================================
    
    let point = Point { x: 10, y: 20 };
    println!("\n=== Derived Traits ===");
    println!("Debug: {:?}", point);
    
    let point2 = point.clone();
    println!("Cloned: {:?}", point2);
    println!("Equal? {}", point == point2);
    
    // ================================
    // OPERATOR OVERLOADING
    // ================================
    
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("\n=== Operator Overload ===");
    println!("p1 + p2 = {:?}", p3);
}

// ================================
// DEFINE A TRAIT
// ================================

trait Summary {
    // Required method (no body)
    fn summarize(&self) -> String;
    
    // Default implementation
    fn author(&self) -> String {
        String::from("(Unknown)")
    }
}

// ================================
// IMPLEMENT TRAIT FOR TYPES
// ================================

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    
    fn author(&self) -> String {
        self.author.clone()
    }
}

struct Tweet {
    username: String,
    content: String,
    retweets: u32,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {} (🔁 {})", self.username, self.content, self.retweets)
    }
    // Uses default author() implementation
}

// ================================
// TRAIT AS PARAMETER
// ================================

// Option 1: impl Trait syntax
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Option 2: Trait bound syntax
fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple traits
fn notify_with_display(item: &(impl Summary + Display)) {
    println!("Display: {}", item);
    println!("Summary: {}", item.summarize());
}

// Implement Display for NewsArticle
impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] {}", self.location, self.headline)
    }
}

// ================================
// RETURNING TRAITS
// ================================

fn create_summarizable() -> impl Summary {
    Tweet {
        username: String::from("@automated"),
        content: String::from("This is an auto-generated tweet"),
        retweets: 0,
    }
}

// ================================
// DERIVE TRAITS
// ================================

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Operator overloading with Add trait
use std::ops::Add;

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// EXERCISES:
// 1. Buat trait `Drawable` dengan method `draw()`, implement untuk Circle dan Rectangle
// 2. Buat function yang menerima `impl Drawable` dan panggil draw()
// 3. Implement operator subtraction (-) untuk Point struct
