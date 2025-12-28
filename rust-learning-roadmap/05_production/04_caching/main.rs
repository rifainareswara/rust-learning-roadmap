// Caching in Rust
// ================

fn main() {
    println!("=== Caching in Rust ===\n");
    
    println!("=== Redis with deadpool-redis ===\n");
    println!(r#"
use deadpool_redis::{Config, Runtime};
use redis::AsyncCommands;

// Setup
let cfg = Config::from_url("redis://localhost");
let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();

// Usage
async fn get_cached(pool: &Pool, key: &str) -> Option<String> {
    let mut conn = pool.get().await.ok()?;
    conn.get(key).await.ok()
}

async fn set_cached(pool: &Pool, key: &str, value: &str, ttl: usize) {
    let mut conn = pool.get().await.unwrap();
    let _: () = conn.set_ex(key, value, ttl).await.unwrap();
}
"#);

    println!("=== In-Memory Cache (moka) ===\n");
    println!(r#"
use moka::sync::Cache;
use std::time::Duration;

let cache: Cache<String, String> = Cache::builder()
    .max_capacity(10_000)
    .time_to_live(Duration::from_secs(300))
    .build();

cache.insert("key".to_string(), "value".to_string());
let value = cache.get(&"key".to_string());
"#);

    println!("=== Caching Strategies ===");
    println!("- Cache-aside: Check cache, if miss query DB");
    println!("- Write-through: Update cache on every write");
    println!("- TTL: Set expiration for cached data");
}

// EXERCISES:
// 1. Implement cache-aside pattern
// 2. Cache API responses with Redis
// 3. Invalidate cache on data changes
