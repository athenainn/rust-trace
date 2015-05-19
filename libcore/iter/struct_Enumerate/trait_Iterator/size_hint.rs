#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Enumerate;

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

		// fn enumerate(self) -> Enumerate<Self> where Self: Sized {
		//     Enumerate { iter: self, count: 0 }
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<I> Iterator for Enumerate<I> where I: Iterator {
    //     type Item = (usize, <I as Iterator>::Item);
    //
    //     /// # Overflow Behavior
    //     ///
    //     /// The method does no guarding against overflows, so enumerating more than
    //     /// `usize::MAX` elements either produces the wrong result or panics. If
    //     /// debug assertions are enabled, a panic is guaranteed.
    //     ///
    //     /// # Panics
    //     ///
    //     /// Might panic if the index of the element overflows a `usize`.
    //     #[inline]
    //     fn next(&mut self) -> Option<(usize, <I as Iterator>::Item)> {
    //         self.iter.next().map(|a| {
    //             let ret = (self.count, a);
    //             // Possible undefined overflow.
    //             self.count += 1;
    //             ret
    //         })
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         self.iter.size_hint()
    //     }
    //
    //     #[inline]
    //     fn nth(&mut self, n: usize) -> Option<(usize, I::Item)> {
    //         self.iter.nth(n).map(|a| {
    //             let i = self.count + n;
    //             self.count = i + 1;
    //             (i, a)
    //         })
    //     }
    //
    //     #[inline]
    //     fn count(self) -> usize {
    //         self.iter.count()
    //     }
    // }

    #[test]
    fn size_hint_test1() {
	let a: A<T> = A { begin: 10, end: 20 };
	let enumerate: Enumerate<A<T>> = a.enumerate();

	let (lower, upper): (usize, Option<usize>) = enumerate.size_hint();

	assert_eq!(lower, 10);
	assert_eq!(upper, Some::<usize>(10));
    }

    #[test]
    fn size_hint_test2() {
	let a: A<T> = A { begin: 10, end: 20 };
	let mut enumerate: Enumerate<A<T>> = a.enumerate();

	enumerate.next();
	let (lower, upper): (usize, Option<usize>) = enumerate.size_hint();

	assert_eq!(lower, 9);
	assert_eq!(upper, Some::<usize>(9));
    }
}
