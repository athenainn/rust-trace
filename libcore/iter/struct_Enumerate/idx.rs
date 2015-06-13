#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::RandomAccessIterator;
    use core::iter::Enumerate;

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

		// fn enumerate(self) -> Enumerate<Self> where Self: Sized {
		//     Enumerate { iter: self, count: 0 }
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

    // impl<I> RandomAccessIterator for Enumerate<I> where I: RandomAccessIterator {
    //     #[inline]
    //     fn indexable(&self) -> usize {
    //         self.iter.indexable()
    //     }
    //
    //     #[inline]
    //     fn idx(&mut self, index: usize) -> Option<(usize, <I as Iterator>::Item)> {
    //         // Can safely add, `ExactSizeIterator` (ancestor of
    //         // `RandomAccessIterator`) promises that the number of elements fits
    //         // into a `usize`.
    //         self.iter.idx(index).map(|a| (self.count + index, a))
    //     }
    // }

    #[test]
    fn idx_test1() {
	let a: A<T> = A { begin: 10, end: 20 };
	let mut enumerate: Enumerate<A<T>> = a.enumerate();

	for i in 0..10 {
	    let x: Option<(usize, T)> = enumerate.idx(i);
	    match x {
		Some(v) => { assert_eq!(v, (i, 10 + i as T)); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(enumerate.idx(10), None::<(usize, T)>);
    }
}
