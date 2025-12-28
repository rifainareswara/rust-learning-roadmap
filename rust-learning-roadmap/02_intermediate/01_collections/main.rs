// Collections in Rust
// ====================
// Vec, HashMap, HashSet - the most commonly used collections

use std::collections::{HashMap, HashSet};

fn main() {
    // ================================
    // VECTOR (Vec<T>)
    // ================================
    // Dynamic array that can grow or shrink
    
    // Create a new vector
    let mut numbers: Vec<i32> = Vec::new();
    
    // Push elements
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("Vector: {:?}", numbers);
    
    // Create with macro
    let fruits = vec!["apel", "jeruk", "mangga"];
    println!("Fruits: {:?}", fruits);
    
    // Access elements
    let first = &numbers[0];           // Panic if out of bounds!
    let second = numbers.get(1);       // Returns Option<T> - safer!
    println!("First: {}, Second: {:?}", first, second);
    
    // Iterate over vector
    println!("\nIterating:");
    for num in &numbers {
        println!("  - {}", num);
    }
    
    // Iterate with mutable reference
    for num in &mut numbers {
        *num *= 2;
    }
    println!("After doubling: {:?}", numbers);
    
    // Pop element
    let last = numbers.pop();
    println!("Popped: {:?}, Remaining: {:?}", last, numbers);
    
    // ================================
    // HASHMAP
    // ================================
    // Key-value pairs, keys must be unique
    
    println!("\n--- HashMap ---");
    
    let mut scores: HashMap<String, i32> = HashMap::new();
    
    // Insert
    scores.insert(String::from("Budi"), 90);
    scores.insert(String::from("Ani"), 85);
    scores.insert(String::from("Citra"), 92);
    println!("Scores: {:?}", scores);
    
    // Access
    let budi_score = scores.get("Budi");
    println!("Budi's score: {:?}", budi_score);
    
    // Check if key exists
    if scores.contains_key("Ani") {
        println!("Ani ada di daftar!");
    }
    
    // Update
    scores.insert(String::from("Budi"), 95); // Overwrite
    println!("After update: {:?}", scores);
    
    // Insert only if key doesn't exist
    scores.entry(String::from("Dedi")).or_insert(88);
    scores.entry(String::from("Budi")).or_insert(100); // Won't change
    println!("After entry: {:?}", scores);
    
    // Iterate
    println!("\nAll scores:");
    for (name, score) in &scores {
        println!("  {} => {}", name, score);
    }
    
    // ================================
    // HASHSET
    // ================================
    // Unique values only, no duplicates
    
    println!("\n--- HashSet ---");
    
    let mut colors: HashSet<&str> = HashSet::new();
    
    colors.insert("red");
    colors.insert("blue");
    colors.insert("green");
    colors.insert("red"); // Duplicate - ignored!
    
    println!("Colors: {:?}", colors);
    println!("Contains red? {}", colors.contains("red"));
    
    // Set operations
    let primary: HashSet<&str> = ["red", "yellow", "blue"].iter().cloned().collect();
    let secondary: HashSet<&str> = ["orange", "green", "purple"].iter().cloned().collect();
    
    // Union
    let all_colors: HashSet<_> = primary.union(&secondary).collect();
    println!("Union: {:?}", all_colors);
    
    // Intersection
    let common: HashSet<_> = colors.intersection(&primary).collect();
    println!("Intersection with primary: {:?}", common);
}

// EXERCISES:
// 1. Buat Vec<String> berisi 5 nama teman, lalu filter yang huruf pertamanya 'A'
// 2. Buat HashMap untuk menyimpan harga barang (nama -> harga), hitung total harga
// 3. Buat HashSet untuk track visited pages, cek apakah sudah pernah visit
