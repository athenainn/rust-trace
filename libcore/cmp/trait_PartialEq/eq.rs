#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::PartialEq;

    struct A<T> {
	index: T
    }

    impl PartialEq<A<T>> for A<T> {
	fn eq(&self, other: &A<T>) -> bool {
	    self.index == other.index
	}

	// fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
    }

    type T = i32;

    #[test]
    fn eq_test1() {
	let a: A<T> = A { index: 68 };
	let other: A<T> = A { index: 68 };
	let result: bool = a.eq(&other);

	assert_eq!(result, true);
	assert_eq!(result, a == other);
    }

    #[test]
    fn eq_test2() {
	let a: A<T> = A { index: 68 };
	let other: A<T> = A { index: 500 };
	let result: bool = a.eq(&other);

	assert_eq!(result, false);
	assert_eq!(result, a == other);
    }
}
