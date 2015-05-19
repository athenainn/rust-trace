#![feature(core, step_trait, step_by, zero_one)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::StepBy;
    use core::iter::Step;

    use core::clone::Clone;

    use core::num::Zero;

    use core::cmp::PartialEq;
    use core::cmp::Ordering::{self, Less, Equal, Greater};
    use core::cmp::PartialOrd;

    use core::ops::Range;

    struct A<T> {
	index: T
    }

    impl Clone for A<T> {
	fn clone(&self) -> Self {
	    A { index: self.index }
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

    type T = i32;

    #[test]
    fn step_by_test1() {
	let a: A<T> = A { index: 68 };
	let b: A<T> = A { index: 500 };
	let by: A<T> = A { index: 100 };
	let stepby: StepBy<A<T>, Range<A<T>>> = (a..b).step_by(by);
	let mut n: usize = 0;

	for _ in stepby { n += 1; }

	assert_eq!(n, 5);
    }
}
