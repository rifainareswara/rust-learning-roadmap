// Ownership & Borrowing in Rust
// ==============================
// 🔥 INI ADALAH KONSEP PALING PENTING DI RUST!

fn main() {
    // ================================
    // OWNERSHIP RULES:
    // 1. Each value has exactly one owner
    // 2. When owner goes out of scope, value is dropped
    // 3. Value can be moved or copied
    // ================================

    // String lives on HEAP (owned type)
    let s1 = String::from("hello");
    let s2 = s1; // MOVE! s1 is no longer valid
    // println!("{}", s1); // ERROR: value borrowed after move
    println!("s2 = {}", s2);

    // Clone untuk deep copy
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4); // Both valid!

    // Integers are COPIED (implement Copy trait)
    let x = 5;
    let y = x; // COPY, not move
    println!("x = {}, y = {}", x, y); // Both valid!

    // ================================
    // OWNERSHIP WITH FUNCTIONS
    // ================================

    let s = String::from("ownership");
    takes_ownership(s);
    // println!("{}", s); // ERROR: s was moved

    let x = 5;
    makes_copy(x);
    println!("x still valid: {}", x); // OK, was copied

    // Return ownership
    let s1 = gives_ownership();
    println!("Got ownership: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // s2 invalid, s3 is valid
    println!("s3 = {}", s3);

    // ================================
    // REFERENCES & BORROWING
    // ================================
    
    let s1 = String::from("hello");
    
    // & creates a reference (borrow)
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}", s1, len); // s1 still valid!

    // ================================
    // MUTABLE REFERENCES
    // ================================
    
    let mut s = String::from("hello");
    change(&mut s);
    println!("After change: {}", s);

    // RULE: Only ONE mutable reference at a time!
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // ERROR: cannot borrow as mutable more than once
    println!("{}", r1);

    // Can't mix mutable and immutable references
    let mut s = String::from("hello");
    let r1 = &s; // OK
    let r2 = &s; // OK
    println!("{} and {}", r1, r2);
    // References r1 and r2 are no longer used after this point
    
    let r3 = &mut s; // OK now, r1/r2 out of scope
    println!("{}", r3);

    // ================================
    // SLICES
    // ================================
    
    let s = String::from("hello world");
    
    let hello = &s[0..5];  // or &s[..5]
    let world = &s[6..11]; // or &s[6..]
    let whole = &s[..];    // whole string
    
    println!("Slices: '{}' '{}'", hello, world);

    // String slice type: &str
    let word = first_word(&s);
    println!("First word: {}", word);

    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("Array slice: {:?}", slice); // [2, 3]
}

fn takes_ownership(s: String) {
    println!("Took ownership of: {}", s);
} // s is dropped here

fn makes_copy(x: i32) {
    println!("Copied value: {}", x);
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope but doesn't drop (it's a reference)

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    
    &s[..]
}

// ================================
// SUMMARY - Ownership Rules
// ================================
// 
// 1. Each value has ONE owner
// 2. When owner goes out of scope, value is dropped
// 3. To avoid ownership transfer:
//    - Use &T for immutable borrow
//    - Use &mut T for mutable borrow
// 4. References must always be valid (no dangling)
// 5. At any time, you can have:
//    - ONE mutable reference, OR
//    - ANY number of immutable references
//
// EXERCISES:
// 1. Perbaiki code ini:
//    let s = String::from("test");
//    let s2 = s;
//    println!("{}", s);
//
// 2. Buat fungsi yang menghitung jumlah kata dalam string
//    tanpa mengambil ownership
//
// 3. Buat fungsi yang membalik string secara in-place
//    menggunakan mutable reference
