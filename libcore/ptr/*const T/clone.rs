#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    // impl<T: ?Sized> Clone for *const T {
    //     #[inline]
    //     fn clone(&self) -> *const T {
    //         *self
    //     }
    // }

    type T = i32;

    #[test]
    fn eq_test1() {
	let x: T = 68;
	let ptr: *const T = &x;
	let clone: *const T = ptr.clone();

	assert_eq!(ptr, clone);
    }
}
