#![feature(core, alloc, heap_api)]
extern crate core;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::heap::reallocate_inplace;
    use alloc::heap::allocate;
    use alloc::heap::usable_size;
    use alloc::heap::deallocate;

    use core::mem::size_of;

    // pub unsafe fn reallocate_inplace(ptr: *mut u8, old_size: usize, size: usize,
    //                                  align: usize) -> usize {
    //     check_size_and_alignment(size, align);
    //     imp::reallocate_inplace(ptr, old_size, size, align)
    // }

    type T = u8;

    #[test]
    fn reallocate_inplace_test1() {
	let size: usize = 5 * size_of::<T>(); // != 0
	let align: usize = 8; // power of 2
	let ptr: *mut u8 = unsafe { allocate(size, align) };
	assert!(ptr != 0 as *mut u8);

	let usable: usize = usable_size(size, align);
	assert_eq!(usable, 8);

	let old_size: usize = size;
	let size: usize = 6 * size_of::<T>(); // != 0
	// The `ptr` parameter must not be null.
	let result: usize = unsafe { reallocate_inplace(ptr, old_size, size, align) };
	assert_eq!(result, 8);

	let old_size: usize = size;
	unsafe { deallocate(ptr, old_size, align); }
    }

    #[test]
    fn reallocate_inplace_test2() {
	let size: usize = 8 * size_of::<T>(); // != 0
	let align: usize = 8; // power of 2
	let ptr: *mut u8 = unsafe { allocate(size, align) };
	assert!(ptr != 0 as *mut u8);

	let usable: usize = usable_size(size, align);
	assert_eq!(usable, 8);

	let old_size: usize = size;
	let size: usize = 100 * size_of::<T>(); // != 0
	// The `ptr` parameter must not be null.
	let result: usize = unsafe { reallocate_inplace(ptr, old_size, size, align) };
	assert_eq!(result, 8);

	let old_size: usize = size;
	unsafe { deallocate(ptr, old_size, align); }
    }
}
