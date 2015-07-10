#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Deref;

    struct A<T> {
	value: T
    }

    impl Deref for A<T> {
	type Target = T;

	fn deref<'a>(&'a self) -> &'a Self::Target {
	    &self.value
	}
    }

    type T = i32;

    #[test]
    fn deref_test1() {
	let a: A<T> = A { value: 68 };

	assert_eq!(*a, 68);
	assert_eq!(a.deref(), &a.value);
	assert_eq!(*a.deref(), 68);
    }
}
