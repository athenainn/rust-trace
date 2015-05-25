#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{Less, Equal, Greater};

    // impl<T: ?Sized> Ord for *const T {
    //     #[inline]
    //     fn cmp(&self, other: &*const T) -> Ordering {
    //         if self < other {
    //             Less
    //         } else if self == other {
    //             Equal
    //         } else {
    //             Greater
    //         }
    //     }
    // }

    type T = i32;

    #[test]
    fn cmp_test1() {
	let x: T = 68;
	let y: T = 500;
	let z: T = 999;
	let x_ptr: *const T = &x;
	let y_ptr: *const T = &y;
	let z_ptr: *const T = &z;

	assert_eq!(x_ptr.cmp(&x_ptr), Equal);
	assert_eq!(x_ptr.cmp(&y_ptr), Greater);
	assert_eq!(x_ptr.cmp(&z_ptr), Greater);

	assert_eq!(y_ptr.cmp(&x_ptr), Less);
	assert_eq!(y_ptr.cmp(&y_ptr), Equal);
	assert_eq!(y_ptr.cmp(&z_ptr), Greater);

	assert_eq!(z_ptr.cmp(&x_ptr), Less);
	assert_eq!(z_ptr.cmp(&y_ptr), Less);
	assert_eq!(z_ptr.cmp(&z_ptr), Equal);
    }
}
