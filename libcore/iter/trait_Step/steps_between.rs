#![feature(core, step_trait)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Step;

    use core::cmp::PartialEq;
    use core::cmp::Ordering::{self, Less, Equal, Greater};
    use core::cmp::PartialOrd;

    struct A<T> {
	index: T
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

    type T = i32;

    #[test]
    fn steps_between_test1() {
	let a: A<T> = A { index: 68 };
	let b: A<T> = A { index: 100 };
	let by: A<T> = A { index: 10 };
	let result: Option<usize> = Step::steps_between(&a, &b, &by);

	match result {
	    Some(v) => assert_eq!(v, 4),
	    None => assert!(false)
	}
    }

    #[test]
    fn step_test2() {
	assert_eq!(usize::steps_between(&68, &100, &10), Some::<usize>(4));
	assert_eq!(u8::steps_between(&68, &100, &10), Some::<usize>(4));
	assert_eq!(u16::steps_between(&68, &100, &10), Some::<usize>(4));
	assert_eq!(u32::steps_between(&68, &100, &10), Some::<usize>(4));

	assert_eq!(u64::steps_between(&68, &100, &10), Some::<usize>(4));
    }

    #[test]
    fn step_test3() {
	assert_eq!(isize::steps_between(&68, &100, &10), Some::<usize>(4));
	assert_eq!(i8::steps_between(&68, &100, &10), Some::<usize>(4));
	assert_eq!(i16::steps_between(&68, &100, &10), Some::<usize>(4));
	assert_eq!(i32::steps_between(&68, &100, &10), Some::<usize>(4));

	assert_eq!(i64::steps_between(&68, &100, &10), Some::<usize>(4));
    }
}
