use std::collections::HashMap;
// use url::Host;

fn main() {
    // Collections
    // std::collections
    // Vec<T>
    // String
    // HasMap<K, V>
    // HashSet<T>

    let mut products: Vec<String> = Vec::new();
    // Add new Product
    products.push(String::from("Latte"));
    products.push(String::from("Americano"));
    products.push(String::from("Black"));
    products.push(String::from("Cappucino"));

    // Delete Product
    let index= 0;
    // Check Products pastikan tidak kosong
    if !products.is_empty() {
        // Check index product itu ada
        if products.get(index).is_some() {
            // Hapus product dengan index 0
            products.remove(index);
            println!("Success to remove product with index: {index}");
        }
    }

    // Update Product
    let index_to_update = 1;
    if !products.is_empty() {
        if let Some(product) = products.get_mut(index_to_update) {
            *product = "Machiato".to_string();
            println!("Success to update product with index: {index_to_update}");
        }
    }

    // List all Product
    println!("List of Products: ");
    for (index, product) in products.iter().enumerate() {
        println!("{}. {}", index + 1, product);
    }
}

// String
#[test]
fn hello() {
    let mut text = String::new();
    text.push('H');
    text.push_str("Hello");
    text.pop();
    text = text.replace("Hehe", "Hehe");
    println!("{}", text);
}

#[test]
fn crud() {
    let mut laptop: HashMap<&str, String> = HashMap::new();
    laptop.insert("1", "Laptop Asus".to_string());
    laptop.insert("2", "Laptop HP".to_string());
    laptop.insert("3", "Laptop MM".to_string());

    laptop.insert("3", "Laptop CP".to_string());

    for (key, value) in laptop {
        println!("{}: {}", key, value);
    }
}
