#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};
    use core::cmp::PartialEq;
    use core::cmp::PartialOrd;
    use core::cmp::partial_max;

    use core::f64;

    #[derive(Debug)]
    struct A<T> {
	index: T
    }

    impl PartialEq<A<T>> for A<T> {
	fn eq(&self, other: &A<T>) -> bool {
	    self.index == other.index
	}

	// fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
    }

    impl PartialOrd for A<T> {
	fn partial_cmp(&self, other: &A<T>) -> Option<Ordering>  {
	    match (self.index <= other.index, self.index >= other.index) {
		(true, false) => Some::<Ordering>(Less),
		(true, true) => Some::<Ordering>(Equal),
		(false, true) => Some::<Ordering>(Greater),
		(false, false) => None::<Ordering>
	    }
	}
    }

    type T = f64;

    #[test]
    fn partial_max_test1() {
	let v1: A<T> = A { index: 68 as T };
	let v2: A<T> = A { index: 500 as T };
	let result: Option<A<T>> = partial_max::<A<T>>(v1, v2);

	match result {
	    Some(v) => assert_eq!(v.index, 500 as T),
	    None => assert!(false)
	}
    }

    #[test]
    fn partial_max_test2() {
	let v1: A<T> = A { index: f64::NAN as T };
	let v2: A<T> = A { index: 500 as T };
	let result: Option<A<T>> = partial_max::<A<T>>(v1, v2);

	assert_eq!(result, None::<A<T>>);
    }

}
