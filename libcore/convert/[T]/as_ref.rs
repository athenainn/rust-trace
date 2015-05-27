#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::convert::AsRef;

    // impl<T> AsRef<[T]> for [T] {
    //     fn as_ref(&self) -> &[T] {
    //         self
    //     }
    // }

    // impl<'a, T: ?Sized, U: ?Sized> AsRef<U> for &'a T where T: AsRef<U> {
    //     fn as_ref(&self) -> &U {
    //         <T as AsRef<U>>::as_ref(*self)
    //     }
    // }

    type T = i32;

    #[test]
    fn as_ref_test1() {
	let slice: &[T] = &[68, 500, 999];
	let as_ref: &[T] = (*slice).as_ref();

	assert_eq!(as_ref, slice);
    }

    #[test]
    fn as_ref_test2() {
	let slice: &[T] = &[68, 500, 999];
	let as_ref: &[T] = slice.as_ref();

	assert_eq!(as_ref, slice);
    }
}
