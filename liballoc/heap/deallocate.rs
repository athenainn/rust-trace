#![feature(core, alloc, heap_api)]
extern crate core;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::heap::deallocate;
    use alloc::heap::allocate;
    use alloc::heap::usable_size;

    use core::mem::size_of;

    // pub unsafe fn deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    //     imp::deallocate(ptr, old_size, align)
    // }

    type T = u16;

    #[test]
    fn deallocate_test1() {
	let size: usize = 10 * size_of::<T>(); // != 0
	let align: usize = 4; // power of 2
	let ptr: *mut u8 = unsafe { allocate(size, align) };
	assert!(ptr != 0 as *mut u8);

	let usable: usize = usable_size(size, align);
	assert_eq!(usable, 32);

	let old_size: usize = size;
	unsafe { deallocate(ptr, old_size, align); }
    }
}
