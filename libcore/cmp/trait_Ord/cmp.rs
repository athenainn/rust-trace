#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};
    use core::cmp::PartialEq;
    use core::cmp::PartialOrd;
    use core::cmp::Eq;
    use core::cmp::Ord;

    struct A<T> {
	index: T
    }

    impl PartialEq<A<T>> for A<T> {
	fn eq(&self, other: &A<T>) -> bool {
	    self.index == other.index
	}

	// fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
    }

    impl Eq for A<T> {}

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

    impl Ord for A<T> {
	fn cmp(&self, other: &Self) -> Ordering {
	    self.index.cmp(&(other.index))
	}
    }

    type T = i32;

    #[test]
    fn cmp_test1() {
	let a: A<T> = A { index: 68 as T };
	let other: A<T> = A { index: 500 as T };
	let result: Ordering = a.cmp(&other);

	assert_eq!(result, Less);
    }

    #[test]
    fn cmp_test2() {
	let a: A<T> = A { index: 68 as T };
	let other: A<T> = A { index: 68 as T };
	let result: Ordering = a.cmp(&other);

	assert_eq!(result, Equal);
    }

    #[test]
    fn cmp_test3() {
	let a: A<T> = A { index: 999 as T };
	let other: A<T> = A { index: 68 as T };
	let result: Ordering = a.cmp(&other);

	assert_eq!(result, Greater);
    }
}
