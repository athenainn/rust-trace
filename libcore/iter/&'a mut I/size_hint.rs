#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::ExactSizeIterator;

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
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<'a, I: Iterator + ?Sized> Iterator for &'a mut I {
    //     type Item = I::Item;
    //     fn next(&mut self) -> Option<I::Item> { (**self).next() }
    //     fn size_hint(&self) -> (usize, Option<usize>) { (**self).size_hint() }
    // }

    type I = A<T>;

    #[test]
    fn size_hint_test1() {
	let mut a: A<T> = A { begin: 0, end: 10 };
	let iter: &mut I = &mut a;
	let (lower, upper): (usize, Option<usize>) = iter.size_hint();

	assert_eq!(lower, 10);
	assert_eq!(upper, Some::<usize>(10));
    }
}
