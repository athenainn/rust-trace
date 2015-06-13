#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Take;

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

		// fn take(self, n: usize) -> Take<Self> where Self: Sized, {
		//     Take{iter: self, n: n}
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

    // impl<I> Iterator for Take<I> where I: Iterator{
    //     type Item = <I as Iterator>::Item;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<<I as Iterator>::Item> {
    //         if self.n != 0 {
    //             self.n -= 1;
    //             self.iter.next()
    //         } else {
    //             None
    //         }
    //     }
    //
    //     #[inline]
    //     fn nth(&mut self, n: usize) -> Option<I::Item> {
    //         if self.n > n {
    //             self.n -= n + 1;
    //             self.iter.nth(n)
    //         } else {
    //             if self.n > 0 {
    //                 self.iter.nth(self.n - 1);
    //                 self.n = 0;
    //             }
    //             None
    //         }
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         let (lower, upper) = self.iter.size_hint();
    //
    //         let lower = cmp::min(lower, self.n);
    //
    //         let upper = match upper {
    //             Some(x) if x < self.n => Some(x),
    //             _ => Some(self.n)
    //         };
    //
    //         (lower, upper)
    //     }
    // }

    #[test]
    fn nth_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let n: usize = 5;
	let mut take: Take<A<T>> = a.take(n);

	for x in 0..n {
	    let y: Option<T> = take.nth(0);
	    match y {
		Some(v) => { assert_eq!(v, x as T); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(take.nth(0), None::<T>);
    }
}
