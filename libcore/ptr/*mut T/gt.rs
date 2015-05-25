#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    // impl<T: ?Sized> PartialOrd for *mut T {
    //     #[inline]
    //     fn partial_cmp(&self, other: &*mut T) -> Option<Ordering> {
    //         Some(self.cmp(other))
    //     }
    //
    //     #[inline]
    //     fn lt(&self, other: &*mut T) -> bool { *self < *other }
    //
    //     #[inline]
    //     fn le(&self, other: &*mut T) -> bool { *self <= *other }
    //
    //     #[inline]
    //     fn gt(&self, other: &*mut T) -> bool { *self > *other }
    //
    //     #[inline]
    //     fn ge(&self, other: &*mut T) -> bool { *self >= *other }
    // }

    type T = i32;

    #[test]
    fn gt_test1() {
	let mut x: T = 68;
	let mut y: T = 500;
	let mut z: T = 999;
	let x_ptr: *mut T = &mut x;
	let y_ptr: *mut T = &mut y;
	let z_ptr: *mut T = &mut z;

	assert_eq!(x_ptr.gt(&x_ptr), false);
	assert_eq!(x_ptr.gt(&y_ptr), true);
	assert_eq!(x_ptr.gt(&z_ptr), true);

	assert_eq!(y_ptr.gt(&x_ptr), false);
	assert_eq!(y_ptr.gt(&y_ptr), false);
	assert_eq!(y_ptr.gt(&z_ptr), true);

	assert_eq!(z_ptr.gt(&x_ptr), false);
	assert_eq!(z_ptr.gt(&y_ptr), false);
	assert_eq!(z_ptr.gt(&z_ptr), false);
    }

    #[test]
    fn gt_test2() {
	let mut x: T = 68;
	let mut y: T = 500;
	let mut z: T = 999;
	let x_ptr: *mut T = &mut x;
	let y_ptr: *mut T = &mut y;
	let z_ptr: *mut T = &mut z;

	assert_eq!(x_ptr > x_ptr, false);
	assert_eq!(x_ptr > y_ptr, true);
	assert_eq!(x_ptr > z_ptr, true);

	assert_eq!(y_ptr > x_ptr, false);
	assert_eq!(y_ptr > y_ptr, false);
	assert_eq!(y_ptr > z_ptr, true);

	assert_eq!(z_ptr > x_ptr, false);
	assert_eq!(z_ptr > y_ptr, false);
	assert_eq!(z_ptr > z_ptr, false);
    }
}
