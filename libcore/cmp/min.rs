#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};
    use core::cmp::PartialEq;
    use core::cmp::PartialOrd;
    use core::cmp::Eq;
    use core::cmp::Ord;
    use core::cmp::min;

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

	// fn le(&self, other: &Rhs) -> bool {
	//     match self.partial_cmp(other) {
	//         Some(Less) | Some(Equal) => true,
	//         _ => false,
	//     }
	// }
    }

    impl Ord for A<T> {
	fn cmp(&self, other: &Self) -> Ordering {
	    self.index.cmp(&(other.index))
	}
    }

    type T = i32;

    #[test]
    fn min_test1() {
	let v1: A<T> = A { index: 68 as T };
	let v2: A<T> = A { index: 500 as T };
	let result: A<T> = min::<A<T>>(v1, v2);

	assert_eq!(result.index, 68);
    }
}
