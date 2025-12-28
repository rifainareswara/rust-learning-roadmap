// API Testing in Rust
// ====================

fn main() {
    println!("=== API Testing in Rust ===\n");
    
    println!("=== Unit Tests ===\n");
    println!(r#"
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_users() {
        let app = test::init_service(
            App::new().route("/users", web::get().to(get_users))
        ).await;
        
        let req = test::TestRequest::get().uri("/users").to_request();
        let resp = test::call_service(&app, req).await;
        
        assert!(resp.status().is_success());
    }
}
"#);

    println!("=== Integration Tests ===\n");
    println!(r#"
// tests/api_test.rs
use reqwest;

#[tokio::test]
async fn test_api_flow() {
    let client = reqwest::Client::new();
    
    // Create user
    let res = client.post("http://localhost:8080/users")
        .json(&json!({{"name": "Test", "email": "test@test.com"}}))
        .send().await.unwrap();
    assert_eq!(res.status(), 201);
    
    // Get users
    let res = client.get("http://localhost:8080/users")
        .send().await.unwrap();
    assert!(res.status().is_success());
}
"#);

    println!("=== Test Commands ===\n");
    println!("cargo test                    # Run all tests");
    println!("cargo test test_get_users     # Run specific test");
    println!("cargo test -- --nocapture     # Show println output");
    println!("cargo test -- --test-threads=1  # Sequential");
}

// EXERCISES:
// 1. Write tests for CRUD operations
// 2. Test error responses (404, 400, 500)
// 3. Test authentication flow
