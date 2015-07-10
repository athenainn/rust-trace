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

    // impl<I: Iterator> Peekable<I> {
    //     /// Returns a reference to the next element of the iterator with out
    //     /// advancing it, or None if the iterator is exhausted.
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn peek(&mut self) -> Option<&I::Item> {
    //         if self.peeked.is_none() {
    //             self.peeked = self.iter.next();
    //         }
    //         match self.peeked {
    //             Some(ref value) => Some(value),
    //             None => None,
    //         }
    //     }
    //
    //     /// Checks whether peekable iterator is empty or not.
    //     #[inline]
    //     pub fn is_empty(&mut self) -> bool {
    //         self.peek().is_none()
    //     }
    // }

    #[test]
    fn peek_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let mut peekable: Peekable<A<T>> = a.peekable();

	assert_eq!(peekable.peek(), Some::<&T>(&0));
    }

    #[test]
    fn peek_test2() {
	let a: A<T> = A { begin: 0, end: 10 };
	let mut peekable: Peekable<A<T>> = a.peekable();

	peekable.next();

	assert_eq!(peekable.peek(), Some::<&T>(&1));
    }
}
