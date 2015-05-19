#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::DoubleEndedIterator;
    use core::iter::ExactSizeIterator;
    use core::iter::Rev;

    struct A<T> {
	begin: T,
	end: T
    }

    macro_rules! Iterator_impl {
	($T:ty) => {
	    impl Iterator for A<$T> {
		type Item = $T;

		fn next(&mut self) -> Option<Self::Item> {
		    if self.begin < self.end {
			let result = self.begin;
			self.begin = self.begin.wrapping_add(1);
			Some::<Self::Item>(result)
		    } else {
			None::<Self::Item>
		    }
		}

		fn size_hint(&self) -> (usize, Option<usize>) {
		    debug_assert!(self.begin <= self.end);
		    let exact: usize = (self.end - self.begin) as usize;
		    (exact, Some::<usize>(exact))
		}

		// fn rev(self) -> Rev<Self> where Self: Sized + DoubleEndedIterator {
		//     Rev{iter: self}
		// }
	    }
	}
    }

    impl DoubleEndedIterator for A<T> {
	fn next_back(&mut self) -> Option<Self::Item> {
	    if self.begin < self.end {
		self.end = self.end.wrapping_sub(1);
		Some::<Self::Item>(self.end)
	    } else {
		None::<Self::Item>
	    }
	}
    }

    impl ExactSizeIterator for A<T> {
	// fn len(&self) -> usize {
	//     let (lower, upper) = self.size_hint();
	//     // Note: This assertion is overly defensive, but it checks the invariant
	//     // guaranteed by the trait. If this trait were rust-internal,
	//     // we could use debug_assert!; assert_eq! will check all Rust user
	//     // implementations too.
	//     assert_eq!(upper, Some(lower));
	//     lower
	// }
    }

    type T = i32;
    Iterator_impl!(T);

    // pub trait ExactSizeIterator: Iterator {
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     /// Returns the exact length of the iterator.
    //     fn len(&self) -> usize {
    //         let (lower, upper) = self.size_hint();
    //         // Note: This assertion is overly defensive, but it checks the invariant
    //         // guaranteed by the trait. If this trait were rust-internal,
    //         // we could use debug_assert!; assert_eq! will check all Rust user
    //         // implementations too.
    //         assert_eq!(upper, Some(lower));
    //         lower
    //     }
    // }

    // impl<I> ExactSizeIterator for Rev<I>
    //     where I: ExactSizeIterator + DoubleEndedIterator {}

    #[test]
    fn len_test1() {
	let a: A<T> = A { begin: 1, end: 6 };
	let rev: Rev<A<T>> = a.rev();

	assert_eq!(rev.len(), 5);
    }

    #[test]
    fn len_test2() {
	let a: A<T> = A { begin: 1, end: 6 };
	let mut rev: Rev<A<T>> = a.rev();

	rev.next_back();

	assert_eq!(rev.len(), 4);
    }
}
