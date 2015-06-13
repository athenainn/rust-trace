#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Skip;

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

		// fn skip(self, n: usize) -> Skip<Self> where Self: Sized {
		//     Skip{iter: self, n: n}
		// }

		// fn nth(&mut self, mut n: usize) -> Option<Self::Item> where Self: Sized {
		//     for x in self.by_ref() {
		//         if n == 0 { return Some(x) }
		//         n -= 1;
		//     }
		//     None
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<I> Iterator for Skip<I> where I: Iterator {
    //     type Item = <I as Iterator>::Item;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<I::Item> {
    //         if self.n == 0 {
    //             self.iter.next()
    //         } else {
    //             let old_n = self.n;
    //             self.n = 0;
    //             self.iter.nth(old_n)
    //         }
    //     }
    //
    //     #[inline]
    //     fn nth(&mut self, n: usize) -> Option<I::Item> {
    //         // Can't just add n + self.n due to overflow.
    //         if self.n == 0 {
    //             self.iter.nth(n)
    //         } else {
    //             let to_skip = self.n;
    //             self.n = 0;
    //             // nth(n) skips n+1
    //             if self.iter.nth(to_skip-1).is_none() {
    //                 return None;
    //             }
    //             self.iter.nth(n)
    //         }
    //     }
    //
    //     #[inline]
    //     fn count(self) -> usize {
    //         self.iter.count().saturating_sub(self.n)
    //     }
    //
    //     #[inline]
    //     fn last(mut self) -> Option<I::Item> {
    //         if self.n == 0 {
    //             self.iter.last()
    //         } else {
    //             let next = self.next();
    //             if next.is_some() {
    //                 // recurse. n should be 0.
    //                 self.last().or(next)
    //             } else {
    //                 None
    //             }
    //         }
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         let (lower, upper) = self.iter.size_hint();
    // 
    //         let lower = lower.saturating_sub(self.n);
    //         let upper = upper.map(|x| x.saturating_sub(self.n));
    // 
    //         (lower, upper)
    //     }
    // }

    #[test]
    fn nth_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let n: usize = 3;
	let mut skip: Skip<A<T>> = a.skip(n);

	for x in 0..7 {
	    let y: Option<T> = skip.nth(0);
	    match y {
		Some(v) => { assert_eq!(v, (n + x) as T); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(skip.nth(0), None::<T>);
    }
}
