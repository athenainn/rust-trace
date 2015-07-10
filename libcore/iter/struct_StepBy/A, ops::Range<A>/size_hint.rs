#![feature(core, step_trait, step_by, zero_one)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::StepBy;
    use core::iter::Step;

    use core::clone::Clone;

    use core::ops::Range;
    use core::ops::Add;

    use core::num::Zero;

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

    impl<'a> Add for &'a A<T> {
	type Output = A<T>;

	fn add(self, rhs: Self) -> Self::Output {
	    A { index: self.index + rhs.index }
	}
    }

    impl Zero for A<T> {
	fn zero() -> Self {
	    A { index: 0 as T }
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

    // impl<A: Step + Zero + Clone> Iterator for StepBy<A, ops::Range<A>> {
    //     type Item = A;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<A> {
    //         let rev = self.step_by < A::zero();
    //         if (rev && self.range.start > self.range.end) ||
    //            (!rev && self.range.start < self.range.end)
    //         {
    //             match self.range.start.step(&self.step_by) {
    //                 Some(mut n) => {
    //                     mem::swap(&mut self.range.start, &mut n);
    //                     Some(n)
    //                 },
    //                 None => {
    //                     let mut n = self.range.end.clone();
    //                     mem::swap(&mut self.range.start, &mut n);
    //                     Some(n)
    //                 }
    //             }
    //         } else {
    //             None
    //         }
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         match Step::steps_between(&self.range.start,
    //                                   &self.range.end,
    //                                   &self.step_by) {
    //             Some(hint) => (hint, Some(hint)),
    //             None       => (0, None)
    //         }
    //     }
    // }

    type T = i32;

    #[test]
    fn size_hint_test1() {
	let a: A<T> = A { index: 68 };
	let b: A<T> = A { index: 500 };
	let by: A<T> = A { index: 100 };
	let stepby: StepBy<A<T>, Range<A<T>>> = (a..b).step_by(by);
	let (lower, upper): (usize, Option<usize>) = stepby.size_hint();

	assert_eq!(lower, 5);
	assert_eq!(upper, Some::<usize>(5));
    }

    #[test]
    fn size_hint_test2() {
	let a: A<T> = A { index: 68 };
	let b: A<T> = A { index: 500 };
	let by: A<T> = A { index: 100 };
	let mut stepby: StepBy<A<T>, Range<A<T>>> = (a..b).step_by(by);

	stepby.next();
	let (lower, upper): (usize, Option<usize>) = stepby.size_hint();

	assert_eq!(lower, 4);
	assert_eq!(upper, Some::<usize>(4));
    }

    #[test]
    fn size_hint_test3() {
	let a: A<T> = A { index: 68 };
	let b: A<T> = A { index: 500 };
	let by: A<T> = A { index: 100 };
	let mut stepby: StepBy<A<T>, Range<A<T>>> = (a..b).step_by(by);

	for _ in 0..5 { stepby.next(); }
	let (lower, upper): (usize, Option<usize>) = stepby.size_hint();

	assert_eq!(lower, 0);
	assert_eq!(upper, Some::<usize>(0));
    }
}
