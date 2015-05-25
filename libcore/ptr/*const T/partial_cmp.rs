#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};

    // impl<T: ?Sized> PartialOrd for *const T {
    //     #[inline]
    //     fn partial_cmp(&self, other: &*const T) -> Option<Ordering> {
    //         Some(self.cmp(other))
    //     }
    //
    //     #[inline]
    //     fn lt(&self, other: &*const T) -> bool { *self < *other }
    //
    //     #[inline]
    //     fn le(&self, other: &*const T) -> bool { *self <= *other }
    //
    //     #[inline]
    //     fn gt(&self, other: &*const T) -> bool { *self > *other }
    //
    //     #[inline]
    //     fn ge(&self, other: &*const T) -> bool { *self >= *other }
    // }

    type T = i32;

    #[test]
    fn partial_cmp_test1() {
	let x: T = 68;
	let y: T = 500;
	let z: T = 999;
	let x_ptr: *const T = &x;
	let y_ptr: *const T = &y;
	let z_ptr: *const T = &z;

	assert_eq!(x_ptr.partial_cmp(&x_ptr), Some::<Ordering>(Equal));
	assert_eq!(x_ptr.partial_cmp(&y_ptr), Some::<Ordering>(Greater));
	assert_eq!(x_ptr.partial_cmp(&z_ptr), Some::<Ordering>(Greater));

	assert_eq!(y_ptr.partial_cmp(&x_ptr), Some::<Ordering>(Less));
	assert_eq!(y_ptr.partial_cmp(&y_ptr), Some::<Ordering>(Equal));
	assert_eq!(y_ptr.partial_cmp(&z_ptr), Some::<Ordering>(Greater));

	assert_eq!(z_ptr.partial_cmp(&x_ptr), Some::<Ordering>(Less));
	assert_eq!(z_ptr.partial_cmp(&y_ptr), Some::<Ordering>(Less));
	assert_eq!(z_ptr.partial_cmp(&z_ptr), Some::<Ordering>(Equal));
    }
}
