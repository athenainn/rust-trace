#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Range;

    // impl<A: Step> ops::Range<A> {
    //     /// Creates an iterator with the same range, but stepping by the
    //     /// given amount at each iteration.
    //     ///
    //     /// The resulting iterator handles overflow by stopping.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(step_by, core)]
    //     /// for i in (0..10).step_by(2) {
    //     ///     println!("{}", i);
    //     /// }
    //     /// ```
    //     ///
    //     /// This prints:
    //     ///
    //     /// ```text
    //     /// 0
    //     /// 2
    //     /// 4
    //     /// 6
    //     /// 8
    //     /// ```
    //     #[unstable(feature = "step_by", reason = "recent addition")]
    //     pub fn step_by(self, by: A) -> StepBy<A, Self> {
    //         StepBy {
    //             step_by: by,
    //             range: self
    //         }
    //     }
    // }

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

    // macro_rules! range_exact_iter_impl {
    //     ($($t:ty)*) => ($(
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl ExactSizeIterator for ops::Range<$t> { }
    //     )*)
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

    // // Ranges of u64 and i64 are excluded because they cannot guarantee having
    // // a length <= usize::MAX, which is required by ExactSizeIterator.
    // range_exact_iter_impl!(usize u8 u16 u32 isize i8 i16 i32);

    // impl<A: Step + One + Clone> DoubleEndedIterator for ops::Range<A> where
    //     for<'a> &'a A: Add<&'a A, Output = A>,
    //     for<'a> &'a A: Sub<&'a A, Output = A>
    // {
    //     #[inline]
    //     fn next_back(&mut self) -> Option<A> {
    //         if self.start < self.end {
    //             self.end = &self.end - &A::one();
    //             Some(self.end.clone())
    //         } else {
    //             None
    //         }
    //     }
    // }

    macro_rules! range_next {
	($($t:ty)*) => ($({
	    let mut range: Range<$t> = 68..100;
	    let mut n: $t = 0;

	    loop {
		let x: Option<$t> = range.next_back();

		match x {
		    Some(v) => assert_eq!(v, (99 - n) as $t),
		    None => break
		}

		n += 1;
	    }

	    assert_eq!(n, 32);
	    assert_eq!(range.next_back(), None::<$t>);
	})*)
    }

    macro_rules! range_next_back {
	($($t:ty)*) => ($({
	    let mut range: Range<$t> = 68..100;
	    let mut n: usize = 0;

	    loop {
		let x: Option<$t> = range.next_back();

		match x {
		    Some(v) => assert_eq!(v, (68 + n) as $t),
		    None => break
		}

		n += 1;
	    }

	    assert_eq!(range.next(), None::<$t>);
	})*)
    }

    #[test]
    fn next_back_test1() {
	range_next!(usize u8 u16 u32 isize i8 i16 i32);
    }
}
