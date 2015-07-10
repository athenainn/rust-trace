#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::volatile_set_memory;

    // pub fn volatile_set_memory<T>(dst: *mut T, val: u8, count: usize);

    type T = u32;

    #[test]
    fn volatile_set_memory_test1() {
	let mut buf: [T; 6] = [0, 1, 2, 3, 4, 5];
	let dst: *mut T = buf.as_mut_ptr();
	let val: u8 = 0x1d;
	let count: usize = 3;

	unsafe {
	    volatile_set_memory::<T>(dst, val, count);
	}

	assert_eq!(buf, [0x1d1d1d1d, 0x1d1d1d1d, 0x1d1d1d1d, 3, 4, 5]);
    }
}
