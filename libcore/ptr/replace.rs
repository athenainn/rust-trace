#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ptr::replace;

    // pub unsafe fn replace<T>(dest: *mut T, mut src: T) -> T {
    //     mem::swap(mem::transmute(dest), &mut src); // cannot overlap
    //     src
    // }

    type T = i32;

    #[test]
    fn replace_test1() {
	let mut dest: T = 68;
	let src: T = 500;
	let result: T = unsafe {
	    replace::<T>(&mut dest, src)
	};

	assert_eq!(result, 68);
	assert_eq!(dest, 500);
    }
}
