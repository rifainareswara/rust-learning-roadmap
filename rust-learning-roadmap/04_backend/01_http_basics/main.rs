// HTTP Basics in Rust
// ====================
// Understanding HTTP concepts for backend development

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("=== HTTP Basics ===\n");
    
    // ================================
    // HTTP OVERVIEW
    // ================================
    
    println!("HTTP (HyperText Transfer Protocol)");
    println!("- Client-Server model");
    println!("- Request-Response cycle");
    println!("- Stateless protocol");
    println!();
    
    // ================================
    // HTTP METHODS
    // ================================
    
    println!("=== HTTP Methods ===\n");
    
    let methods = vec![
        ("GET", "Retrieve data", "GET /users"),
        ("POST", "Create new resource", "POST /users"),
        ("PUT", "Update entire resource", "PUT /users/1"),
        ("PATCH", "Partial update", "PATCH /users/1"),
        ("DELETE", "Delete resource", "DELETE /users/1"),
        ("HEAD", "GET without body", "HEAD /users"),
        ("OPTIONS", "Get allowed methods", "OPTIONS /users"),
    ];
    
    for (method, description, example) in methods {
        println!("{:8} - {} ({})", method, description, example);
    }
    
    // ================================
    // HTTP STATUS CODES
    // ================================
    
    println!("\n=== HTTP Status Codes ===\n");
    
    let status_codes = vec![
        ("1xx", "Informational", "100 Continue"),
        ("2xx", "Success", "200 OK, 201 Created, 204 No Content"),
        ("3xx", "Redirection", "301 Moved, 302 Found, 304 Not Modified"),
        ("4xx", "Client Error", "400 Bad Request, 401 Unauthorized, 404 Not Found"),
        ("5xx", "Server Error", "500 Internal Error, 502 Bad Gateway, 503 Unavailable"),
    ];
    
    for (range, category, examples) in status_codes {
        println!("{} - {}", range, category);
        println!("    Examples: {}", examples);
    }
    
    // ================================
    // HTTP HEADERS
    // ================================
    
    println!("\n=== Common HTTP Headers ===\n");
    
    let headers: HashMap<&str, &str> = [
        ("Content-Type", "application/json, text/html, etc."),
        ("Authorization", "Bearer token, Basic auth"),
        ("Accept", "Acceptable response formats"),
        ("User-Agent", "Client software info"),
        ("Cache-Control", "Caching directives"),
        ("Content-Length", "Size of request/response body"),
        ("Set-Cookie", "Set cookies in response"),
        ("Cookie", "Send cookies in request"),
    ].into_iter().collect();
    
    for (header, description) in &headers {
        println!("{:16} - {}", header, description);
    }
    
    // ================================
    // REQUEST STRUCTURE
    // ================================
    
    println!("\n=== HTTP Request Structure ===\n");
    println!(r#"
GET /api/users HTTP/1.1
Host: example.com
Accept: application/json
Authorization: Bearer xyz123

(empty line = end of headers)
(optional body for POST/PUT/PATCH)
"#);
    
    // ================================
    // RESPONSE STRUCTURE
    // ================================
    
    println!("=== HTTP Response Structure ===\n");
    println!(r#"
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 45

{{"id": 1, "name": "Rustacean", "active": true}}
"#);
    
    // ================================
    // SIMPLE TCP SERVER
    // ================================
    
    println!("=== Simple TCP Server Demo ===\n");
    println!("Starting server on 127.0.0.1:7878...");
    println!("Press Ctrl+C to stop\n");
    println!("Try: curl http://127.0.0.1:7878/");
    println!("Or open in browser: http://127.0.0.1:7878/\n");
    
    // Start server (comment out to just see theory)
    start_server();
}

fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let request = String::from_utf8_lossy(&buffer);
    println!("Request received:\n{}", request.lines().next().unwrap_or(""));
    
    // Parse request line
    let request_line = request.lines().next().unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    
    let (status, content) = if parts.len() >= 2 {
        let method = parts[0];
        let path = parts[1];
        
        match (method, path) {
            ("GET", "/") => {
                ("200 OK", r#"<!DOCTYPE html>
<html>
<head><title>Rust HTTP Server</title></head>
<body>
    <h1>🦀 Welcome to Rust HTTP Server!</h1>
    <p>This is a simple HTTP server built with Rust.</p>
    <ul>
        <li><a href="/">Home</a></li>
        <li><a href="/api/hello">API: Hello</a></li>
        <li><a href="/api/users">API: Users</a></li>
    </ul>
</body>
</html>"#)
            }
            ("GET", "/api/hello") => {
                ("200 OK", r#"{"message": "Hello from Rust API! 🦀"}"#)
            }
            ("GET", "/api/users") => {
                ("200 OK", r#"[
    {"id": 1, "name": "Budi", "email": "budi@example.com"},
    {"id": 2, "name": "Ani", "email": "ani@example.com"},
    {"id": 3, "name": "Citra", "email": "citra@example.com"}
]"#)
            }
            _ => {
                ("404 NOT FOUND", r#"{"error": "Not Found"}"#)
            }
        }
    } else {
        ("400 BAD REQUEST", r#"{"error": "Bad Request"}"#)
    };
    
    let content_type = if content.starts_with("<!DOCTYPE") || content.starts_with("<html") {
        "text/html"
    } else {
        "application/json"
    };
    
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        content_type,
        content.len(),
        content
    );
    
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// EXERCISES:
// 1. Tambahkan endpoint POST /api/users yang menerima JSON
// 2. Tambahkan logging untuk setiap request (method, path, timestamp)
// 3. Implement simple routing menggunakan HashMap
