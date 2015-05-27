#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::convert::AsRef;

    struct A<T> {
	value: T
    }

    impl AsRef<T> for A<T> {
	fn as_ref(&self) -> &T {
	    &self.value
	}
    }

    // impl<'a, T: ?Sized, U: ?Sized> AsRef<U> for &'a mut T where T: AsRef<U> {
    //     fn as_ref(&self) -> &U {
    //         <T as AsRef<U>>::as_ref(*self)
    //     }
    // }

    type T = i32;
    type U = T;

    #[test]
    fn as_ref_test1() {
	let value: T = 68;
	let mut a: A<T> = A { value: value };
	let a_refmut: &mut A<T> = &mut a;
	let as_ref: &U = a_refmut.as_ref();

	assert_eq!(*as_ref, value);
    }
}
