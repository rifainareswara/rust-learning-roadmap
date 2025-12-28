mod library;

use library::book::list_books;
use library::book::borrow::borrow_book;
use library::book::admin;
use library::book::search::search_book;

fn main() {
    list_books();
    borrow_book();
    admin();
    search_book();
    // library::book::list_books();
    // library::book::borrow::borrow_book();
    //
    // library::book::search::search_book();
    //
    // library::book::admin();
}

// pub mod library {
//     pub mod book {
//         use crate::library;
//
//         pub fn list_books() {
//             println!("List book!");
//         }
//
//         pub mod borrow {
//             pub fn borrow_book() {
//                 println!("Borrow book!");
//             }
//
//             // hanya bisa di akses dari parent module
//             pub(super) fn internal_log() {
//                 println!("Internal log!");
//             }
//
//             pub(in crate::library::book) fn secret_book_id() {
//                 println!("secret_book_id");
//             }
//         }
//
//         pub fn admin() {
//             println!("Admin book:");
//
//             super::book::borrow::internal_log();
//             library::book::borrow::secret_book_id()
//         }
//
//         pub mod search {
//             pub fn search_book() {
//                 println!("Search book!");
//             }
//         }
//     }
// }