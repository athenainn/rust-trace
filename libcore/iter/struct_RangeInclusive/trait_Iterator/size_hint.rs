#![feature(core, step_trait, zero_one)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::RangeInclusive;
    use core::iter::Step;
    use core::iter::range_inclusive;

    use core::clone::Clone;

    use core::num::One;

    use core::ops::Add;

    use core::cmp::PartialEq;
    use core::cmp::Ordering::{self, Less, Equal, Greater};
    use core::cmp::PartialOrd;

    #[derive(Debug)]
    struct A<T> {
	index: T
    }

    impl Clone for A<T> {
	fn clone(&self) -> Self {
	    A { index: self.index }
	}
    }

    impl One for A<T> {
	fn one() -> Self {
	    A { index: 100 as T }
	}
    }

    impl<'a> Add for &'a A<T> {
	type Output = A<T>;

	fn add(self, rhs: Self) -> Self::Output {
	    A { index: self.index + rhs.index }
	}
    }

    impl<T> PartialEq<A<T>> for A<T> where T: PartialEq {
	fn eq(&self, other: &A<T>) -> bool {
	    self.index == other.index
	}

	// fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
    }

    impl<T> PartialOrd<A<T>> for A<T> where T: PartialOrd {
	fn partial_cmp(&self, other: &A<T>) -> Option<Ordering> {
	    match (self.index <= other.index, self.index >= other.index) {
		(false, false) => None,
		(false, true) => Some(Greater),
		(true, false) => Some(Less),
		(true, true) => Some(Equal)
	    }
	}

	// fn lt(&self, other: &Rhs) -> bool {
	//     match self.partial_cmp(other) {
	//         Some(Less) => true,
	//         _ => false,
	//     }
	// }

	// fn le(&self, other: &Rhs) -> bool {
	//     match self.partial_cmp(other) {
	//         Some(Less) | Some(Equal) => true,
	//         _ => false,
	//     }
	// }

	// fn gt(&self, other: &Rhs) -> bool {
	//     match self.partial_cmp(other) {
	//         Some(Greater) => true,
	//         _ => false,
	//     }
	// }

	// fn ge(&self, other: &Rhs) -> bool {
	//     match self.partial_cmp(other) {
	//         Some(Greater) | Some(Equal) => true,
	//         _ => false,
	//     }
	// }
    }

    impl Step for A<T> {
	fn step(&self, by: &Self) -> Option<Self> {
	    let step: T = self.index.step(&by.index).unwrap();
	    Some::<Self>(A { index: step })
	}

	fn steps_between(start: &Self, end: &Self, by: &Self) -> Option<usize> {
	    let distance: usize =
		Step::steps_between(
		    &start.index, &end.index, &by.index
		).unwrap();

	    Some::<usize>(distance)
	}
    }

    // pub trait Step: PartialOrd {
    //     /// Steps `self` if possible.
    //     fn step(&self, by: &Self) -> Option<Self>;
    //
    //     /// Returns the number of steps between two step objects. The count is
    //     /// inclusive of `start` and exclusive of `end`.
    //     ///
    //     /// Returns `None` if it is not possible to calculate `steps_between`
    //     /// without overflow.
    //     fn steps_between(start: &Self, end: &Self, by: &Self) -> Option<usize>;
    // }


    // impl<A: Step + One> Iterator for ops::Range<A> where
    //     for<'a> &'a A: Add<&'a A, Output = A>
    // {
    //     type Item = A;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<A> {
    //         if self.start < self.end {
    //             let mut n = &self.start + &A::one();
    //             mem::swap(&mut n, &mut self.start);
    //             Some(n)
    //         } else {
    //             None
    //         }
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         match Step::steps_between(&self.start, &self.end, &A::one()) {
    //             Some(hint) => (hint, Some(hint)),
    //             None => (0, None)
    //         }
    //     }
    // }

    // impl<A> Iterator for RangeInclusive<A> where
    //     A: PartialEq + Step + One + Clone,
    //     for<'a> &'a A: Add<&'a A, Output = A>
    // {
    //     type Item = A;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<A> {
    //         self.range.next().or_else(|| {
    //             if !self.done && self.range.start == self.range.end {
    //                 self.done = true;
    //                 Some(self.range.end.clone())
    //             } else {
    //                 None
    //             }
    //         })
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         let (lo, hi) = self.range.size_hint();
    //         if self.done {
    //             (lo, hi)
    //         } else {
    //             let lo = lo.saturating_add(1);
    //             let hi = hi.and_then(|x| x.checked_add(1));
    //             (lo, hi)
    //         }
    //     }
    // }

    type T = i32;

    #[test]
    fn size_hint_test1() {
	let start: A<T> = A { index: 68 };
	let stop: A<T> = A { index: 500 };
	let range_inclusive: RangeInclusive<A<T>> = range_inclusive(start, stop);
	let (lower, upper): (usize, Option<usize>) = range_inclusive.size_hint();

	assert_eq!(lower, 6);
	assert_eq!(upper, Some::<usize>(6));
    }

    #[test]
    fn size_hint_test2() {
	let start: A<T> = A { index: 68 };
	let stop: A<T> = A { index: 500 };
	let mut range_inclusive: RangeInclusive<A<T>> = range_inclusive(start, stop);

	range_inclusive.next();
	let (lower, upper): (usize, Option<usize>) = range_inclusive.size_hint();

	assert_eq!(lower, 5);
	assert_eq!(upper, Some::<usize>(5));
    }

    #[test]
    fn size_hint_test3() {
	let start: A<T> = A { index: 68 };
	let stop: A<T> = A { index: 500 };
	let mut range_inclusive: RangeInclusive<A<T>> = range_inclusive(start, stop);

	for _ in 0..5 { range_inclusive.next(); }
	let (lower, upper): (usize, Option<usize>) = range_inclusive.size_hint();

	assert_eq!(lower, 1);
	assert_eq!(upper, Some::<usize>(1));
    }

    #[test]
    fn size_hint_test4() {
	let start: A<T> = A { index: 100 };
	let stop: A<T> = A { index: 500 };
	let mut range_inclusive: RangeInclusive<A<T>> = range_inclusive(start, stop);

	for _ in 0..5 { range_inclusive.next(); }
	let (lower, upper): (usize, Option<usize>) = range_inclusive.size_hint();

	assert_eq!(lower, 0);
	assert_eq!(upper, Some::<usize>(0));
    }
}
