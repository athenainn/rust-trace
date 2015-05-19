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

    // macro_rules! step_impl_unsigned {
    //     ($($t:ty)*) => ($(
    //         impl Step for $t {
    //             #[inline]
    //             fn step(&self, by: &$t) -> Option<$t> {
    //                 (*self).checked_add(*by)
    //             }
    //             #[inline]
    //             #[allow(trivial_numeric_casts)]
    //             fn steps_between(start: &$t, end: &$t, by: &$t) -> Option<usize> {
    //                 if *by == 0 { return None; }
    //                 if *start < *end {
    //                     // Note: We assume $t <= usize here
    //                     let diff = (*end - *start) as usize;
    //                     let by = *by as usize;
    //                     if diff % by > 0 {
    //                         Some(diff / by + 1)
    //                     } else {
    //                         Some(diff / by)
    //                     }
    //                 } else {
    //                     Some(0)
    //                 }
    //             }
    //         }
    //     )*)
    // }

    // macro_rules! step_impl_signed {
    //     ($($t:ty)*) => ($(
    //         impl Step for $t {
    //             #[inline]
    //             fn step(&self, by: &$t) -> Option<$t> {
    //                 (*self).checked_add(*by)
    //             }
    //             #[inline]
    //             #[allow(trivial_numeric_casts)]
    //             fn steps_between(start: &$t, end: &$t, by: &$t) -> Option<usize> {
    //                 if *by == 0 { return None; }
    //                 let mut diff: usize;
    //                 let mut by_u: usize;
    //                 if *by > 0 {
    //                     if *start >= *end {
    //                         return Some(0);
    //                     }
    //                     // Note: We assume $t <= isize here
    //                     // Use .wrapping_sub and cast to usize to compute the
    //                     // difference that may not fit inside the range of isize.
    //                     diff = (*end as isize).wrapping_sub(*start as isize) as usize;
    //                     by_u = *by as usize;
    //                 } else {
    //                     if *start <= *end {
    //                         return Some(0);
    //                     }
    //                     diff = (*start as isize).wrapping_sub(*end as isize) as usize;
    //                     by_u = (*by as isize).wrapping_mul(-1) as usize;
    //                 }
    //                 if diff % by_u > 0 {
    //                     Some(diff / by_u + 1)
    //                 } else {
    //                     Some(diff / by_u)
    //                 }
    //             }
    //         }
    //     )*)
    // }

    // macro_rules! step_impl_no_between {
    //     ($($t:ty)*) => ($(
    //         impl Step for $t {
    //             #[inline]
    //             fn step(&self, by: &$t) -> Option<$t> {
    //                 (*self).checked_add(*by)
    //             }
    //             #[inline]
    //             fn steps_between(_a: &$t, _b: &$t, _by: &$t) -> Option<usize> {
    //                 None
    //             }
    //         }
    //     )*)
    // }

    // step_impl_unsigned!(usize u8 u16 u32);
    // step_impl_signed!(isize i8 i16 i32);
    // #[cfg(target_pointer_width = "64")]
    // step_impl_unsigned!(u64);
    // #[cfg(target_pointer_width = "64")]
    // step_impl_signed!(i64);
    // #[cfg(target_pointer_width = "32")]
    // step_impl_no_between!(u64 i64);

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
    fn range_inclusive_test1() {
	let start: A<T> = A { index: 68 };
	let stop: A<T> = A { index: 500 };
	let mut range_inclusive: RangeInclusive<A<T>> = range_inclusive(start, stop);

	for n in 0..5 {
	    let y: Option<A<T>> = range_inclusive.next();
	    match y {
		Some(v) => assert_eq!(v.index, 68 + n * 100),
		None => assert!(false)
	    }
	}

	assert_eq!(range_inclusive.next(), None::<A<T>>)
    }
}
