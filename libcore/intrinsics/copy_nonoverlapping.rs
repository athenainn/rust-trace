#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::copy_nonoverlapping;

    // pub fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize);

    type T = u8;

    #[test]
    fn copy_nonoverlapping_test1() {
	let mut buf: [T; 6] = [0, 1, 2, 3, 4, 5];
	let src: *const T = buf.as_ptr();
	let dst: *mut T = buf[2..].as_mut_ptr();
	let count: usize = 3;

	unsafe {
	    copy_nonoverlapping::<T>(src, dst, count);
	}

	assert_eq!(buf, [0, 1, 0, 1, 2, 5]);
    }

    #[test]
    fn copy_nonoverlapping_test2() {
	let mut buf: [T; 6] = [0, 1, 2, 3, 4, 5];
	let dst: *mut T = buf.as_mut_ptr();
	let src: *const T = buf[2..].as_ptr();
	let count: usize = 3;

	unsafe {
	    copy_nonoverlapping::<T>(src, dst, count);
	}

	assert_eq!(buf, [2, 3, 4, 3, 4, 5]);
    }
}
