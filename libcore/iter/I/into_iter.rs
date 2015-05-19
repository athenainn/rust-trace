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
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<I: Iterator> IntoIterator for I {
    //     type Item = I::Item;
    //     type IntoIter = I;
    //
    //     fn into_iter(self) -> I {
    //         self
    //     }
    // }

    type I = A<T>;

    #[test]
    fn into_iter_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let mut n: T = 0;

	for x in a {
	    assert_eq!(x, n);
	    n += 1;
	}

	assert_eq!(n, 10);
    }

    #[test]
    fn into_iter_test2() {
	let a: A<T> = A { begin: 0, end: 10 };
	let iter: I = a.into_iter();
	let mut n: T = 0;

	for x in iter {
	    assert_eq!(x, n);
	    n += 1;
	}

	assert_eq!(n, 10);
    }
}
