// Smart Pointers in Rust
// =======================
// Box, Rc, RefCell - Advanced memory management

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // ================================
    // BOX<T> - Heap Allocation
    // ================================
    
    println!("=== Box<T> ===\n");
    
    // Box stores data on the heap
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Useful for recursive types
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("List: {:?}", list);
    
    // ================================
    // DEREF TRAIT
    // ================================
    
    println!("\n=== Deref Trait ===\n");
    
    let x = 5;
    let y = Box::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);  // Dereference Box
    println!("x = {}, *y = {}", x, *y);
    
    // Custom smart pointer
    let my_box = MyBox::new(10);
    println!("MyBox value: {}", *my_box);
    
    // Deref coercion
    let name = MyBox::new(String::from("Rustacean"));
    hello(&name);  // &MyBox<String> -> &String -> &str
    
    // ================================
    // DROP TRAIT
    // ================================
    
    println!("\n=== Drop Trait ===\n");
    
    let c = CustomSmartPointer {
        data: String::from("first"),
    };
    let d = CustomSmartPointer {
        data: String::from("second"),
    };
    println!("CustomSmartPointers created.");
    
    // Early drop
    drop(c);
    println!("CustomSmartPointer c dropped before end of main.");
    // d will be dropped automatically at end of scope
    
    // ================================
    // RC<T> - Reference Counting
    // ================================
    
    println!("\n=== Rc<T> ===\n");
    
    // Multiple ownership with Rc
    let a = Rc::new(RcList::Cons(5, Rc::new(RcList::Cons(10, Rc::new(RcList::Nil)))));
    println!("Count after creating a: {}", Rc::strong_count(&a));
    
    let b = RcList::Cons(3, Rc::clone(&a));
    println!("Count after creating b: {}", Rc::strong_count(&a));
    
    {
        let c = RcList::Cons(4, Rc::clone(&a));
        println!("Count after creating c: {}", Rc::strong_count(&a));
    }
    
    println!("Count after c goes out of scope: {}", Rc::strong_count(&a));
    
    // ================================
    // REFCELL<T> - Interior Mutability
    // ================================
    
    println!("\n=== RefCell<T> ===\n");
    
    // RefCell allows mutation even when RefCell itself is immutable
    let data = RefCell::new(5);
    
    println!("Before: {:?}", data);
    *data.borrow_mut() += 10;
    println!("After: {:?}", data);
    
    // Multiple borrows
    let value = RefCell::new(vec![1, 2, 3]);
    value.borrow_mut().push(4);
    println!("Vector: {:?}", value.borrow());
    
    // ================================
    // RC<REFCELL<T>> - Multiple Owners with Mutation
    // ================================
    
    println!("\n=== Rc<RefCell<T>> ===\n");
    
    let value = Rc::new(RefCell::new(5));
    
    let a = Rc::clone(&value);
    let b = Rc::clone(&value);
    
    *a.borrow_mut() += 10;
    *b.borrow_mut() += 20;
    
    println!("Value: {:?}", value.borrow());  // 35
}

// ================================
// RECURSIVE TYPE WITH BOX
// ================================

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// ================================
// CUSTOM SMART POINTER
// ================================

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implement Deref
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// ================================
// DROP TRAIT
// ================================

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// ================================
// RC LIST
// ================================

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

// EXERCISES:
// 1. Buat linked list menggunakan Box<T>
// 2. Buat shared counter menggunakan Rc<RefCell<i32>>
// 3. Implement tree structure dengan multiple parents menggunakan Rc<T>
