// Unsafe Rust
// ============
// Low-level operations that bypass Rust's safety guarantees

fn main() {
    // ================================
    // WHY UNSAFE?
    // ================================
    
    println!("=== Why Unsafe? ===\n");
    println!("Unsafe allows:");
    println!("1. Dereference raw pointers");
    println!("2. Call unsafe functions");
    println!("3. Access mutable static variables");
    println!("4. Implement unsafe traits");
    println!("5. Access fields of unions");
    
    // ================================
    // RAW POINTERS
    // ================================
    
    println!("\n=== Raw Pointers ===\n");
    
    let mut num = 5;
    
    // Creating raw pointers is safe
    let r1 = &num as *const i32;      // immutable raw pointer
    let r2 = &mut num as *mut i32;    // mutable raw pointer
    
    println!("r1: {:?}", r1);
    println!("r2: {:?}", r2);
    
    // Dereferencing requires unsafe
    unsafe {
        println!("*r1 = {}", *r1);
        println!("*r2 = {}", *r2);
        
        // Modify through raw pointer
        *r2 = 10;
        println!("After modification: *r2 = {}", *r2);
    }
    
    // ================================
    // UNSAFE FUNCTIONS
    // ================================
    
    println!("\n=== Unsafe Functions ===\n");
    
    // Call unsafe function
    unsafe {
        dangerous();
    }
    
    // Safe wrapper around unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);
    println!("Left: {:?}", left);
    println!("Right: {:?}", right);
    
    // ================================
    // EXTERN FUNCTIONS (FFI)
    // ================================
    
    println!("\n=== Extern Functions (FFI) ===\n");
    
    unsafe {
        println!("Absolute value of -3 from C: {}", abs(-3));
    }
    
    // ================================
    // MUTABLE STATIC VARIABLES
    // ================================
    
    println!("\n=== Mutable Static Variables ===\n");
    
    println!("COUNTER before: {}", unsafe { COUNTER });
    
    add_to_counter(5);
    add_to_counter(3);
    
    println!("COUNTER after: {}", unsafe { COUNTER });
    
    // ================================
    // UNSAFE TRAITS
    // ================================
    
    println!("\n=== Unsafe Traits ===\n");
    
    // Send and Sync are unsafe traits
    // Implementing them requires careful consideration
    println!("Send: Type can be transferred across threads");
    println!("Sync: Type can be referenced from multiple threads");
    
    // ================================
    // UNIONS
    // ================================
    
    println!("\n=== Unions ===\n");
    
    let u = MyUnion { i: 42 };
    
    // Reading union fields requires unsafe
    unsafe {
        println!("u.i = {}", u.i);
        println!("u.f = {}", u.f);  // Reinterprets bits as f32!
    }
    
    // ================================
    // BEST PRACTICES
    // ================================
    
    println!("\n=== Best Practices ===\n");
    println!("1. Minimize unsafe blocks");
    println!("2. Encapsulate unsafe in safe abstractions");
    println!("3. Document why unsafe is needed");
    println!("4. Add assertions to validate invariants");
    println!("5. Use miri for additional checking");
}

// ================================
// UNSAFE FUNCTION
// ================================

unsafe fn dangerous() {
    println!("Called an unsafe function!");
}

// ================================
// SAFE WRAPPER FOR UNSAFE CODE
// ================================

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    
    assert!(mid <= len);
    
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// ================================
// EXTERN (FFI)
// ================================

extern "C" {
    fn abs(input: i32) -> i32;
}

// Expose Rust function to C
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Called from C!");
}

// ================================
// MUTABLE STATIC
// ================================

static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// ================================
// UNION
// ================================

#[repr(C)]
union MyUnion {
    i: i32,
    f: f32,
}

/*
=== UNSAFE CHECKLIST ===

Before using unsafe, ask:
□ Is there a safe alternative?
□ Have I minimized the unsafe scope?
□ Can I wrap it in a safe abstraction?
□ Have I documented the safety invariants?
□ Have I tested edge cases?

Common legitimate uses:
- FFI (calling C libraries)
- Performance-critical code
- Implementing data structures (e.g., linked lists)
- Low-level memory manipulation
*/

// EXERCISES:
// 1. Buat safe wrapper untuk mengakses elemen array tanpa bounds check
// 2. Implement simple linked list menggunakan raw pointers
// 3. Buat FFI binding ke fungsi C sederhana
