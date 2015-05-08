#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    struct A {
	value: u32
    }

    impl Default for A {
	fn default() -> Self
	{
	    A { value: 68  }
	}
    }

    #[test]
    fn unwrap_or_default_test1() {
	type T = A;

	let x: Option<T> = None::<T>;
	let y: T = x.unwrap_or_default();

	assert_eq!(y.value, 68);
    }

    #[test]
    fn unwrap_or_default_test2() {
	type T = A;

	let x: Option<T> = Some::<T>(A { value: 500 });
	let y: T = x.unwrap_or_default();

	assert_eq!(y.value, 500);
    }
}
