// Security Best Practices
// ========================

fn main() {
    println!("=== Security in Rust Backend ===\n");
    
    println!("=== Input Validation ===");
    println!("- Use validator crate for struct validation");
    println!("- Sanitize all user inputs");
    println!("- Use parameterized queries (SQLx does this)\n");
    
    println!("=== Authentication ===");
    println!("- Use Argon2 for password hashing");
    println!("- JWT with short expiration");
    println!("- Refresh token rotation\n");
    
    println!("=== HTTPS ===");
    println!("- Always use HTTPS in production");
    println!("- Use reverse proxy (nginx) for TLS\n");
    
    println!("=== Headers ===\n");
    println!(r#"
.wrap(DefaultHeaders::new()
    .add(("X-Content-Type-Options", "nosniff"))
    .add(("X-Frame-Options", "DENY"))
    .add(("X-XSS-Protection", "1; mode=block"))
    .add(("Strict-Transport-Security", "max-age=31536000"))
)
"#);

    println!("=== Environment Variables ===");
    println!("- Never commit secrets to git");
    println!("- Use .env for development only");
    println!("- Use secrets manager in production\n");
    
    println!("=== Rate Limiting ===");
    println!("- Limit requests per IP/user");
    println!("- Use actix-governor crate");
}

// EXERCISES:
// 1. Implement input validation with validator crate
// 2. Add security headers middleware
// 3. Setup rate limiting
