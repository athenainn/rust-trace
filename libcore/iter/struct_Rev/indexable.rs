#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::DoubleEndedIterator;
    use core::iter::RandomAccessIterator;
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

    // impl<I> RandomAccessIterator for Rev<I>
    //     where I: DoubleEndedIterator + RandomAccessIterator
    // {
    //     #[inline]
    //     fn indexable(&self) -> usize { self.iter.indexable() }
    //     #[inline]
    //     fn idx(&mut self, index: usize) -> Option<<I as Iterator>::Item> {
    //         let amt = self.indexable();
    //         if amt > index {
    //             self.iter.idx(amt - index - 1)
    //         } else {
    //             None
    //         }
    //     }
    // }

    #[test]
    fn indexable_test1() {
	let a: A<T> = A { begin: 1, end: 6 };
	let rev: Rev<A<T>> = a.rev();

	assert_eq!(rev.indexable(), 5);
    }

    #[test]
    fn indexable_test2() {
	let a: A<T> = A { begin: 1, end: 6 };
	let mut rev: Rev<A<T>> = a.rev();

	rev.next();

	assert_eq!(rev.indexable(), 4);
    }
}
