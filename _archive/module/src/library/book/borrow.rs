pub fn borrow_book() {
    println!("\nBorrow Book:\n");
}

pub(super) fn internal_log() {
    println!("\nInternal Log:\n");
}

pub(in crate::library::book) fn secret_book_id() {
    println!("\nSecret Book ID:\n");
}