#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::ExactSizeIterator;
    use core::iter::Cloned;

    use core::marker::PhantomData;

    struct A<'a, T: 'a + Clone> {
	begin: T,
	end: T,
	marker: PhantomData<&'a T>
    }

    macro_rules! Iterator_impl {
	($T:ty) => {
	    impl<'a> Iterator for A<'a, $T> {
		type Item = &'a $T;

		fn next(&mut self) -> Option<&'a $T> {
		    unsafe {
			static mut result: $T = 0;

			if self.begin < self.end {
			    result = self.begin;
			    self.begin = self.begin.wrapping_add(1);
			    Some::<&'a $T>(&result)
			} else {
			    None::<&'a $T>
			}
		    }
		}

		fn size_hint(&self) -> (usize, Option<usize>) {
		    debug_assert!(self.begin <= self.end);
		    let exact: usize = (self.end - self.begin) as usize;
		    (exact, Some::<usize>(exact))
		}

		// fn cloned<'a, T: 'a>(self) -> Cloned<Self>
		//     where Self: Sized + Iterator<Item=&'a T>, T: Clone
		// {
		//     Cloned { it: self }
		// }
	    }
	}
    }

    impl<'a> ExactSizeIterator for A<'a, T> {
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

    type T = i32; // T: Clone
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

    // impl<'a, I, T: 'a> ExactSizeIterator for Cloned<I>
    //     where I: ExactSizeIterator<Item=&'a T>, T: Clone
    // {}

    #[test]
    fn len_test1() {
	let a: A<T> = A { begin: 0, end: 10, marker: PhantomData::<&T> };
	let cloned: Cloned<A<T>> = a.cloned::<T>();

	assert_eq!(cloned.len(), 10);
    }

    #[test]
    fn len_test2() {
	let a: A<T> = A { begin: 0, end: 10, marker: PhantomData::<&T> };
	let mut cloned: Cloned<A<T>> = a.cloned::<T>();

	cloned.next();

	assert_eq!(cloned.len(), 9);
    }
}
