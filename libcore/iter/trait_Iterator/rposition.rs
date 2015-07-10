#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::ExactSizeIterator;
    use core::iter::DoubleEndedIterator;

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

		// fn rposition<P>(&mut self, mut predicate: P) -> Option<usize> where
		//     P: FnMut(Self::Item) -> bool,
		//     Self: Sized + ExactSizeIterator + DoubleEndedIterator
		// {
		//     let mut i = self.len();
		//
		//     while let Some(v) = self.next_back() {
		//         if predicate(v) {
		//             return Some(i - 1);
		//         }
		//         // No need for an overflow check here, because `ExactSizeIterator`
		//         // implies that the number of elements fits into a `usize`.
		//         i -= 1;
		//     }
		//     None
		// }
	    }
	}
    }

    impl ExactSizeIterator for A<T> {}
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

    struct P;

    type Item = T;
    type Args = (Item,);

    impl FnOnce<Args> for P {
	type Output = bool;
	extern "rust-call" fn call_once(self, (item,): Args) -> Self::Output {
	    match item {
		2 => true,
		_ => false
	    }
	}
    }

    impl FnMut<Args> for P {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    match item {
		2 => true,
		_ => false
	    }
	}
    }

    #[test]
    fn rposition_test1() {
	let mut a: A<T> = A { begin: 1, end: 6 };
	let predicate: P = P;
	let rposition: Option<usize> = a.rposition::<P>(predicate);

	assert_eq!(rposition, Some::<usize>(1));
    }

    #[test]
    fn rposition_test2() {
	let mut a: A<T> = A { begin: 5, end: 10 };
	let predicate: P = P;
	let rposition: Option<usize> = a.rposition::<P>(predicate);

	assert_eq!(rposition, None::<usize>);
    }
}
