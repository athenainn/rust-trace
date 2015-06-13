#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::DoubleEndedIterator;
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

    // impl<I: DoubleEndedIterator, U, F> DoubleEndedIterator for FlatMap<I, U, F> where
    //     F: FnMut(I::Item) -> U,
    //     U: IntoIterator,
    //     U::IntoIter: DoubleEndedIterator
    // {
    //     #[inline]
    //     fn next_back(&mut self) -> Option<U::Item> {
    //         loop {
    //             if let Some(ref mut inner) = self.backiter {
    //                 if let Some(y) = inner.next_back() {
    //                     return Some(y)
    //                 }
    //             }
    //             match self.iter.next_back().map(|x| (self.f)(x)) {
    //                 None => return self.frontiter.as_mut().and_then(|it| it.next_back()),
    //                 next => self.backiter = next.map(IntoIterator::into_iter),
    //             }
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
    fn next_back_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let f: F = F;
	let mut flat_map: FlatMap<A<T>, U, F> = a.flat_map::<U, F>(f);

	for n in 0 .. 10 {
	    for i in 0..(10 - n - 1) {
		let x: Option<<U as IntoIterator>::Item> = flat_map.next_back();
		match x {
		    Some(v) => { assert_eq!(v, (10 - n - 1) - i - 1); }
		    None => { assert!(false); }
		}
	    }
	}

	assert_eq!(flat_map.next_back(), None::<Item>);
    }
}
