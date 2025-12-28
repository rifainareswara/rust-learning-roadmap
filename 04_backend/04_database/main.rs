// Database with SQLx (PostgreSQL)
// ================================
// Async database operations with compile-time checked queries

/*
=== SETUP ===

1. Add to Cargo.toml:
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
tokio = { version = "1", features = ["full"] }
dotenvy = "0.15"

2. Create .env file:
DATABASE_URL=postgres://user:password@localhost/mydb

3. Install SQLx CLI:
cargo install sqlx-cli

4. Create database:
sqlx database create

5. Run migrations:
sqlx migrate run
*/

use std::env;

// Simulated types (actual code requires sqlx crate)
fn main() {
    println!("=== Database with SQLx ===\n");
    
    // ================================
    // CONNECTION
    // ================================
    
    println!("=== Connection ===\n");
    println!(r#"
use sqlx::postgres::PgPoolOptions;

// Create connection pool
let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&env::var("DATABASE_URL")?)
    .await?;
"#);
    
    // ================================
    // MIGRATIONS
    // ================================
    
    println!("=== Migrations ===\n");
    println!(r#"
# Create migration
sqlx migrate add create_users_table

# This creates: migrations/YYYYMMDDHHMMSS_create_users_table.sql

-- migrations/20240101000000_create_users_table.sql
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
"#);
    
    // ================================
    // MODELS
    // ================================
    
    println!("=== Models ===\n");
    println!(r#"
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow)]
struct User {{
    id: i32,
    name: String,
    email: String,
    created_at: DateTime<Utc>,
}}

#[derive(Debug)]
struct CreateUser {{
    name: String,
    email: String,
}}
"#);
    
    // ================================
    // BASIC QUERIES
    // ================================
    
    println!("=== Basic Queries ===\n");
    println!(r#"
// SELECT all users
let users = sqlx::query_as::<_, User>(
    "SELECT * FROM users ORDER BY id"
)
.fetch_all(&pool)
.await?;

// SELECT single user
let user = sqlx::query_as::<_, User>(
    "SELECT * FROM users WHERE id = $1"
)
.bind(1)
.fetch_one(&pool)
.await?;

// SELECT optional (might not exist)
let user = sqlx::query_as::<_, User>(
    "SELECT * FROM users WHERE email = $1"
)
.bind("test@example.com")
.fetch_optional(&pool)
.await?;
"#);
    
    // ================================
    // INSERT
    // ================================
    
    println!("=== INSERT ===\n");
    println!(r#"
// Insert and return created row
let user = sqlx::query_as::<_, User>(
    r#"
    INSERT INTO users (name, email) 
    VALUES ($1, $2) 
    RETURNING *
    "#
)
.bind("John Doe")
.bind("john@example.com")
.fetch_one(&pool)
.await?;

println!("Created user: {{:?}}", user);
"#);
    
    // ================================
    // UPDATE
    // ================================
    
    println!("=== UPDATE ===\n");
    println!(r#"
// Update and return updated row
let user = sqlx::query_as::<_, User>(
    r#"
    UPDATE users 
    SET name = $1, email = $2 
    WHERE id = $3 
    RETURNING *
    "#
)
.bind("Jane Doe")
.bind("jane@example.com")
.bind(1)
.fetch_one(&pool)
.await?;

// Update without returning
let result = sqlx::query(
    "UPDATE users SET name = $1 WHERE id = $2"
)
.bind("Updated Name")
.bind(1)
.execute(&pool)
.await?;

println!("Rows affected: {{}}", result.rows_affected());
"#);
    
    // ================================
    // DELETE
    // ================================
    
    println!("=== DELETE ===\n");
    println!(r#"
let result = sqlx::query("DELETE FROM users WHERE id = $1")
    .bind(1)
    .execute(&pool)
    .await?;

if result.rows_affected() > 0 {{
    println!("User deleted!");
}} else {{
    println!("User not found");
}}
"#);
    
    // ================================
    // COMPILE-TIME CHECKED QUERIES
    // ================================
    
    println!("=== Compile-Time Checked Queries ===\n");
    println!(r#"
// query! macro checks SQL at compile time!
let user = sqlx::query!(
    "SELECT id, name, email FROM users WHERE id = $1",
    1i32
)
.fetch_one(&pool)
.await?;

println!("Name: {{}}", user.name);

// query_as! with custom struct
let users = sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE name LIKE $1",
    "%doe%"
)
.fetch_all(&pool)
.await?;
"#);
    
    // ================================
    // TRANSACTIONS
    // ================================
    
    println!("=== Transactions ===\n");
    println!(r#"
// Start transaction
let mut tx = pool.begin().await?;

// Multiple operations
sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
    .bind("User 1")
    .bind("user1@example.com")
    .execute(&mut *tx)
    .await?;

sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
    .bind("User 2")
    .bind("user2@example.com")
    .execute(&mut *tx)
    .await?;

// Commit transaction
tx.commit().await?;

// Or rollback on error (automatic if tx is dropped without commit)
"#);
    
    // ================================
    // REPOSITORY PATTERN
    // ================================
    
    println!("=== Repository Pattern ===\n");
    println!(r#"
pub struct UserRepository {{
    pool: PgPool,
}}

impl UserRepository {{
    pub fn new(pool: PgPool) -> Self {{
        Self {{ pool }}
    }}
    
    pub async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {{
        sqlx::query_as("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
    }}
    
    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {{
        sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }}
    
    pub async fn create(&self, data: CreateUser) -> Result<User, sqlx::Error> {{
        sqlx::query_as(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *"
        )
        .bind(&data.name)
        .bind(&data.email)
        .fetch_one(&self.pool)
        .await
    }}
    
    pub async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {{
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }}
}}
"#);
    
    println!("\n=== Summary ===\n");
    println!("SQLx Features:");
    println!("✅ Compile-time checked SQL queries");
    println!("✅ Async/await support");
    println!("✅ Connection pooling");
    println!("✅ Migrations");
    println!("✅ Multiple database support (PostgreSQL, MySQL, SQLite)");
}

// EXERCISES:
// 1. Setup PostgreSQL dan jalankan migrasi untuk tabel users
// 2. Buat CRUD API endpoints menggunakan repository pattern
// 3. Implement pagination untuk list users (LIMIT/OFFSET)
