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

		// fn fold<B, F>(self, init: B, mut f: F) -> B where
		//     Self: Sized, F: FnMut(B, Self::Item) -> B,
		// {
		//     let mut accum = init;
		//     for x in self {
		//         accum = f(accum, x);
		//     }
		//     accum
		// }
	    }
	}
    }

    struct F;

    macro_rules! F_impl {
	($T:ty, $B:ty) => {
	    type Args = ($B, $T);

	    impl FnOnce<Args> for F {
		type Output = $B;
		extern "rust-call"
		fn call_once(self, (accum, item): Args) -> Self::Output {
		    accum + item
		}
	    }

	    impl FnMut<Args> for F {
		extern "rust-call"
		fn call_mut(&mut self, (accum, item): Args) -> Self::Output {
		    accum + item
		}
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    type B = T;
    F_impl!(T, B);

    #[test]
    fn fold_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let init: B = 0;
	let f: F = F;
	let fold: B = a.fold::<B, F>(init, f);

	assert_eq!(fold, 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9);
    }
}
