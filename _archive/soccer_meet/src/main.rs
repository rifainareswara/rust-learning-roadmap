mod routes;
mod models;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use routes::{config::config, health_route::health_checker_handler};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load variabel dari file .env
    dotenv().ok();

    // Set default log level ke "actix_web=info"
    // tetapi tetap bisa dioverride dengan RUST_LOG di environment
    env_logger::Builder::from_env(Env::default().filter_or("RUST_LOG", "actix_web=info")).init();

    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the db is successful!");
            pool
        }
        Err(err) => {
            eprintln!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("Server started successfully 🚀!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::CONTENT_ENCODING,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(actix_web::web::Data::new(AppState { db: pool.clone() }))
            .service(health_checker_handler)
            .configure(config)
            .wrap(cors)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}