#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::RandomAccessIterator;
    use core::iter::Map;

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

		// fn map<B, F>(self, f: F) -> Map<Self, F> where
		//     Self: Sized, F: FnMut(Self::Item) -> B,
		// {
		//     Map{iter: self, f: f}
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

    // impl<B, I: RandomAccessIterator, F> RandomAccessIterator for Map<I, F> where
    //     F: FnMut(I::Item) -> B,
    // {
    //     #[inline]
    //     fn indexable(&self) -> usize {
    //         self.iter.indexable()
    //     }
    //
    //     #[inline]
    //     fn idx(&mut self, index: usize) -> Option<B> {
    //         self.iter.idx(index).map(|a| (self.f)(a))
    //     }
    // }

    struct F;

    type B = T;
    type Item = T;
    type Args = (Item,);

    impl FnOnce<Args> for F {
	type Output = B;
	extern "rust-call" fn call_once(self, (item,): Args) -> Self::Output {
	    -item
	}
    }

    impl FnMut<Args> for F {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    -item
	}
    }

    #[test]
    fn indexable_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let f: F = F;
	let map: Map<A<T>, F> = a.map::<B, F>(f);

	assert_eq!(map.indexable(), 10);
    }

    #[test]
    fn indexable_test2() {
	let a: A<T> = A { begin: 0, end: 10 };
	let f: F = F;
	let mut map: Map<A<T>, F> = a.map::<B, F>(f);

	map.next();

	assert_eq!(map.indexable(), 9);
    }
}
