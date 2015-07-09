#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::volatile_load;

    // pub fn volatile_load<T>(src: *const T) -> T;

    type T = u32;

    #[test]
    fn volatile_load_test1() {
	let value: T = 68;
	let src: *const T = &value;
	let result: T = unsafe {
	    volatile_load::<T>(src)
	};

	assert_eq!(result, 68);
    }
}
