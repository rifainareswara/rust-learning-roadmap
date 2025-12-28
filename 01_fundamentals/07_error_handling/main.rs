// Error Handling in Rust
// =======================

use std::fs::File;
use std::io::{self, Read};

fn main() {
    // ================================
    // PANIC! (Unrecoverable errors)
    // ================================
    
    // panic!("crash and burn"); // Uncomment to see panic
    
    // Accessing invalid index causes panic
    // let v = vec![1, 2, 3];
    // v[99]; // panic: index out of bounds

    // ================================
    // RESULT<T, E> (Recoverable errors)
    // ================================
    
    // File::open returns Result<File, Error>
    let file_result = File::open("hello.txt");
    
    let _file = match file_result {
        Ok(file) => {
            println!("File opened successfully");
            file
        }
        Err(error) => {
            println!("Problem opening file: {:?}", error);
            return;
        }
    };

    // ================================
    // MATCHING DIFFERENT ERRORS
    // ================================
    
    use std::io::ErrorKind;
    
    let file_result = File::open("test.txt");
    
    let _file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(fc) => {
                    println!("Created new file");
                    fc
                }
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error);
            }
        },
    };

    // ================================
    // SHORTCUTS: unwrap() and expect()
    // ================================
    
    // unwrap: panic if Err
    // let f = File::open("hello.txt").unwrap();
    
    // expect: panic with custom message
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // ================================
    // THE ? OPERATOR
    // ================================
    
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error: {}", e),
    }

    // Chained ?
    match read_username_short() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error: {}", e),
    }

    // ================================
    // CUSTOM ERROR TYPES
    // ================================
    
    match validate_username("ab") {
        Ok(()) => println!("Username valid"),
        Err(e) => println!("Validation error: {:?}", e),
    }

    match validate_username("valid_user") {
        Ok(()) => println!("Username valid"),
        Err(e) => println!("Validation error: {:?}", e),
    }

    // ================================
    // CONVERTING ERRORS
    // ================================
    
    let result: Result<i32, _> = "42".parse();
    println!("Parsed: {:?}", result);
    
    let result: Result<i32, _> = "not a number".parse();
    println!("Parse failed: {:?}", result);

    // ================================
    // Option vs Result
    // ================================
    
    let numbers = vec![1, 2, 3];
    
    // Option<T>: absence of value
    let first = numbers.first();
    match first {
        Some(n) => println!("First: {}", n),
        None => println!("Empty vector"),
    }
    
    // Convert Option to Result
    let first_result = numbers.first().ok_or("Empty vector");
    println!("As result: {:?}", first_result);
}

// ================================
// PROPAGATING ERRORS WITH ?
// ================================

fn read_username_from_file() -> Result<String, io::Error> {
    let file_result = File::open("username.txt");
    
    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();
    
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Same function using ? operator (much cleaner!)
fn read_username_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Even shorter with fs::read_to_string
fn read_username_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("username.txt")
}

// ================================
// CUSTOM ERROR TYPES
// ================================

#[derive(Debug)]
enum ValidationError {
    TooShort { min: usize, actual: usize },
    TooLong { max: usize, actual: usize },
    InvalidCharacter(char),
}

fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.len() < 3 {
        return Err(ValidationError::TooShort {
            min: 3,
            actual: username.len(),
        });
    }
    
    if username.len() > 20 {
        return Err(ValidationError::TooLong {
            max: 20,
            actual: username.len(),
        });
    }
    
    for c in username.chars() {
        if !c.is_alphanumeric() && c != '_' {
            return Err(ValidationError::InvalidCharacter(c));
        }
    }
    
    Ok(())
}

// ================================
// USING ANYHOW (for applications)
// ================================
// 
// Add to Cargo.toml:
// anyhow = "1.0"
//
// use anyhow::{Context, Result};
//
// fn main() -> Result<()> {
//     let content = std::fs::read_to_string("file.txt")
//         .context("Failed to read file")?;
//     Ok(())
// }

// ================================
// USING THISERROR (for libraries)
// ================================
//
// Add to Cargo.toml:
// thiserror = "1.0"
//
// use thiserror::Error;
//
// #[derive(Error, Debug)]
// pub enum MyError {
//     #[error("Invalid input: {0}")]
//     InvalidInput(String),
//     #[error("IO error")]
//     Io(#[from] std::io::Error),
// }

// EXERCISES:
// 1. Buat fungsi `divide(a: f64, b: f64) -> Result<f64, String>`
//    yang mengembalikan error jika b == 0
//
// 2. Buat custom error enum untuk validasi email:
//    - MissingAtSymbol
//    - MissingDomain
//    - InvalidFormat
//
// 3. Buat program yang membaca file JSON dan handle semua
//    kemungkinan error (file not found, invalid JSON, dll)
