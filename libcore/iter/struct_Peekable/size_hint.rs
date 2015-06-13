#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Peekable;

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

		// fn peekable(self) -> Peekable<Self> where Self: Sized {
		//     Peekable{iter: self, peeked: None}
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<I: Iterator> Iterator for Peekable<I> {
    //     type Item = I::Item;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<I::Item> {
    //         match self.peeked {
    //             Some(_) => self.peeked.take(),
    //             None => self.iter.next(),
    //         }
    //     }
    //
    //     #[inline]
    //     fn count(self) -> usize {
    //         (if self.peeked.is_some() { 1 } else { 0 }) + self.iter.count()
    //     }
    //
    //     #[inline]
    //     fn nth(&mut self, n: usize) -> Option<I::Item> {
    //         match self.peeked {
    //             Some(_) if n == 0 => self.peeked.take(),
    //             Some(_) => {
    //                 self.peeked = None;
    //                 self.iter.nth(n-1)
    //             },
    //             None => self.iter.nth(n)
    //         }
    //     }
    //
    //     #[inline]
    //     fn last(self) -> Option<I::Item> {
    //         self.iter.last().or(self.peeked)
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         let (lo, hi) = self.iter.size_hint();
    //         if self.peeked.is_some() {
    //             let lo = lo.saturating_add(1);
    //             let hi = hi.and_then(|x| x.checked_add(1));
    //             (lo, hi)
    //         } else {
    //             (lo, hi)
    //         }
    //     }
    // }

    #[test]
    fn size_hint_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let peekable: Peekable<A<T>> = a.peekable();
	let (lower, upper): (usize, Option<usize>) = peekable.size_hint();

	assert_eq!(lower, 10);
	assert_eq!(upper, Some::<usize>(10));
    }

    #[test]
    fn size_hint_test2() {
	let a: A<T> = A { begin: 0, end: 10 };
	let mut peekable: Peekable<A<T>> = a.peekable();

	peekable.next();
	let (lower, upper): (usize, Option<usize>) = peekable.size_hint();

	assert_eq!(lower, 9);
	assert_eq!(upper, Some::<usize>(9));
    }
}
