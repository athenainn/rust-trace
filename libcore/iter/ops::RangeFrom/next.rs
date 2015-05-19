#![feature(core, step_trait, zero_one)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Step;

    use core::num::One;

    use core::ops::RangeFrom;
    use core::ops::Add;

    use core::cmp::PartialEq;
    use core::cmp::Ordering::{self, Less, Equal, Greater};
    use core::cmp::PartialOrd;

    #[derive(Debug)]
    struct A<T> {
	index: T
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

    // impl<A: Step> RangeFrom<A> {
    //     /// Creates an iterator starting at the same point, but stepping by
    //     /// the given amount at each iteration.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```ignore
    //     /// for i in (0u8..).step_by(2) {
    //     ///     println!("{}", i);
    //     /// }
    //     /// ```
    //     ///
    //     /// This prints all even `u8` values.
    //     #[unstable(feature = "step_by", reason = "recent addition")]
    //     pub fn step_by(self, by: A) -> StepBy<A, Self> {
    //         StepBy {
    //             step_by: by,
    //             range: self
    //         }
    //     }
    // }

    // impl<A: Step + One> Iterator for ops::RangeFrom<A> where
    //     for<'a> &'a A: Add<&'a A, Output = A>
    // {
    //     type Item = A;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<A> {
    //         let mut n = &self.start + &A::one();
    //         mem::swap(&mut n, &mut self.start);
    //         Some(n)
    //     }
    // }

    type T = i32;

    #[test]
    fn next_test1() {
	let a: A<T> = A { index: 68 };
	let mut rangefrom: RangeFrom<A<T>> = RangeFrom::<A<T>> { start: a };

	for n in 0..5 {
	    let x: Option<A<T>> = rangefrom.next();
	    match x {
		Some(v) => assert_eq!(v.index, 68 + n * 100),
		None => assert!(false)
	    }
	}

	assert_eq!(rangefrom.next(), Some::<A<T>>(A { index: 568}));
    }
}
