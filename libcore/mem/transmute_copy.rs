#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::mem::transmute_copy;

    // pub unsafe fn transmute_copy<T, U>(src: &T) -> U {
    //     // FIXME(#23542) Replace with type ascription.
    //     #![allow(trivial_casts)]
    //     ptr::read(src as *const T as *const U)
    // }

    type T = i32;
    type U = i32;

    #[test]
    fn transmute_copy_test1() {
	let src: T = 68;
	let copy: U = unsafe { transmute_copy::<T, U>(&src) };

	assert_eq!(src, copy);
    }
}
