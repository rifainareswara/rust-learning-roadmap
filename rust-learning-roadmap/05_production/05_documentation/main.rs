// API Documentation
// ==================

fn main() {
    println!("=== API Documentation in Rust ===\n");
    
    println!("=== utoipa (OpenAPI/Swagger) ===\n");
    println!(r#"
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(ToSchema)]
struct User {
    id: i32,
    name: String,
}

#[utoipa::path(
    get,
    path = "/users",
    responses((status = 200, body = Vec<User>))
)]
async fn get_users() -> impl Responder {...}

#[derive(OpenApi)]
#[openapi(paths(get_users), components(schemas(User)))]
struct ApiDoc;

// Mount Swagger UI
App::new()
    .service(SwaggerUi::new("/swagger-ui/{_:.*}")
        .url("/api-docs/openapi.json", ApiDoc::openapi()))
"#);

    println!("=== Rust Doc Comments ===\n");
    println!(r#"
/// Creates a new user in the database.
/// 
/// # Arguments
/// * `user` - The user data to create
/// 
/// # Returns
/// The created user with assigned ID
/// 
/// # Errors
/// Returns error if email already exists
pub async fn create_user(user: CreateUser) -> Result<User, Error> {...}
"#);

    println!("Generate docs: cargo doc --open");
}

// EXERCISES:
// 1. Add OpenAPI annotations to all endpoints
// 2. Generate API documentation
// 3. Add example requests/responses
