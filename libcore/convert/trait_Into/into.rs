#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::convert::Into;

    struct A<T> {
	value: T
    }

    impl Into<T> for A<T> {
	fn into(self) -> T {
	    self.value
	}
    }

    type T = i32;

    #[test]
    fn into_test1() {
	let value: T = 68;
	let a: A<T> = A { value: value };
	let into: T = a.into();

	assert_eq!(into, value);
    }
}
