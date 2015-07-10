#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::offset;

    use core::intrinsics::size_of;

    // pub fn offset<T>(dst: *const T, offset: isize) -> *const T;

    type T = i32;

    #[test]
    fn offset_test1() {
	let x: T = 68;
	let dst: *const T = &x;
	let count: isize = 1;
	let ptr: *const T = unsafe {
	    offset::<T>(dst, count)
	};

	assert_eq!(
	    ptr as isize - dst as isize,
	    count * unsafe { size_of::<T>() } as isize
	);
    }
}
