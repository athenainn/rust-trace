#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;

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

		// fn max(self) -> Option<Self::Item> where Self: Sized, Self::Item: Ord
		// {
		//     select_fold1(self,
		//                  |_| (),
		//                  // switch to y even if it is only equal, to preserve
		//                  // stability.
		//                  |_, x, _, y| *x <= *y)
		//         .map(|(_, x)| x)
		// }
		// fn min(self) -> Option<Self::Item> where Self: Sized, Self::Item: Ord
		// {
		//     select_fold1(self,
		//                  |_| (),
		//                  // only switch to y if it is strictly smaller, to
		//                  // preserve stability.
		//                  |_, x, _, y| *x > *y)
		//         .map(|(_, x)| x)
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    #[test]
    fn min_test1() {
	let a: A<T> = A { begin: 1, end: 6 };
	let min: Option<<A<T> as Iterator>::Item> = a.min();

	assert_eq!(min, Some::<<A<T> as Iterator>::Item>(1));
    }

    #[test]
    fn min_test2() {
	let a: A<T> = A { begin: 1, end: 1 };
	let min: Option<<A<T> as Iterator>::Item> = a.min();

	assert_eq!(min, None::<<A<T> as Iterator>::Item>);
    }
}
