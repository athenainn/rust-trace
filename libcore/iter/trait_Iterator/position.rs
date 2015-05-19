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

		// fn position<P>(&mut self, mut predicate: P) -> Option<usize> where
		//     Self: Sized,
		//     P: FnMut(Self::Item) -> bool,
		// {
		//     // `enumerate` might overflow.
		//     for (i, x) in self.by_ref().enumerate() {
		//         if predicate(x) {
		//             return Some(i);
		//         }
		//     }
		//     None
		// }
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
		3 => true,
		_ => false
	    }
	}
    }

    impl FnMut<Args> for P {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    match item {
		3 => true,
		_ => false
	    }
	}
    }

    #[test]
    fn position_test1() {
	let mut a: A<T> = A { begin: 1, end: 6 };
	let predicate: P = P;
	let position: Option<usize> = a.position::<P>(predicate);

	assert_eq!(position, Some::<usize>(2));
    }

    #[test]
    fn position_test2() {
	let mut a: A<T> = A { begin: 5, end: 10 };
	let predicate: P = P;
	let position: Option<usize> = a.position::<P>(predicate);

	assert_eq!(position, None::<usize>);
    }
}
