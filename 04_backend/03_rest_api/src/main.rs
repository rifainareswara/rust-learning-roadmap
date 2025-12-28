// REST API CRUD Example
// ======================
// Complete CRUD API with in-memory storage

use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ================================
// DATA MODELS
// ================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: Uuid,
    title: String,
    description: Option<String>,
    completed: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct CreateTodoRequest {
    title: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateTodoRequest {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

// ================================
// APPLICATION STATE
// ================================

struct AppState {
    todos: Mutex<Vec<Todo>>,
}

// ================================
// API RESPONSES
// ================================

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    fn error(message: &str) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            message: Some(message.to_string()),
        }
    }
}

// ================================
// HANDLERS
// ================================

// GET /api/todos - List all todos
#[get("/api/todos")]
async fn list_todos(data: web::Data<AppState>) -> impl Responder {
    let todos = data.todos.lock().unwrap();
    HttpResponse::Ok().json(ApiResponse::success(todos.clone()))
}

// GET /api/todos/{id} - Get single todo
#[get("/api/todos/{id}")]
async fn get_todo(
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let todo_id = path.into_inner();
    let todos = data.todos.lock().unwrap();

    match todos.iter().find(|t| t.id == todo_id) {
        Some(todo) => HttpResponse::Ok().json(ApiResponse::success(todo.clone())),
        None => HttpResponse::NotFound().json(ApiResponse::<()>::error("Todo not found")),
    }
}

// POST /api/todos - Create new todo
#[post("/api/todos")]
async fn create_todo(
    body: web::Json<CreateTodoRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let now = Utc::now();
    let todo = Todo {
        id: Uuid::new_v4(),
        title: body.title.clone(),
        description: body.description.clone(),
        completed: false,
        created_at: now,
        updated_at: now,
    };

    let mut todos = data.todos.lock().unwrap();
    todos.push(todo.clone());

    HttpResponse::Created().json(ApiResponse::success(todo))
}

// PUT /api/todos/{id} - Update todo
#[put("/api/todos/{id}")]
async fn update_todo(
    path: web::Path<Uuid>,
    body: web::Json<UpdateTodoRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let todo_id = path.into_inner();
    let mut todos = data.todos.lock().unwrap();

    if let Some(todo) = todos.iter_mut().find(|t| t.id == todo_id) {
        if let Some(title) = &body.title {
            todo.title = title.clone();
        }
        if let Some(description) = &body.description {
            todo.description = Some(description.clone());
        }
        if let Some(completed) = body.completed {
            todo.completed = completed;
        }
        todo.updated_at = Utc::now();

        HttpResponse::Ok().json(ApiResponse::success(todo.clone()))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("Todo not found"))
    }
}

// DELETE /api/todos/{id} - Delete todo
#[delete("/api/todos/{id}")]
async fn delete_todo(
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let todo_id = path.into_inner();
    let mut todos = data.todos.lock().unwrap();

    let initial_len = todos.len();
    todos.retain(|t| t.id != todo_id);

    if todos.len() < initial_len {
        HttpResponse::Ok().json(ApiResponse::success("Todo deleted"))
    } else {
        HttpResponse::NotFound().json(ApiResponse::<()>::error("Todo not found"))
    }
}

// Health check
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy"
    }))
}

// ================================
// MAIN
// ================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 REST API Server starting at http://127.0.0.1:8080");
    println!("");
    println!("📚 Endpoints:");
    println!("   GET    /health          - Health check");
    println!("   GET    /api/todos       - List all todos");
    println!("   GET    /api/todos/{id}  - Get single todo");
    println!("   POST   /api/todos       - Create todo");
    println!("   PUT    /api/todos/{id}  - Update todo");
    println!("   DELETE /api/todos/{id}  - Delete todo");
    println!("");
    println!("💡 Example requests:");
    println!(r#"   curl -X POST http://localhost:8080/api/todos \"#);
    println!(r#"        -H "Content-Type: application/json" \"#);
    println!(r#"        -d '{{"title":"Learn Rust","description":"Complete the roadmap"}}'"#);

    let app_state = web::Data::new(AppState {
        todos: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(health)
            .service(list_todos)
            .service(get_todo)
            .service(create_todo)
            .service(update_todo)
            .service(delete_todo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// ================================
// API TESTING COMMANDS
// ================================
/*
# Health check
curl http://localhost:8080/health

# Create todos
curl -X POST http://localhost:8080/api/todos \
     -H "Content-Type: application/json" \
     -d '{"title":"Learn Rust","description":"Complete the roadmap"}'

curl -X POST http://localhost:8080/api/todos \
     -H "Content-Type: application/json" \
     -d '{"title":"Build REST API"}'

# List all todos
curl http://localhost:8080/api/todos

# Get single todo (replace {id} with actual UUID)
curl http://localhost:8080/api/todos/{id}

# Update todo
curl -X PUT http://localhost:8080/api/todos/{id} \
     -H "Content-Type: application/json" \
     -d '{"completed":true}'

# Delete todo
curl -X DELETE http://localhost:8080/api/todos/{id}
*/

// NEXT STEPS:
// 1. Tambahkan validasi input (title tidak boleh kosong)
// 2. Implementasikan pagination untuk list todos
// 3. Tambahkan filtering (completed=true/false)
// 4. Ganti in-memory storage dengan database (SQLx + PostgreSQL)
