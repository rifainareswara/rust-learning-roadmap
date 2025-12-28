# 🦀 Rust Learning Path: Backend Developer Journey

> Roadmap pembelajaran Rust dari dasar hingga menjadi Backend Developer profesional.

## 🎮 Quick Start

```bash
# RECOMMENDED: Jalankan Interactive Quest System
cd rustquest
cargo run --release

# Atau jalankan file pembelajaran manual:
cd 01_fundamentals/01_variables
rustc main.rs -o main && ./main
```

## 📊 Learning Phases

| Phase | Folder             | Topik                                      | Durasi     |
| ----- | ------------------ | ------------------------------------------ | ---------- |
| 1     | `01_fundamentals/` | Variables, Ownership, Error Handling       | 2-4 minggu |
| 2     | `02_intermediate/` | Collections, Generics, Traits, Lifetimes   | 2-4 minggu |
| 3     | `03_advanced/`     | Async, Concurrency, Smart Pointers, Macros | 3-4 minggu |
| 4     | `04_backend/`      | Actix-web, REST API, PostgreSQL, JWT       | 4-6 minggu |
| 5     | `05_production/`   | Docker, Logging, Caching, CI/CD            | 4-6 minggu |

## � Project Structure

```
rust-learning-roadmap/
├── rustquest/                 # 🎮 Interactive Quest CLI
│   └── cargo run --release    # Start learning here!
│
├── 01_fundamentals/           # ⭐ Phase 1
│   ├── 01_variables/          # Variables & Data Types
│   ├── 02_control_flow/       # If, Loop, Match
│   ├── 03_functions/          # Functions & Closures
│   ├── 04_ownership/          # 🔥 CRITICAL - don't skip!
│   ├── 05_structs_enums/      # Structs & Enums
│   ├── 06_pattern_matching/   # Pattern Matching
│   └── 07_error_handling/     # Result & Option
│
├── 02_intermediate/           # 📚 Phase 2
│   ├── 01_collections/        # Vec, HashMap, HashSet
│   ├── 02_generics/           # Generic Types & Functions
│   ├── 03_traits/             # Traits & Implementations
│   ├── 04_lifetimes/          # Lifetime Annotations
│   ├── 05_modules/            # Modules & Crates
│   ├── 06_testing/            # Unit & Integration Tests
│   └── 07_iterators_closures/ # Functional Programming
│
├── 03_advanced/               # � Phase 3
│   ├── 01_smart_pointers/     # Box, Rc, RefCell
│   ├── 02_concurrency/        # Threads, Channels, Mutex
│   ├── 03_async/              # Async/Await with Tokio
│   ├── 04_macros/             # Declarative Macros
│   └── 05_unsafe/             # Unsafe Rust
│
├── 04_backend/                # 🌐 Phase 4
│   ├── 01_http_basics/        # HTTP Fundamentals
│   ├── 02_actix_intro/        # Actix-web Hello World
│   ├── 03_rest_api/           # Full CRUD REST API
│   ├── 04_database/           # PostgreSQL with SQLx
│   ├── 05_auth/               # JWT Authentication
│   ├── 06_middleware/         # Custom Middleware
│   └── 07_api_testing/        # API Testing
│
└── 05_production/             # 🏭 Phase 5
    ├── 01_docker/             # Containerization
    ├── 02_logging/            # Structured Logging
    ├── 03_security/           # Security Best Practices
    ├── 04_caching/            # Redis Caching
    ├── 05_documentation/      # API Docs with utoipa
    └── 06_cicd/               # GitHub Actions CI/CD
```

## 🛠️ Tech Stack

| Category         | Technology        |
| ---------------- | ----------------- |
| Framework        | Actix-web         |
| Database         | PostgreSQL + SQLx |
| Auth             | JWT + Argon2      |
| Async Runtime    | Tokio             |
| Caching          | Redis             |
| Containerization | Docker            |

## 📚 Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official guide
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises
- [Actix-web Docs](https://actix.rs/) - Web framework
- [Zero To Production](https://www.zero2prod.com/) - Backend book

## 💡 Tips

1. **Jangan skip Ownership** (Phase 1.4) - Ini fondasi Rust!
2. Setiap file punya **exercises** di bagian bawah
3. Baca error messages - Rust punya pesan error terbaik
4. Gunakan `cargo clippy` untuk code quality
5. Test dengan `cargo test`

## 🏃 Running Examples

```bash
# Single file (fundamentals)
cd 01_fundamentals/01_variables
rustc main.rs -o main && ./main

# Cargo project (backend)
cd 04_backend/02_actix_intro
cargo run
```

---

_Happy Learning! 🦀_
