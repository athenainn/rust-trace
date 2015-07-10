#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};
    use core::cmp::PartialEq;
    use core::cmp::PartialOrd;

    use core::f64;

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
    fn partial_cmp_test1() {
	let a: A<T> = A { index: 68 as T };
	let other: A<T> = A { index: 500 as T };
	let result: Option<Ordering> = a.partial_cmp(&other);

	match result {
	    Some(v) => assert_eq!(v, Less),
	    None => assert!(false)
	}
    }

    #[test]
    fn partial_cmp_test2() {
	let a: A<T> = A { index: 68 as T };
	let other: A<T> = A { index: 68 as T };
	let result: Option<Ordering> = a.partial_cmp(&other);

	match result {
	    Some(v) => assert_eq!(v, Equal),
	    None => assert!(false)
	}
    }

    #[test]
    fn partial_cmp_test3() {
	let a: A<T> = A { index: 999 as T };
	let other: A<T> = A { index: 68 as T };
	let result: Option<Ordering> = a.partial_cmp(&other);

	match result {
	    Some(v) => assert_eq!(v, Greater),
	    None => assert!(false)
	}
    }

    #[test]
    fn partial_cmp_test4() {
	let a: A<T> = A { index: f64::NAN };
	let other: A<T> = A { index: 500 as T };
	let result: Option<Ordering> = a.partial_cmp(&other);

	assert_eq!(result, None::<Ordering>);
    }

    #[test]
    fn partial_cmp_test5() {
	let a: A<T> = A { index: 68 as T };
	let other: A<T> = A { index: f64::NAN };
	let result: Option<Ordering> = a.partial_cmp(&other);

	assert_eq!(result, None::<Ordering>);
    }

    #[test]
    fn partial_cmp_test6() {
	let a: A<T> = A { index: f64::NAN };
	let other: A<T> = A { index: f64::NAN };
	let result: Option<Ordering> = a.partial_cmp(&other);

	assert_eq!(result, None::<Ordering>);
    }
}
