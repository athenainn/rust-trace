#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    // impl<T: ?Sized> Clone for *mut T {
    //     #[inline]
    //     fn clone(&self) -> *mut T {
    //         *self
    //     }
    // }

    type T = i32;

    #[test]
    fn eq_test1() {
	let mut x: T = 68;
	let ptr: *mut T = &mut x;
	let clone: *mut T = ptr.clone();

	assert_eq!(ptr, clone);
    }
}
