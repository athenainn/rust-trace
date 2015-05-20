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

	// fn gt(&self, other: &Rhs) -> bool {
	//     match self.partial_cmp(other) {
	//         Some(Greater) => true,
	//         _ => false,
	//     }
	// }

    }

    type T = f64;

    #[test]
    fn gt_test1() {
	let a: A<T> = A { index: 68 as T };
	let other: A<T> = A { index: 68 as T };
	let result: bool = a.gt(&other);

	assert_eq!(result, false);
	assert_eq!(result, a > other);
    }

    #[test]
    fn gt_test2() {
	let a: A<T> = A { index: 68 as T };
	let other: A<T> = A { index: 500 as T };
	let result: bool = a.gt(&other);

	assert_eq!(result, false);
	assert_eq!(result, a > other);
    }

    #[test]
    fn gt_test3() {
	let a: A<T> = A { index: 999 as T };
	let other: A<T> = A { index: 500 as T };
	let result: bool = a.gt(&other);

	assert_eq!(result, true);
	assert_eq!(result, a > other);
    }

    #[test]
    fn gt_test4() {
	let a: A<T> = A { index: f64::NAN };
	let other: A<T> = A { index: 500 as T };
	let result: bool = a.gt(&other);

	assert_eq!(result, false);
	assert_eq!(result, a > other);
    }

    #[test]
    fn gt_test5() {
	let a: A<T> = A { index: 68 as T };
	let other: A<T> = A { index: f64::NAN };
	let result: bool = a.gt(&other);

	assert_eq!(result, false);
	assert_eq!(result, a > other);
    }

    #[test]
    fn gt_test6() {
	let a: A<T> = A { index: f64::NAN };
	let other: A<T> = A { index: f64::NAN };
	let result: bool = a.gt(&other);

	assert_eq!(result, false);
	assert_eq!(result, a > other);
    }
}
