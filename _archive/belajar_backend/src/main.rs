use axum::{
    routing::{get, post},
    Router, Json,
};

use serde::{ Serialize, Deserialize};

// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .route("/", get(handler_utama));
//
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     println!("Server berjalan di local");
//
//     axum::serve(listener, app).await.unwrap();
// }
//
// async fn handler_utama() -> &'static str {
//     "Hallo ini backend pertamaku menggunakan bahasa rust"
// }

#[derive(Deserialize, Serialize)]
struct User {
    username: String,
    email: String,
}

#[derive(Deserialize, Serialize)]
struct Book {
    bookname: String,
    author: String,
}

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
    user_created: User,
}

#[derive(Serialize)]
struct ResponseMessageBook {
    message: String,
    book_created: Book,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/users", post(create_user)) // Route baru method POST
        .route("/books", post(create_book));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server berjalan di port 3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Server aktif!"
}

// Handler untuk POST user
// Axum otomatis memparsing JSON body ke struct User
async fn create_user(Json(payload): Json<User>) -> Json<ResponseMessage> {
    // Di sini biasanya kita simpan ke database
    println!("Menerima user baru: {}", payload.username);

    let response = ResponseMessage {
        message: "User berhasil dibuat!".to_string(),
        user_created: payload,
    };

    Json(response)
}

async fn create_book(Json(payload): Json<Book>) -> Json<ResponseMessageBook> {
    println!("Menerima book dibuat! {}", payload.bookname);

    let response = ResponseMessageBook {
        message: "Pinjaman Buku dibuat!".to_string(),
        book_created: payload,
    };

    Json(response)
}
