#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Chain;

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

		// fn chain<U>(self, other: U) -> Chain<Self, U::IntoIter> where
		//     Self: Sized, U: IntoIterator<Item=Self::Item>,
		// {
		//     Chain{a: self, b: other.into_iter(), flag: false}
		// }

		// fn by_ref(&mut self) -> &mut Self where Self: Sized { self }

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

    // impl<I: Iterator> IntoIterator for I {
    //     type Item = I::Item;
    //     type IntoIter = I;
    //
    //     fn into_iter(self) -> I {
    //         self
    //     }
    // }

    // impl<A, B> Iterator for Chain<A, B> where
    //     A: Iterator,
    //     B: Iterator<Item = A::Item>
    // {
    //     type Item = A::Item;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<A::Item> {
    //         if self.flag {
    //             self.b.next()
    //         } else {
    //             match self.a.next() {
    //                 Some(x) => return Some(x),
    //                 _ => ()
    //             }
    //             self.flag = true;
    //             self.b.next()
    //         }
    //     }
    //
    //     #[inline]
    //     fn count(self) -> usize {
    //         (if !self.flag { self.a.count() } else { 0 }) + self.b.count()
    //     }
    //
    //     #[inline]
    //     fn nth(&mut self, mut n: usize) -> Option<A::Item> {
    //         if !self.flag {
    //             for x in self.a.by_ref() {
    //                 if n == 0 {
    //                     return Some(x)
    //                 }
    //                 n -= 1;
    //             }
    //             self.flag = true;
    //         }
    //         self.b.nth(n)
    //     }
    //
    //     #[inline]
    //     fn last(self) -> Option<A::Item> {
    //         let a_last = if self.flag { None } else { self.a.last() };
    //         let b_last = self.b.last();
    //         b_last.or(a_last)
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         let (a_lower, a_upper) = self.a.size_hint();
    //         let (b_lower, b_upper) = self.b.size_hint();
    //
    //         let lower = a_lower.saturating_add(b_lower);
    //
    //         let upper = match (a_upper, b_upper) {
    //             (Some(x), Some(y)) => x.checked_add(y),
    //             _ => None
    //         };
    //
    //         (lower, upper)
    //     }
    // }

    #[test]
    fn nth_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let other: A<T> = A { begin: 10, end: 20 };

	type U = A<T>;
	let mut chain: Chain<A<T>, U> = a.chain::<U>(other);

	for x in 0..20 {
	    let y: Option<T> = chain.nth(0);
	    match y {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(chain.nth(0), None::<T>);
    }
}
