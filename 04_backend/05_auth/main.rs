// Authentication in Rust
// =======================
// JWT, Password Hashing, and Auth Middleware

fn main() {
    println!("=== Authentication in Rust ===\n");
    
    println!("Key Dependencies:");
    println!("- jsonwebtoken = \"9\"");
    println!("- argon2 = \"0.5\"");
    println!("- chrono = {{ features = [\"serde\"] }}");
    
    println!("\n=== Password Hashing (Argon2) ===\n");
    println!(r#"
use argon2::{{password_hash::*, Argon2}};

pub fn hash_password(password: &str) -> Result<String, Error> {{
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}}

pub fn verify_password(password: &str, hash: &str) -> bool {{
    let parsed = PasswordHash::new(hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}}
"#);
    
    println!("=== JWT Tokens ===\n");
    println!(r#"
use jsonwebtoken::{{encode, decode, Header, EncodingKey, DecodingKey, Validation}};

#[derive(Serialize, Deserialize)]
pub struct Claims {{
    pub sub: String,   // user id
    pub exp: usize,    // expiration
    pub role: String,
}}

pub fn create_token(user_id: &str, role: &str) -> Result<String, Error> {{
    let claims = Claims {{
        sub: user_id.to_string(),
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
        role: role.to_string(),
    }};
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
}}

pub fn validate_token(token: &str) -> Result<Claims, Error> {{
    Ok(decode::<Claims>(token, &DecodingKey::from_secret(SECRET), 
        &Validation::default())?.claims)
}}
"#);

    println!("=== Login Flow ===");
    println!("1. Find user by email");
    println!("2. Verify password with Argon2");
    println!("3. Generate JWT token");
    println!("4. Return token + user info");
}

// EXERCISES:
// 1. Implement refresh token mechanism
// 2. Add password reset functionality
// 3. Implement role-based middleware
