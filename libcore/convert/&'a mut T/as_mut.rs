#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::convert::AsMut;

    struct A<T> {
	value: T
    }

    impl AsMut<T> for A<T> {
	fn as_mut(&mut self) -> &mut T {
	    &mut self.value
	}
    }

    // impl<'a, T: ?Sized, U: ?Sized> AsMut<U> for &'a mut T where T: AsMut<U> {
    //     fn as_mut(&mut self) -> &mut U {
    //         (*self).as_mut()
    //     }
    // }

    type T = i32;
    type U = T;

    #[test]
    fn as_mut_test1() {
	let value: T = 68;
	let mut a: A<T> = A { value: value };
	let a_refmut: &mut A<T> = &mut a;
	let as_ref: &mut U = a_refmut.as_mut();

	assert_eq!(*as_ref, value);
    }
}
