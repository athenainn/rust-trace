#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::SkipWhile;

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

		// fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P> where
		//     Self: Sized, P: FnMut(&Self::Item) -> bool,
		// {
		//     SkipWhile{iter: self, flag: false, predicate: predicate}
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<I: Iterator, P> Iterator for SkipWhile<I, P>
    //     where P: FnMut(&I::Item) -> bool
    // {
    //     type Item = I::Item;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<I::Item> {
    //         for x in self.iter.by_ref() {
    //             if self.flag || !(self.predicate)(&x) {
    //                 self.flag = true;
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
		_ if *item < 0 => true,
		_ => false
	    }
	}
    }

    impl<'a> FnMut<Args<'a>> for P {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    match item {
		_ if *item < 0 => true,
		_ => false
	    }
	}
    }

    #[test]
    fn size_hint_test1() {
	let a: A<T> = A { begin: -10, end: 10 };
	let predicate: P = P;
	let skip_while: SkipWhile<A<T>, P> = a.skip_while::<P>(predicate);

	let (lower, upper): (usize, Option<usize>) = skip_while.size_hint();

	assert_eq!(lower, 0);
	assert_eq!(upper, Some::<usize>(20));
    }
}
