#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    //     pub unsafe fn offset(self, count: isize) -> *const T where T: Sized {
    //         intrinsics::offset(self, count)
    //     }

    type T = i32;

    #[test]
    fn offset_test1() {
	let x: T = 68;
	let ptr: *const T = &x;
	let count: isize = 1;
	let result: *const T = unsafe { ptr.offset(count) };

	assert_eq!(
	    result as isize - ptr as isize,
	    count * size_of::<T>() as isize
	);
    }
}
