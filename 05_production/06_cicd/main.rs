// CI/CD for Rust
// ===============

fn main() {
    println!("=== CI/CD for Rust ===\n");
    
    println!("=== GitHub Actions ===\n");
    println!(r#"
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test
      - run: cargo clippy -- -D warnings
      - run: cargo fmt --check

  build:
    runs-on: ubuntu-latest
    needs: test
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release
      - uses: actions/upload-artifact@v3
        with:
          name: binary
          path: target/release/myapp
"#);

    println!("=== Docker Build & Push ===\n");
    println!(r#"
  docker:
    runs-on: ubuntu-latest
    needs: test
    steps:
      - uses: docker/login-action@v3
        with:
          username: ${{ secrets.DOCKER_USER }}
          password: ${{ secrets.DOCKER_TOKEN }}
      - uses: docker/build-push-action@v5
        with:
          push: true
          tags: user/app:latest
"#);

    println!("=== Best Practices ===");
    println!("✅ Run tests on every push");
    println!("✅ Use clippy for linting");
    println!("✅ Check formatting with rustfmt");
    println!("✅ Cache dependencies for speed");
    println!("✅ Build release binaries");
}

// EXERCISES:
// 1. Setup GitHub Actions for your project
// 2. Add code coverage reporting
// 3. Setup automatic deployment
