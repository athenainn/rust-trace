#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::RandomAccessIterator;
    use core::iter::Chain;

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


		// fn chain<U>(self, other: U) -> Chain<Self, U::IntoIter> where
		//     Self: Sized, U: IntoIterator<Item=Self::Item>,
		// {
		//     Chain{a: self, b: other.into_iter(), flag: false}
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

    // impl<A, B> RandomAccessIterator for Chain<A, B> where
    //     A: RandomAccessIterator,
    //     B: RandomAccessIterator<Item = A::Item>,
    // {
    //     #[inline]
    //     fn indexable(&self) -> usize {
    //         let (a, b) = (self.a.indexable(), self.b.indexable());
    //         a.saturating_add(b)
    //     }
    //
    //     #[inline]
    //     fn idx(&mut self, index: usize) -> Option<A::Item> {
    //         let len = self.a.indexable();
    //         if index < len {
    //             self.a.idx(index)
    //         } else {
    //             self.b.idx(index - len)
    //         }
    //     }
    // }

    #[test]
    fn idx_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let other: A<T> = A { begin: 10, end: 20 };

	type U = A<T>;
	let mut chain: Chain<A<T>, U> = a.chain::<U>(other);

	for i in 0..20 {
	    let x: Option<T> = chain.idx(i);
	    match x {
		Some(v) => assert_eq!(v, i as T),
		_ => assert!(false)
	    }
	}

	assert_eq!(chain.idx(20), None::<T>);
    }
}
