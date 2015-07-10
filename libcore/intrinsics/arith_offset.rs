#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::arith_offset;

    use core::intrinsics::size_of;

    // pub fn arith_offset<T>(dst: *const T, offset: isize) -> *const T;

    type T = i32;

    #[test]
    fn arith_offset_test1() {
	let x: T = 68;
	let dst: *const T = &x;
	let count: isize = unsafe { size_of::<T>() } as isize;
	let ptr: *const T = unsafe {
	    arith_offset::<i8>(dst as *const i8, count) as *const T
	};

	assert_eq!(ptr as isize - dst as isize, count);
    }
}
