#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Scan;

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

		// fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>
		//     where Self: Sized, F: FnMut(&mut St, Self::Item) -> Option<B>,
		// {
		//     Scan{iter: self, f: f, state: initial_state}
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<B, I, St, F> Iterator for Scan<I, St, F> where
    //     I: Iterator,
    //     F: FnMut(&mut St, I::Item) -> Option<B>,
    // {
    //     type Item = B;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<B> {
    //         self.iter.next().and_then(|a| (self.f)(&mut self.state, a))
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         let (_, upper) = self.iter.size_hint();
    //         (0, upper) // can't know a lower bound, due to the scan function
    //     }
    // }

    struct F;

    type B = T;
    type St = T;
    type Item = T;
    type Args<'a> = (&'a mut St, Item);

    impl<'a> FnOnce<Args<'a>> for F {
	type Output = Option<B>;
	extern "rust-call" fn call_once(self, (st, item): Args) -> Self::Output {
	    *st += item;
	    Some::<B>(*st)
	}
    }

    impl<'a> FnMut<Args<'a>> for F {
	extern "rust-call" fn call_mut(&mut self, (st, item): Args) -> Self::Output {
	    *st += item;
	    Some::<B>(*st)
	}
    }

    #[test]
    fn size_hint_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let st: St = 0;
	let f: F = F;
	let scan: Scan<A<T>, B, F> = a.scan::<St, B, F>(st, f);

	let (lower, upper): (usize, Option<usize>) = scan.size_hint();

	assert_eq!(lower, 0);
	assert_eq!(upper, Some::<usize>(10));
    }
}
