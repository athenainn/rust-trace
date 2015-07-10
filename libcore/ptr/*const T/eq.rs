#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    // impl<T: ?Sized> PartialEq for *const T {
    //     #[inline]
    //     fn eq(&self, other: &*const T) -> bool { *self == *other }
    // }

    type T = i32;

    #[test]
    fn eq_test1() {
	let x: T = 68;
	let ptr: *const T = &x;
	let other: *const T = ptr;
	let result: bool = ptr.eq(&other);

	assert_eq!(result, true);
    }

    #[test]
    fn eq_test2() {
	let x: T = 68;
	let ptr: *const T = &x;
	let other: *const T = ptr;
	let result: bool = ptr == other;

	assert_eq!(result, true);
    }
}
