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

		// fn count(self) -> usize where Self: Sized {
		//     // Might overflow.
		//     self.fold(0, |cnt, _| cnt + 1)
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    #[test]
    fn count_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let count: usize = a.count();

	assert_eq!(count, 10);
    }

    #[test]
    fn count_test2() {
	let mut a: A<T> = A { begin: 0, end: 10 };
	assert_eq!(a.next(), Some::<T>(0));

	let count: usize = a.count();
	assert_eq!(count, 9);
    }
}
