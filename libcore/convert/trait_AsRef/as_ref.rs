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

    type T = i32;

    #[test]
    fn as_ref_test1() {
	let value: T = 68;
	let a: A<T> = A { value: value };
	let as_ref: &T = a.as_ref();

	assert_eq!(*as_ref, value);
    }
}
