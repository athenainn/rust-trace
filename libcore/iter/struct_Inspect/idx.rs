#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::RandomAccessIterator;
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

    impl RandomAccessIterator for A<T> {
	fn indexable(&self) -> usize {
	    let (exact, _) = self.size_hint();
	    exact
	}

	fn idx(&mut self, index: usize) -> Option<Self::Item> {
	    if index < self.indexable() {
		Some::<Self::Item>(self.begin + index as T)
	    } else {
		None::<Self::Item>
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<I: Iterator, F> Inspect<I, F> where F: FnMut(&I::Item) {
    //     #[inline]
    //     fn do_inspect(&mut self, elt: Option<I::Item>) -> Option<I::Item> {
    //         if let Some(ref a) = elt {
    //             (self.f)(a);
    //         }
    //
    //         elt
    //     }
    // }

    // impl<I: RandomAccessIterator, F> RandomAccessIterator for Inspect<I, F>
    //     where F: FnMut(&I::Item),
    // {
    //     #[inline]
    //     fn indexable(&self) -> usize {
    //         self.iter.indexable()
    //     }
    //
    //     #[inline]
    //     fn idx(&mut self, index: usize) -> Option<I::Item> {
    //         let element = self.iter.idx(index);
    //         self.do_inspect(element)
    //     }
    // }

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
    fn idx_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let f: F = F;
	let mut inspect: Inspect<A<T>, F> = a.inspect::<F>(f);

	for i in 0..10 {
	    let x: Option<T> = inspect.idx(i);
	    match x {
		Some(v) => { assert_eq!(v, i as T); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(inspect.idx(10), None::<T>);
    }
}
