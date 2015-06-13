#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::RandomAccessIterator;
    use core::iter::Cycle;

    use core::clone::Clone;

    struct A<T> {
	begin: T,
	end: T
    }

    impl Clone for A<T> {
	fn clone(&self) -> Self {
	    A {
		begin: self.begin,
		end: self.end
	    }
	}
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

		// fn cycle(self) -> Cycle<Self> where Self: Sized + Clone {
		//     Cycle{orig: self.clone(), iter: self}
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

    // impl<I> RandomAccessIterator for Cycle<I> where
    //     I: Clone + RandomAccessIterator,
    // {
    //     #[inline]
    //     fn indexable(&self) -> usize {
    //         if self.orig.indexable() > 0 {
    //             usize::MAX
    //         } else {
    //             0
    //         }
    //     }
    //
    //     #[inline]
    //     fn idx(&mut self, index: usize) -> Option<<I as Iterator>::Item> {
    //         let liter = self.iter.indexable();
    //         let lorig = self.orig.indexable();
    //         if lorig == 0 {
    //             None
    //         } else if index < liter {
    //             self.iter.idx(index)
    //         } else {
    //              self.orig.idx((index - liter) % lorig)
    //         }
    //     }
    // }

    #[test]
    fn idx_test1() {
	let a: A<T> = A { begin: 0, end: 3 };
	let mut cycle: Cycle<A<T>> = a.cycle();

	for n in 0..10 {
	    for x in 0..3 {
		let y: Option<T> = cycle.idx(3 * n + x);
		match y {
		    Some(v) => { assert_eq!(v, x as T); }
		    None => { assert!(false); }
		}
	    }
	}
    }
}
