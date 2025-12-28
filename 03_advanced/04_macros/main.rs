// Macros in Rust
// ===============
// Declarative and Procedural macros

fn main() {
    // ================================
    // BUILT-IN MACROS
    // ================================
    
    println!("=== Built-in Macros ===\n");
    
    // println! - format and print
    println!("Hello, {}!", "World");
    
    // vec! - create vector
    let v = vec![1, 2, 3];
    println!("vec! macro: {:?}", v);
    
    // format! - create String
    let s = format!("x = {}, y = {}", 10, 20);
    println!("format! macro: {}", s);
    
    // panic! - crash program
    // panic!("This will crash!");
    
    // assert! - check condition
    assert!(2 + 2 == 4);
    
    // assert_eq! and assert_ne!
    assert_eq!(1 + 1, 2);
    assert_ne!(1 + 1, 3);
    
    // dbg! - debug print with file and line
    let x = dbg!(5 * 2);
    println!("x = {}", x);
    
    // todo! and unimplemented!
    // todo!("Implement this later");
    
    // cfg! - check compile-time configuration
    if cfg!(target_os = "macos") {
        println!("Running on macOS!");
    }
    
    // ================================
    // CUSTOM DECLARATIVE MACRO
    // ================================
    
    println!("\n=== Custom Macros ===\n");
    
    // Simple say_hello macro
    say_hello!();
    
    // vec-like macro
    let v = my_vec![1, 2, 3, 4, 5];
    println!("my_vec! result: {:?}", v);
    
    // HashMap macro
    let scores = hashmap! {
        "Budi" => 90,
        "Ani" => 85,
        "Citra" => 92
    };
    println!("hashmap! result: {:?}", scores);
    
    // Calculate macro
    let result = calculate!(5 + 3);
    println!("calculate!(5 + 3) = {}", result);
    
    let result = calculate!(10 - 4);
    println!("calculate!(10 - 4) = {}", result);
    
    // Multiple patterns
    let result = math!(2 times 3);
    println!("math!(2 times 3) = {}", result);
    
    let result = math!(10 plus 5);
    println!("math!(10 plus 5) = {}", result);
    
    // ================================
    // REPETITION IN MACROS
    // ================================
    
    println!("\n=== Repetition ===\n");
    
    // Print all
    print_all!(1, 2, 3, "hello", true);
    
    // Max of multiple values
    let m = max!(5, 3, 9, 1, 7);
    println!("max!(5, 3, 9, 1, 7) = {}", m);
}

// ================================
// SIMPLE MACRO
// ================================

macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
}

// ================================
// VEC-LIKE MACRO
// ================================

macro_rules! my_vec {
    // Empty
    () => {
        Vec::new()
    };
    // With elements
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// ================================
// HASHMAP MACRO
// ================================

macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

// ================================
// EXPRESSION MACRO
// ================================

macro_rules! calculate {
    ( $a:expr + $b:expr ) => {
        $a + $b
    };
    ( $a:expr - $b:expr ) => {
        $a - $b
    };
}

// ================================
// MULTIPLE PATTERNS
// ================================

macro_rules! math {
    ( $a:expr plus $b:expr ) => {
        $a + $b
    };
    ( $a:expr minus $b:expr ) => {
        $a - $b
    };
    ( $a:expr times $b:expr ) => {
        $a * $b
    };
}

// ================================
// REPETITION
// ================================

macro_rules! print_all {
    ( $( $x:expr ),* ) => {
        $(
            println!("  Value: {:?}", $x);
        )*
    };
}

macro_rules! max {
    ( $x:expr ) => {
        $x
    };
    ( $x:expr, $( $y:expr ),+ ) => {
        {
            let mut max_val = $x;
            $(
                if $y > max_val {
                    max_val = $y;
                }
            )+
            max_val
        }
    };
}

/*
=== MACRO SYNTAX REFERENCE ===

Designators (fragment specifiers):
- item: an item (function, struct, module, etc.)
- block: a block expression { ... }
- stmt: a statement
- pat: a pattern
- expr: an expression
- ty: a type
- ident: an identifier
- path: a path (e.g., ::std::mem::replace)
- meta: a meta item (inside #[...])
- tt: a token tree
- literal: a literal (string, number, char)

Repetition:
- $( ... )* : zero or more times
- $( ... )+ : one or more times
- $( ... )? : zero or one time

Separator:
- $( $x:expr ),* : comma-separated
- $( $x:expr );* : semicolon-separated
*/

// EXERCISES:
// 1. Buat macro `min!` yang mirip dengan `max!`
// 2. Buat macro `debug_vars!` yang print nama variable dan nilainya
// 3. Buat macro `measure_time!` yang mengukur waktu eksekusi expression
