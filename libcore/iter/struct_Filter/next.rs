#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Filter;

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

		// fn filter<P>(self, predicate: P) -> Filter<Self, P> where
		//     Self: Sized, P: FnMut(&Self::Item) -> bool,
		// {
		//     Filter{iter: self, predicate: predicate}
		// }

		// fn by_ref(&mut self) -> &mut Self where Self: Sized { self }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<I: Iterator> IntoIterator for I {
    //     type Item = I::Item;
    //     type IntoIter = I;
    //
    //     fn into_iter(self) -> I {
    //         self
    //     }
    // }

    // impl<I: Iterator, P> Iterator for Filter<I, P> where P: FnMut(&I::Item) -> bool {
    //     type Item = I::Item;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<I::Item> {
    //         for x in self.iter.by_ref() {
    //             if (self.predicate)(&x) {
    //                 return Some(x);
    //             }
    //         }
    //         None
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         let (_, upper) = self.iter.size_hint();
    //         (0, upper) // can't know a lower bound, due to the predicate
    //     }
    // }

    struct P;

    type Item = T;
    type Args<'a> = (&'a Item,);

    impl<'a> FnOnce<Args<'a>> for P {
	type Output = bool;
	extern "rust-call" fn call_once(self, (item,): Args) -> Self::Output {
	    match item {
		_ if *item >= 0 => true,
		_ => false
	    }
	}
    }

    impl<'a> FnMut<Args<'a>> for P {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    match item {
		_ if *item >= 0 => true,
		_ => false
	    }
	}
    }

    #[test]
    fn next_test1() {
	let a: A<T> = A { begin: -10, end: 10 };
	let predicate: P = P;
	let mut filter: Filter<A<T>, P> = a.filter::<P>(predicate);

	for x in 0..10 {
	    let y: Option<Item> = filter.next();
	    match y {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(filter.next(), None::<Item>);
    }
}
