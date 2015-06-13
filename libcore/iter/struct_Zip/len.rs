#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::ExactSizeIterator;
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

    // impl<A, B> ExactSizeIterator for Zip<A, B>
    //     where A: ExactSizeIterator, B: ExactSizeIterator {}

    #[test]
    fn len_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let other: A<T> = A { begin: 10, end: 15 };

	type U = A<T>;
	let zip: Zip<A<T>, U> = a.zip::<U>(other);
	let len: usize = zip.len();

	assert_eq!(len, 5);
    }

    #[test]
    fn len_test2() {
	let a: A<T> = A { begin: 0, end: 10 };
	let other: A<T> = A { begin: 10, end: 15 };

	type U = A<T>;
	let mut zip: Zip<A<T>, U> = a.zip::<U>(other);
	zip.next();

	let len: usize = zip.len();

	assert_eq!(len, 4);
    }
}
