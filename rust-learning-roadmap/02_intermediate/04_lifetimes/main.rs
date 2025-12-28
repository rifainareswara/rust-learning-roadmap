// Lifetimes in Rust
// ==================
// Ensure references remain valid

fn main() {
    // ================================
    // WHY LIFETIMES?
    // ================================
    
    // This would NOT compile:
    // let r;
    // {
    //     let x = 5;
    //     r = &x;  // ERROR: x doesn't live long enough
    // }
    // println!("{}", r);  // r would be dangling reference
    
    // ================================
    // BASIC LIFETIME ANNOTATIONS
    // ================================
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("Longest: {}", result);
    
    // ================================
    // LIFETIME IN DIFFERENT SCOPES
    // ================================
    
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("Inner longest: {}", result);
    }
    
    // ================================
    // LIFETIME IN STRUCTS
    // ================================
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("\n=== Struct with Lifetime ===");
    println!("Excerpt: {}", excerpt.part);
    println!("Level: {}", excerpt.level());
    println!("Announced: {}", excerpt.announce_and_return("Attention"));
    
    // ================================
    // STATIC LIFETIME
    // ================================
    
    // 'static lifetime = lives for entire program duration
    let s: &'static str = "I have a static lifetime.";
    println!("\n=== Static Lifetime ===");
    println!("{}", s);
    
    // ================================
    // LIFETIME ELISION RULES
    // ================================
    
    // Rust can often infer lifetimes using 3 rules:
    // 1. Each reference parameter gets its own lifetime
    // 2. If exactly one input lifetime, it's assigned to all outputs
    // 3. If &self or &mut self, the self lifetime is assigned to outputs
    
    // These functions don't need explicit lifetime annotations:
    let s = String::from("hello world");
    println!("\n=== Elision Examples ===");
    println!("First word: {}", first_word(&s));
    
    // ================================
    // COMBINING LIFETIMES, GENERICS, TRAITS
    // ================================
    
    let string1 = String::from("hello");
    let string2 = String::from("world");
    
    let result = longest_with_announcement(
        string1.as_str(),
        string2.as_str(),
        "Today's comparison:",
    );
    println!("\n=== Combined Example ===");
    println!("Result: {}", result);
}

// ================================
// LIFETIME ANNOTATION SYNTAX
// ================================

// 'a is the lifetime parameter
// Both x and y must live at least as long as 'a
// The returned reference will also live at least as long as 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ================================
// STRUCT WITH LIFETIME
// ================================

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // Rule 3: &self lifetime assigned to output
    fn level(&self) -> i32 {
        3
    }
    
    // Explicit lifetimes
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// ================================
// LIFETIME ELISION EXAMPLES
// ================================

// Compiler adds lifetimes automatically based on rules
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// ================================
// COMBINING GENERICS, TRAITS, LIFETIMES
// ================================

use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    announcement: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", announcement);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// EXERCISES:
// 1. Buat function yang mengembalikan reference ke string yang lebih pendek
// 2. Buat struct `Sentence` yang menyimpan reference ke subject dan predicate
// 3. Coba buat kode yang menyebabkan dangling reference, lalu fix
