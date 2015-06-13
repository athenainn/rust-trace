#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::ExactSizeIterator;
    use core::iter::DoubleEndedIterator;
    use core::iter::Zip;

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

		// fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter> where
		//     Self: Sized, U: IntoIterator
		// {
		//     Zip{a: self, b: other.into_iter()}
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

    // impl<A, B> DoubleEndedIterator for Zip<A, B> where
    //     A: DoubleEndedIterator + ExactSizeIterator,
    //     B: DoubleEndedIterator + ExactSizeIterator,
    // {
    //     #[inline]
    //     fn next_back(&mut self) -> Option<(A::Item, B::Item)> {
    //         let a_sz = self.a.len();
    //         let b_sz = self.b.len();
    //         if a_sz != b_sz {
    //             // Adjust a, b to equal length
    //             if a_sz > b_sz {
    //                 for _ in 0..a_sz - b_sz { self.a.next_back(); }
    //             } else {
    //                 for _ in 0..b_sz - a_sz { self.b.next_back(); }
    //             }
    //         }
    //         match (self.a.next_back(), self.b.next_back()) {
    //             (Some(x), Some(y)) => Some((x, y)),
    //             (None, None) => None,
    //             _ => unreachable!(),
    //         }
    //     }
    // }

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

    #[test]
    fn next_back_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let other: A<T> = A { begin: 10, end: 15 };

	type U = A<T>;
	let mut zip: Zip<A<T>, U> = a.zip::<U>(other);

	assert_eq!(zip.next_back(), Some((4, 14)));
	assert_eq!(zip.next_back(), Some((3, 13)));
	assert_eq!(zip.next_back(), Some((2, 12)));
	assert_eq!(zip.next_back(), Some((1, 11)));
	assert_eq!(zip.next_back(), Some((0, 10)));
    }
}
