mod copy_and_move;
mod reference_and_borrowing;
mod scope_and_memory;
mod slices;

pub fn ownership() {
    println!("\n\n--- Ownership and Scopes ---\n");
    copy_and_move::copy_and_move();
    scope_and_memory::scope_and_memory();
    reference_and_borrowing::reference_and_borrowing();
    slices::slices();
}
