#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Deref;

    struct A<T> {
	value: T
    }

    // impl<'a, T: ?Sized> Deref for &'a T {
    //     type Target = T;
    //
    //     fn deref(&self) -> &T { *self }
    // }

    type T = i32;

    #[test]
    fn deref_test1() {
	let a: A<T> = A { value: 68 };
	let a_ptr: &A<T> = &a;

	assert_eq!(a_ptr.value, 68);
	assert_eq!((*a_ptr).value, 68);
	assert_eq!(a_ptr.deref().value, 68);
	assert_eq!((*a_ptr.deref()).value, 68);
    }
}
