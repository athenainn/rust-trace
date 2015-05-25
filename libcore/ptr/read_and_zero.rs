#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ptr::read_and_zero;

    // pub unsafe fn read_and_zero<T>(dest: *mut T) -> T {
    //     // Copy the data out from `dest`:
    //     let tmp = read(&*dest);
    //
    //     // Now zero out `dest`:
    //     write_bytes(dest, 0, 1);
    //
    //     tmp
    // }

    type T = i32;

    #[test]
    fn read_and_zero_test1() {
	let mut dest: T = 68;
	let result: T = unsafe {
	    read_and_zero::<T>(&mut dest)
	};

	assert_eq!(result, 68);
	assert_eq!(dest, 0);
    }
}
