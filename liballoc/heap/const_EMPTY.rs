#![feature(alloc, heap_api)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::heap::EMPTY;

    // pub const EMPTY: *mut () = 0x1 as *mut ();

    #[test]
    fn empty_test1() {
	assert_eq!(EMPTY, 0x1 as *mut ());
    }
}
