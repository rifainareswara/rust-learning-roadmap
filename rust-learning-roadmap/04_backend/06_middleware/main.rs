// Middleware in Actix-web
// ========================

fn main() {
    println!("=== Middleware in Actix-web ===\n");
    
    println!("Middleware = Code yang dieksekusi sebelum/sesudah handler\n");
    
    println!("=== Logger Middleware ===\n");
    println!(r#"
use actix_web::middleware::Logger;

App::new()
    .wrap(Logger::default())
    .wrap(Logger::new("%a %{User-Agent}i"))
"#);

    println!("=== CORS Middleware ===\n");
    println!(r#"
use actix_cors::Cors;

App::new()
    .wrap(
        Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
            .max_age(3600)
    )
"#);

    println!("=== Custom Middleware ===\n");
    println!(r#"
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};

pub struct Timer;

impl<S, B> Transform<S, ServiceRequest> for Timer {...}

// Usage:
App::new()
    .wrap(Timer)
    .route("/", web::get().to(index))
"#);

    println!("=== Rate Limiting ===\n");
    println!(r#"
use actix_governor::{Governor, GovernorConfigBuilder};

let config = GovernorConfigBuilder::default()
    .per_second(2)
    .burst_size(5)
    .finish()
    .unwrap();

App::new()
    .wrap(Governor::new(&config))
"#);

    println!("Common Middleware:");
    println!("✅ Logger - Request logging");
    println!("✅ CORS - Cross-Origin Resource Sharing");
    println!("✅ Compress - Response compression");
    println!("✅ DefaultHeaders - Add default headers");
    println!("✅ Auth - Authentication check");
}

// EXERCISES:
// 1. Buat timing middleware yang log execution time
// 2. Implement request ID middleware
// 3. Buat rate limiting per user
