#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::DoubleEndedIterator;
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

    type T = i32;
    Iterator_impl!(T);

    // impl<I: DoubleEndedIterator, P> DoubleEndedIterator for Filter<I, P>
    //     where P: FnMut(&I::Item) -> bool,
    // {
    //     #[inline]
    //     fn next_back(&mut self) -> Option<I::Item> {
    //         for x in self.iter.by_ref().rev() {
    //             if (self.predicate)(&x) {
    //                 return Some(x);
    //             }
    //         }
    //         None
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
    fn next_back_test1() {
	let a: A<T> = A { begin: -10, end: 10 };
	let predicate: P = P;
	let mut filter: Filter<A<T>, P> = a.filter::<P>(predicate);

	for x in 0..10 {
	    let y: Option<Item> = filter.next_back();
	    match y {
		Some(v) => { assert_eq!(v, 9 - x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(filter.next(), None::<Item>);
    }
}
