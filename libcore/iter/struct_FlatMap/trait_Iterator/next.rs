#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::FlatMap;

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

		// fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>
		//     where Self: Sized, U: IntoIterator, F: FnMut(Self::Item) -> U,
		// {
		//     FlatMap{iter: self, f: f, frontiter: None, backiter: None }
		// }

		// fn by_ref(&mut self) -> &mut Self where Self: Sized { self }
	    }
	}
    }

    type T = u32;
    Iterator_impl!(T);

    // impl<I: Iterator> IntoIterator for I {
    //     type Item = I::Item;
    //     type IntoIter = I;
    //
    //     fn into_iter(self) -> I {
    //         self
    //     }
    // }

    // impl<I: Iterator, U: IntoIterator, F> Iterator for FlatMap<I, U, F>
    //     where F: FnMut(I::Item) -> U,
    // {
    //     type Item = U::Item;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<U::Item> {
    //         loop {
    //             if let Some(ref mut inner) = self.frontiter {
    //                 if let Some(x) = inner.by_ref().next() {
    //                     return Some(x)
    //                 }
    //             }
    //             match self.iter.next().map(|x| (self.f)(x)) {
    //                 None => return self.backiter.as_mut().and_then(|it| it.next()),
    //                 next => self.frontiter = next.map(IntoIterator::into_iter),
    //             }
    //         }
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         let (flo, fhi) = self.frontiter.as_ref().map_or((0, Some(0)), |it| it.size_hint());
    //         let (blo, bhi) = self.backiter.as_ref().map_or((0, Some(0)), |it| it.size_hint());
    //         let lo = flo.saturating_add(blo);
    //         match (self.iter.size_hint(), fhi, bhi) {
    //             ((0, Some(0)), Some(a), Some(b)) => (lo, a.checked_add(b)),
    //             _ => (lo, None)
    //         }
    //     }
    // }

    struct F;

    type Item = T;
    type U = A<T>;
    type Args = (Item,);

    impl FnOnce<Args> for F {
	type Output = U;
	extern "rust-call" fn call_once(self, (item,): Args) -> Self::Output {
	    A { begin: 0, end: item }
	}
    }

    impl FnMut<Args> for F {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    A { begin: 0, end: item }
	}
    }

    #[test]
    fn next_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let f: F = F;
	let mut flat_map: FlatMap<A<T>, U, F> = a.flat_map::<U, F>(f);

	for n in 0 .. 10 {
	    for i in 0..n {
		let x: Option<<U as IntoIterator>::Item> = flat_map.next();
		match x {
		    Some(v) => { assert_eq!(v, i); }
		    None => { assert!(false); }
		}
	    }
	}

	assert_eq!(flat_map.next(), None::<Item>);
    }
}
