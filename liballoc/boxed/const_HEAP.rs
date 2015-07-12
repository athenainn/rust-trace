#![feature(alloc, box_heap)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::HEAP;

    // pub const HEAP: () = ();

    #[test]
    fn heap_test1() {
	assert_eq!(HEAP, ());
    }
}
