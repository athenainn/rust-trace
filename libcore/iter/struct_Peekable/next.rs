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
    fn next_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let mut peekable: Peekable<A<T>> = a.peekable();

	for x in 0..10 {
	    {
		let y: Option<&T> = peekable.peek();
		match y {
		    Some(v) => { assert_eq!(*v, x); }
		    None => { assert!(false); }
		}
	    }

	    let z: Option<T> = peekable.next();
	    match z {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(peekable.next(), None::<T>);
    }
}
