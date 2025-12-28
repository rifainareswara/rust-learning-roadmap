// Control Flow in Rust
// =====================

fn main() {
    // ================================
    // IF / ELSE
    // ================================
    
    let number = 7;

    if number < 5 {
        println!("number kurang dari 5");
    } else if number > 5 {
        println!("number lebih dari 5");
    } else {
        println!("number sama dengan 5");
    }

    // If as expression (ternary-like)
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("result = {}", result);

    // ================================
    // LOOP (infinite loop)
    // ================================
    
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2; // return value from loop
        }
    };
    println!("Loop result: {}", result); // 20

    // Loop labels (for nested loops)
    let mut count = 0;
    'outer: loop {
        let mut remaining = 10;
        
        loop {
            if remaining == 9 {
                break; // break inner loop
            }
            if count == 2 {
                break 'outer; // break outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("Count after nested loops: {}", count);

    // ================================
    // WHILE
    // ================================
    
    let mut n = 3;
    
    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }
    println!("LIFTOFF!");

    // ================================
    // FOR (most common)
    // ================================
    
    // Iterate over array
    let arr = [10, 20, 30, 40, 50];
    
    for element in arr {
        println!("Value: {}", element);
    }

    // Range (exclusive end)
    for i in 1..5 {
        print!("{} ", i); // 1 2 3 4
    }
    println!();

    // Range (inclusive end)
    for i in 1..=5 {
        print!("{} ", i); // 1 2 3 4 5
    }
    println!();

    // Reverse range
    for i in (1..4).rev() {
        print!("{} ", i); // 3 2 1
    }
    println!();

    // With index (enumerate)
    let fruits = ["apple", "banana", "cherry"];
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{}: {}", index, fruit);
    }
}

// EXERCISES:
// 1. Buat program FizzBuzz (1-100)
//    - Print "Fizz" jika habis dibagi 3
//    - Print "Buzz" jika habis dibagi 5
//    - Print "FizzBuzz" jika habis dibagi keduanya
// 2. Hitung factorial menggunakan loop
// 3. Buat program menebak angka sederhana
