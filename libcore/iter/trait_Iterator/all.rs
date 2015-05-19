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

		// fn all<F>(&mut self, mut f: F) -> bool where
		//     Self: Sized, F: FnMut(Self::Item) -> bool
		// {
		//     for x in self.by_ref() {
		//         if !f(x) {
		//             return false;
		//         }
		//     }
		//     true
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    struct F;

    type Item = T;
    type Args = (Item,);

    impl FnOnce<Args> for F {
	type Output = bool;
	extern "rust-call" fn call_once(self, (item,): Args) -> Self::Output {
	    item > 0
	}
    }

    impl FnMut<Args> for F {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    item > 0
	}
    }

    #[test]
    fn all_test1() {
	let mut a: A<T> = A { begin: 0, end: 10 };
	let f: F = F;
	let all: bool = a.all::<F>(f);

	assert_eq!(all, false);
    }

    #[test]
    fn all_test2() {
	let mut a: A<T> = A { begin: 1, end: 10 };
	let f: F = F;
	let all: bool = a.all::<F>(f);

	assert_eq!(all, true);
    }
}
