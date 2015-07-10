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

    type T = i32;

    #[test]
    fn as_mut_test1() {
	let value: T = 68;
	let mut a: A<T> = A { value: value };

	{
	    let as_mut: &mut T = a.as_mut();
	    *as_mut = 500;
	}

	assert_eq!(a.value, 500);
    }
}
