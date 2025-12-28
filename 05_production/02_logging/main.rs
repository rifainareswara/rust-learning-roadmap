// Logging in Rust
// ================

fn main() {
    println!("=== Logging in Rust ===\n");
    
    println!("Dependencies:");
    println!("log = \"0.4\"");
    println!("env_logger = \"0.10\"");
    println!("tracing = \"0.1\"  # For async\n");
    
    println!("=== Basic Logging ===\n");
    println!(r#"
use log::{debug, info, warn, error};

fn main() {
    env_logger::init();  // RUST_LOG=info cargo run
    
    debug!("Debug message");
    info!("Server starting on port 8080");
    warn!("Low memory warning");
    error!("Failed to connect: {}", err);
}
"#);

    println!("=== Tracing (Async) ===\n");
    println!(r#"
use tracing::{info, instrument};
use tracing_subscriber;

#[instrument]
async fn process_request(id: u32) {
    info!(request_id = id, "Processing request");
}

fn main() {
    tracing_subscriber::fmt::init();
}
"#);

    println!("=== Log Levels ===");
    println!("ERROR > WARN > INFO > DEBUG > TRACE\n");
    
    println!("=== Environment ===");
    println!("RUST_LOG=debug cargo run");
    println!("RUST_LOG=myapp=debug,actix=info cargo run");
}

// EXERCISES:
// 1. Setup structured logging with JSON output
// 2. Add request ID to all log entries
// 3. Configure log rotation
