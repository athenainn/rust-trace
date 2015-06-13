#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Zip;

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


		// fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter> where
		//     Self: Sized, U: IntoIterator
		// {
		//     Zip{a: self, b: other.into_iter()}
    		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<A, B> Iterator for Zip<A, B> where A: Iterator, B: Iterator
    // {
    //     type Item = (A::Item, B::Item);
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<(A::Item, B::Item)> {
    //         self.a.next().and_then(|x| {
    //             self.b.next().and_then(|y| {
    //                 Some((x, y))
    //             })
    //         })
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         let (a_lower, a_upper) = self.a.size_hint();
    //         let (b_lower, b_upper) = self.b.size_hint();
    //
    //         let lower = cmp::min(a_lower, b_lower);
    //
    //         let upper = match (a_upper, b_upper) {
    //             (Some(x), Some(y)) => Some(cmp::min(x,y)),
    //             (Some(x), None) => Some(x),
    //             (None, Some(y)) => Some(y),
    //             (None, None) => None
    //         };
    //
    //         (lower, upper)
    //     }
    // }

    #[test]
    fn size_hint_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let other: A<T> = A { begin: 10, end: 15 };

	type U = A<T>;
	let zip: Zip<A<T>, U> = a.zip::<U>(other);
	let (lower, upper): (usize, Option<usize>) = zip.size_hint();

	assert_eq!(lower, 5);
	assert_eq!(upper, Some::<usize>(5));
    }
}
