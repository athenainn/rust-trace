#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::volatile_copy_memory;
    use core::intrinsics::min_align_of;

    // pub fn volatile_copy_memory<T>(dst: *mut T, src: *const T,  count: usize);

    type T = (u16, u32);

    #[test]
    fn volatile_copy_memory_test1() {
	let min_align: usize = unsafe { min_align_of::<T>() };
	assert_eq!(min_align, 4);

	let mut buf: [u8; 10] = [
	    255, 255, 255, 255, 255, 255, 255, 255, 255, 255
	];
	let dst: *mut T = buf.as_mut_ptr() as *mut T;
	let src: *const T = &(0x1234, 0x567890ab);
	let count: usize = 1;

	unsafe {
	    volatile_copy_memory::<T>(dst, src, count);
	}
	assert_eq!(buf, [0x34, 0x12, 0, 0, 0xab, 0x90, 0x78, 0x56, 255, 255]);

	let src: *const T = unsafe { (dst as *const u8).offset(2) as *const T };
	unsafe {
	    volatile_copy_memory::<T>(dst, src, count);
	}
	assert_eq!(buf, [0, 0, 0xab, 0x90, 0x78, 0x56, 255, 255, 255, 255]);
    }
}
