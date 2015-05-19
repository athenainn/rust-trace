#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::RandomAccessIterator;
    use core::iter::Skip;

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

		// fn skip(self, n: usize) -> Skip<Self> where Self: Sized {
		//     Skip{iter: self, n: n}
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

    // impl<I> RandomAccessIterator for Skip<I> where I: RandomAccessIterator{
    //     #[inline]
    //     fn indexable(&self) -> usize {
    //         self.iter.indexable().saturating_sub(self.n)
    //     }
    //
    //     #[inline]
    //     fn idx(&mut self, index: usize) -> Option<<I as Iterator>::Item> {
    //         if index >= self.indexable() {
    //             None
    //         } else {
    //             self.iter.idx(index + self.n)
    //         }
    //     }
    // }

    #[test]
    fn indexable_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let n: usize = 3;
	let skip: Skip<A<T>> = a.skip(n);
	let indexable: usize = skip.indexable();

	assert_eq!(indexable, 10 - n);
    }

    #[test]
    fn indexable_test2() {
	let a: A<T> = A { begin: 0, end: 10 };
	let n: usize = 3;
	let mut skip: Skip<A<T>> = a.skip(n);

	skip.next();
	let indexable: usize = skip.indexable();

	assert_eq!(indexable, 10 - n - 1);
    }
}
