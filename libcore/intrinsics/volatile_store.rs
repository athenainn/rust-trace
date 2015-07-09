#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::volatile_store;

    // pub fn volatile_store<T>(dst: *mut T, val: T);

    type T = u32;

    #[test]
    fn volatile_store_test1() {
	let mut value: T = 68;
	let dst: *mut T = &mut value;
	let val: T = 500;

	unsafe {
	    volatile_store::<T>(dst, val);
	}

	assert_eq!(value, 500);
    }
}
