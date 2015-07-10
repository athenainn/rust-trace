#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::convert::From;

    struct A<T> {
	value: T
    }

    impl From<T> for A<T> {
	fn from(src: T) -> Self {
	    A::<T> { value: src }
	}
    }

    type T = i32;

    #[test]
    fn from_test1() {
	let value: T = 68;
	let a: A<T> = A::<T>::from(value);

	assert_eq!(a.value, value);
    }
}
