// Variables & Data Types in Rust
// =================================

fn main() {
    // 1. IMMUTABILITY (default)
    let x = 5;
    println!("x = {}", x);
    // x = 6; // ERROR! Cannot assign twice to immutable variable

    // 2. MUTABILITY
    let mut y = 5;
    println!("y before = {}", y);
    y = 6;
    println!("y after = {}", y);

    // 3. CONSTANTS
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);

    // 4. SHADOWING
    let z = 5;
    let z = z + 1; // This is allowed!
    let z = z * 2;
    println!("z = {}", z); // 12

    // ================================
    // SCALAR TYPES
    // ================================

    // Integers
    let a: i32 = -42;      // signed 32-bit
    let b: u32 = 42;       // unsigned 32-bit
    let c: i64 = 1_000_000; // underscore for readability
    println!("Integers: a={}, b={}, c={}", a, b, c);

    // Floating-point
    let f1: f64 = 3.14159; // 64-bit (default)
    let f2: f32 = 2.5;     // 32-bit
    println!("Floats: f1={}, f2={}", f1, f2);

    // Boolean
    let is_active: bool = true;
    let is_greater = 10 > 5;
    println!("Booleans: is_active={}, is_greater={}", is_active, is_greater);

    // Character (Unicode)
    let letter: char = 'A';
    let emoji: char = '🦀';
    println!("Characters: letter={}, emoji={}", letter, emoji);

    // ================================
    // COMPOUND TYPES
    // ================================

    // Tuple
    let tup: (i32, f64, char) = (500, 6.4, 'R');
    let (x, y, z) = tup; // destructuring
    println!("Tuple: ({}, {}, {})", x, y, z);
    println!("Access by index: {}", tup.0); // 500

    // Array (fixed size, same type)
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("Array: first={}, second={}", first, second);

    // Create array with same value
    let zeros = [0; 5]; // [0, 0, 0, 0, 0]
    println!("Zeros: {:?}", zeros);
}

// EXERCISES:
// 1. Buat variable untuk menyimpan nama (String) dan umur (u8)
// 2. Buat tuple berisi (nama, umur, is_student)
// 3. Buat array berisi 7 hari dalam seminggu
