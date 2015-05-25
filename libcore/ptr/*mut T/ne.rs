#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    // impl<T: ?Sized> PartialEq for *mut T {
    //     #[inline]
    //     fn eq(&self, other: &*mut T) -> bool { *self == *other }
    // }

    type T = i32;

    #[test]
    fn ne_test1() {
	let mut x: T = 68;
	let ptr: *mut T = &mut x;
	let other: *mut T = ptr;
	let result: bool = ptr.ne(&other);

	assert_eq!(result, false);
    }

    #[test]
    fn ne_test2() {
	let mut x: T = 68;
	let ptr: *mut T = &mut x;
	let other: *mut T = ptr;
	let result: bool = ptr != other;

	assert_eq!(result, false);
    }
}
