# 🦀 Rust Learning Path: Backend Developer Journey

> Roadmap pembelajaran Rust dari dasar hingga menjadi Backend Developer profesional dengan REST API.

## 📊 Learning Phases

| Phase | Folder             | Topik                                    | Durasi     |
| ----- | ------------------ | ---------------------------------------- | ---------- |
| 1     | `01_fundamentals/` | Variables, Ownership, Error Handling     | 2-4 minggu |
| 2     | `02_intermediate/` | Generics, Traits, Lifetimes, Testing     | 2-4 minggu |
| 3     | `03_advanced/`     | Async/Await, Concurrency, Smart Pointers | 3-4 minggu |
| 4     | `04_backend/`      | Actix-web, REST API, PostgreSQL, JWT     | 4-6 minggu |
| 5     | `05_production/`   | Docker, Logging, Caching, CI/CD          | 4-6 minggu |

## 🚀 Quick Start

```bash
# Phase 1 - Run fundamentals
cd 01_fundamentals/01_variables
rustc main.rs && ./main

# Phase 4 - Run REST API
cd 04_backend/02_actix_intro
cargo run
# Open http://localhost:8080
```

## 📁 Structure

```
rust-learning/
├── 01_fundamentals/      # ⭐ Start here!
│   ├── 01_variables/
│   ├── 02_control_flow/
│   ├── 03_functions/
│   ├── 04_ownership/     # 🔥 CRITICAL - jangan skip!
│   ├── 05_structs_enums/
│   ├── 06_pattern_matching/
│   └── 07_error_handling/
├── 02_intermediate/
├── 03_advanced/
│   └── 03_async/         # 🔥 Penting untuk backend
├── 04_backend/
│   ├── 02_actix_intro/   # Hello World API
│   └── 03_rest_api/      # Full CRUD API
├── 05_production/
├── rustlings/            # Interactive exercises
└── _archive/             # Previous learning projects
```

## 🛠️ Tech Stack

- **Framework**: Actix-web
- **Database**: PostgreSQL + SQLx
- **Auth**: JWT
- **Runtime**: Tokio (async)

## 📚 Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Actix-web Docs](https://actix.rs/)
- [Zero To Production in Rust](https://www.zero2prod.com/)

## 💡 Tips

1. **Jangan skip Ownership** - Ini fondasi Rust!
2. Gunakan `rustlings/` untuk latihan interaktif
3. Setiap file ada **exercises** di bagian bawah
4. Baca error messages - Rust punya pesan error terbaik

---

_Happy Learning! 🦀_
