pub mod borrow;
pub mod search;

pub fn list_books() {
    println!("\nList Books:\n");
}

pub fn admin() {
    super::book::borrow::internal_log();
    self::borrow::secret_book_id();
}