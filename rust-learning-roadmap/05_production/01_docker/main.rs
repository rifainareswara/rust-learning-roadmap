// Docker for Rust
// ================

fn main() {
    println!("=== Docker for Rust ===\n");
    
    println!("=== Dockerfile ===\n");
    println!(r#"
# Build stage
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/myapp /usr/local/bin/
EXPOSE 8080
CMD ["myapp"]
"#);

    println!("=== docker-compose.yml ===\n");
    println!(r#"
version: '3.8'
services:
  app:
    build: .
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgres://user:pass@db/mydb
    depends_on:
      - db
  
  db:
    image: postgres:15
    volumes:
      - postgres_data:/var/lib/postgresql/data
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: pass
      POSTGRES_DB: mydb

volumes:
  postgres_data:
"#);

    println!("=== Commands ===\n");
    println!("docker build -t myapp .");
    println!("docker run -p 8080:8080 myapp");
    println!("docker-compose up -d");
    println!("docker-compose logs -f app");
}

// EXERCISES:
// 1. Create multi-stage Dockerfile for your API
// 2. Setup docker-compose with PostgreSQL
// 3. Add health check to container
