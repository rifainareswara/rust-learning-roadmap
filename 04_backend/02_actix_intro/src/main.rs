// Actix-web Introduction
// ======================
// 🚀 First REST API with Rust!

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

// ================================
// BASIC HANDLERS
// ================================

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// ================================
// PATH PARAMETERS
// ================================

#[get("/users/{id}")]
async fn get_user(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    HttpResponse::Ok().json(serde_json::json!({
        "id": user_id,
        "name": format!("User {}", user_id)
    }))
}

#[get("/greet/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

// ================================
// QUERY PARAMETERS
// ================================

#[derive(Deserialize)]
struct PaginationQuery {
    page: Option<u32>,
    limit: Option<u32>,
}

#[get("/items")]
async fn list_items(query: web::Query<PaginationQuery>) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    
    HttpResponse::Ok().json(serde_json::json!({
        "page": page,
        "limit": limit,
        "items": ["item1", "item2", "item3"]
    }))
}

// ================================
// REQUEST BODY (JSON)
// ================================

#[derive(Deserialize, Serialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: u32,
    name: String,
    email: String,
}

#[post("/users")]
async fn create_user(body: web::Json<CreateUser>) -> impl Responder {
    let user = UserResponse {
        id: 1,
        name: body.name.clone(),
        email: body.email.clone(),
    };
    
    HttpResponse::Created().json(user)
}

// ================================
// MANUAL ROUTING (alternative)
// ================================

async fn manual_handler() -> impl Responder {
    HttpResponse::Ok().body("This is a manually routed handler")
}

// ================================
// APPLICATION STATE
// ================================

struct AppState {
    app_name: String,
    visit_count: std::sync::Mutex<u32>,
}

#[get("/app-info")]
async fn app_info(data: web::Data<AppState>) -> impl Responder {
    let mut count = data.visit_count.lock().unwrap();
    *count += 1;
    
    HttpResponse::Ok().json(serde_json::json!({
        "app_name": data.app_name,
        "visit_count": *count
    }))
}

// ================================
// MAIN SERVER
// ================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server starting at http://127.0.0.1:8080");
    println!("📚 Try these endpoints:");
    println!("   GET  /");
    println!("   GET  /health");
    println!("   GET  /greet/{name}");
    println!("   GET  /users/{id}");
    println!("   GET  /items?page=1&limit=10");
    println!("   POST /users (JSON body)");
    println!("   GET  /app-info");
    
    // Shared application state
    let app_state = web::Data::new(AppState {
        app_name: String::from("Rust REST API"),
        visit_count: std::sync::Mutex::new(0),
    });
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            // Attribute macro routes
            .service(hello)
            .service(health_check)
            .service(get_user)
            .service(greet)
            .service(list_items)
            .service(create_user)
            .service(app_info)
            // Manual route
            .route("/manual", web::get().to(manual_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// ================================
// HOW TO RUN
// ================================
/*
1. cd ke folder ini
2. cargo run
3. Test dengan curl atau browser:

   curl http://localhost:8080/
   curl http://localhost:8080/health
   curl http://localhost:8080/greet/Rifai
   curl http://localhost:8080/users/42
   curl "http://localhost:8080/items?page=2&limit=5"
   curl -X POST http://localhost:8080/users \
        -H "Content-Type: application/json" \
        -d '{"name":"John","email":"john@example.com"}'
*/

// EXERCISES:
// 1. Tambahkan endpoint PUT /users/{id} untuk update user
// 2. Tambahkan endpoint DELETE /users/{id}
// 3. Implementasikan in-memory storage menggunakan Vec<User>
//    dengan Mutex untuk thread-safety
// 4. Tambahkan middleware untuk logging request
