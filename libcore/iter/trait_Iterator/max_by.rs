#![feature(core, unboxed_closures)]
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

		// fn max_by<B: Ord, F>(self, f: F) -> Option<Self::Item> where
		//     Self: Sized,
		//     F: FnMut(&Self::Item) -> B,
		// {
		//     select_fold1(self,
		//                  f,
		//                  // switch to y even if it is only equal, to preserve
		//                  // stability.
		//                  |x_p, _, y_p, _| x_p <= y_p)
		//         .map(|(_, x)| x)
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    struct F;

    type B = T;
    type Item = T;
    type Args<'a> = (&'a Item,);

    impl<'a> FnOnce<Args<'a>> for F {
	type Output = B;
	extern "rust-call" fn call_once(self, (&item,): Args) -> Self::Output {
	    item.abs()
	}
    }

    impl<'a> FnMut<Args<'a>> for F {
	extern "rust-call" fn call_mut(&mut self, (&item,): Args) -> Self::Output {
	    item.abs()
	}
    }

    #[test]
    fn max_by_test1() {
	let a: A<T> = A { begin: 1, end: 6 };
	let f: F = F;
	let max_by: Option<Item> = a.max_by::<B, F>(f);

	assert_eq!(max_by, Some::<Item>(5));
    }

    #[test]
    fn max_by_test2() {
	let a: A<T> = A { begin: -6, end: 6 };
	let f: F = F;
	let max_by: Option<Item> = a.max_by::<B, F>(f);

	assert_eq!(max_by, Some::<Item>(-6));
    }

    #[test]
    fn max_by_test3() {
	let a: A<T> = A { begin: 1, end: 1 };
	let f: F = F;
	let max_by: Option<Item> = a.max_by::<B, F>(f);

	assert_eq!(max_by, None::<Item>);
    }
}
