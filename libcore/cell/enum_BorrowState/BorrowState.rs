#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cell::BorrowState::{self, Reading, Writing, Unused};

    // pub enum BorrowState {
    //     /// The cell is currently being read, there is at least one active `borrow`.
    //     Reading,
    //     /// The cell is currently being written to, there is an active `borrow_mut`.
    //     Writing,
    //     /// There are no outstanding borrows on this cell.
    //     Unused,
    // }

    #[test]
    fn borrow_state_test1() {
    }
}
