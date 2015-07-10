#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // impl<A, B> PartialEq<[B]> for [A] where A: PartialEq<B> {
    //     fn eq(&self, other: &[B]) -> bool {
    //         self.len() == other.len() &&
    //             order::eq(self.iter(), other.iter())
    //     }
    //     fn ne(&self, other: &[B]) -> bool {
    //         self.len() != other.len() ||
    //             order::ne(self.iter(), other.iter())
    //     }
    // }

    // impl<T: Eq> Eq for [T] {}

    // impl<T: PartialOrd> PartialOrd for [T] {
    //     #[inline]
    //     fn partial_cmp(&self, other: &[T]) -> Option<Ordering> {
    //         order::partial_cmp(self.iter(), other.iter())
    //     }
    //     #[inline]
    //     fn lt(&self, other: &[T]) -> bool {
    //         order::lt(self.iter(), other.iter())
    //     }
    //     #[inline]
    //     fn le(&self, other: &[T]) -> bool {
    //         order::le(self.iter(), other.iter())
    //     }
    //     #[inline]
    //     fn ge(&self, other: &[T]) -> bool {
    //         order::ge(self.iter(), other.iter())
    //     }
    //     #[inline]
    //     fn gt(&self, other: &[T]) -> bool {
    //         order::gt(self.iter(), other.iter())
    //     }
    // }

    // impl<T: Ord> Ord for [T] {
    //     fn cmp(&self, other: &[T]) -> Ordering {
    //         order::cmp(self.iter(), other.iter())
    //     }
    // }

    type T = i32; // T: Ord

    #[test]
    fn le_test1() {
	let slice: &[T] = &[1, 2];
	let other: &[T] = &[2, 1, 0];
	let resule: bool = slice.le(other);

	assert_eq!(resule, true);
    }

    #[test]
    fn le_test2() {
	let slice: &[T] = &[1, 2];
	let other: &[T] = &[2, 1, 0];
	let resule: bool = slice <= other;

	assert_eq!(resule, true);
    }

    #[test]
    fn le_test3() {
	let slice: &[T] = &[3, 2];
	let other: &[T] = &[2, 1, 0];
	let resule: bool = slice.le(other);

	assert_eq!(resule, false);
    }

    #[test]
    fn le_test4() {
	let slice: &[T] = &[3, 2];
	let other: &[T] = &[2, 1, 0];
	let resule: bool = slice <= other;

	assert_eq!(resule, false);
    }

    #[test]
    fn le_test5() {
	let slice: &[T] = &[1, 2];
	let other: &[T] = &[1, 2];
	let resule: bool = slice.le(other);

	assert_eq!(resule, true);
    }

    #[test]
    fn le_test6() {
	let slice: &[T] = &[1, 2];
	let other: &[T] = &[1, 2];
	let resule: bool = slice <= other;

	assert_eq!(resule, true);
    }

    #[test]
    fn le_test7() {
	let slice: &[T] = &[1, 2];
	let other: &[T] = &[1, 2, 3];
	let resule: bool = slice.le(other);

	assert_eq!(resule, true);
    }

    #[test]
    fn le_test8() {
	let slice: &[T] = &[1, 2];
	let other: &[T] = &[1, 2, 3];
	let resule: bool = slice <= other;

	assert_eq!(resule, true);
    }

    #[test]
    fn le_test9() {
	let slice: &[T] = &[1, 2, 3];
	let other: &[T] = &[1, 2];
	let resule: bool = slice.le(other);

	assert_eq!(resule, false);
    }

    #[test]
    fn le_test10() {
	let slice: &[T] = &[1, 2, 3];
	let other: &[T] = &[1, 2];
	let resule: bool = slice <= other;

	assert_eq!(resule, false);
    }
}
