#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::ExactSizeIterator;
    use core::iter::Inspect;

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

		// fn inspect<F>(self, f: F) -> Inspect<Self, F> where
		//     Self: Sized, F: FnMut(&Self::Item),
		// {
		//     Inspect{iter: self, f: f}
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

    // impl<I: ExactSizeIterator, F> ExactSizeIterator for Inspect<I, F> where
    //     F: FnMut(&I::Item),
    // {}

    struct F;

    type B = T;
    type Item = T;
    type Args<'a> = (&'a Item,);

    impl<'a> FnOnce<Args<'a>> for F {
	type Output = ();
	extern "rust-call" fn call_once(self, (item,): Args) -> Self::Output {
	    assert!(*item >= 0);
	    ()
	}
    }

    impl<'a> FnMut<Args<'a>> for F {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    assert!(*item >= 0);
	    ()
	}
    }

    #[test]
    fn len_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let f: F = F;
	let inspect: Inspect<A<T>, F> = a.inspect::<F>(f);

	assert_eq!(inspect.len(), 10);
    }

    #[test]
    fn len_test2() {
	let a: A<T> = A { begin: 0, end: 10 };
	let f: F = F;
	let mut inspect: Inspect<A<T>, F> = a.inspect::<F>(f);

	inspect.next();

	assert_eq!(inspect.len(), 9);
    }
}
