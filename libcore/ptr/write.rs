#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ptr::write;

    // pub unsafe fn write<T>(dst: *mut T, src: T) {
    //     intrinsics::move_val_init(&mut *dst, src)
    // }

    type T = i32;

    #[test]
    fn write_test1() {
	let mut dst: T = 0;
	let src: T = 68;

	unsafe {
	    write::<T>(&mut dst, src)
	};

	assert_eq!(dst, 68);
    }
}
