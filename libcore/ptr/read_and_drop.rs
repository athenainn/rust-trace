#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ptr::read_and_drop;

    // pub unsafe fn read_and_drop<T>(dest: *mut T) -> T {
    //     // Copy the data out from `dest`:
    //     let tmp = read(&*dest);
    //
    //     // Now mark `dest` as dropped:
    //     write_bytes(dest, mem::POST_DROP_U8, 1);
    //
    //     tmp
    // }

    // pub const POST_DROP_U8: u8 = 0x1d;

    type T = i32;

    #[test]
    fn read_and_drop_test1() {
	let mut dest: T = 68;
	let result: T = unsafe {
	    read_and_drop::<T>(&mut dest)
	};

	assert_eq!(result, 68);
	assert_eq!(dest, 0x1d1d1d1d);
    }
}
