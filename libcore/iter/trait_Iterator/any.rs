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

		// fn any<F>(&mut self, mut f: F) -> bool where
		//     Self: Sized,
		//     F: FnMut(Self::Item) -> bool
    		// {
		//     for x in self.by_ref() {
		//         if f(x) {
		//             return true;
		//         }
		//     }
		//     false
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
	    item == 3
	}
    }

    impl FnMut<Args> for F {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    item == 3
	}
    }

    #[test]
    fn any_test1() {
	let mut a: A<T> = A { begin: 1, end: 6 };
	let f: F = F;
	let any: bool = a.any::<F>(f);

	assert_eq!(any, true);
    }

    #[test]
    fn any_test2() {
	let mut a: A<T> = A { begin: 5, end: 10 };
	let f: F = F;
	let any: bool = a.any::<F>(f);

	assert_eq!(any, false);
    }
}
