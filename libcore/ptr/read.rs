#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ptr::read;

    // pub unsafe fn read<T>(src: *const T) -> T {
    //     let mut tmp: T = mem::uninitialized();
    //     copy_nonoverlapping(src, &mut tmp, 1);
    //     tmp
    // }

    type T = i32;

    #[test]
    fn read_test1() {
	let src: T = 68;
	let result: T = unsafe {
	    read::<T>(&src)
	};

	assert_eq!(result, 68);
    }
}
