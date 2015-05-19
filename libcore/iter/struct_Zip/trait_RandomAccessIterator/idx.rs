#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::RandomAccessIterator;
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

    // impl<A, B> RandomAccessIterator for Zip<A, B> where
    //     A: RandomAccessIterator,
    //     B: RandomAccessIterator
    // {
    //     #[inline]
    //     fn indexable(&self) -> usize {
    //         cmp::min(self.a.indexable(), self.b.indexable())
    //     }
    //
    //     #[inline]
    //     fn idx(&mut self, index: usize) -> Option<(A::Item, B::Item)> {
    //         self.a.idx(index).and_then(|x| {
    //             self.b.idx(index).and_then(|y| {
    //                 Some((x, y))
    //             })
    //         })
    //     }
    // }

    #[test]
    fn idx_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let other: A<T> = A { begin: 10, end: 15 };

	type U = A<T>;
	let mut zip: Zip<A<T>, U> = a.zip::<U>(other);

	for i in 0..5 {
	    let x: Option<(T, T)> = zip.idx(i);
	    match x {
		Some(v) => assert_eq!(v, (i as T, 10 + i as T)),
		_ => assert!(false)
	    }
	}

	assert_eq!(zip.idx(5), None::<(T, T)>);
    }
}
