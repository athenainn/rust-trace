#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Inspect;

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

		// fn inspect<F>(self, f: F) -> Inspect<Self, F> where
		//     Self: Sized, F: FnMut(&Self::Item),
		// {
		//     Inspect{iter: self, f: f}
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<I: Iterator, F> Inspect<I, F> where F: FnMut(&I::Item) {
    //     #[inline]
    //     fn do_inspect(&mut self, elt: Option<I::Item>) -> Option<I::Item> {
    //         if let Some(ref a) = elt {
    //             (self.f)(a);
    //         }
    //
    //         elt
    //     }
    // }

    // impl<I: Iterator, F> Iterator for Inspect<I, F> where F: FnMut(&I::Item) {
    //     type Item = I::Item;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<I::Item> {
    //         let next = self.iter.next();
    //         self.do_inspect(next)
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         self.iter.size_hint()
    // }

    struct F;

    type B = T;
    type Item = T;
    type Args<'a> = (&'a Item,);

    impl<'a> FnOnce<Args<'a>> for F {
	type Output = ();
	extern "rust-call" fn call_once(self, (item,): Args) -> Self::Output {
	    assert!(*item >= 0);
	    ()
	}
    }

    impl<'a> FnMut<Args<'a>> for F {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    assert!(*item >= 0);
	    ()
	}
    }

    #[test]
    fn next_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let f: F = F;
	let mut inspect: Inspect<A<T>, F> = a.inspect::<F>(f);

	for x in 0..10 {
	    let y: Option<Item> = inspect.next();
	    match y {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(inspect.next(), None::<Item>);
    }
}
