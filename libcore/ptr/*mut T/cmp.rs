#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{Less, Equal, Greater};

    // impl<T: ?Sized> Ord for *mut T {
    //     #[inline]
    //     fn cmp(&self, other: &*mut T) -> Ordering {
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
	let mut x: T = 68;
	let mut y: T = 500;
	let mut z: T = 999;
	let x_ptr: *mut T = &mut x;
	let y_ptr: *mut T = &mut y;
	let z_ptr: *mut T = &mut z;

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
