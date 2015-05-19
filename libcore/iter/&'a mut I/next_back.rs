#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
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
	    }
	}
    }

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

    // impl<'a, I: DoubleEndedIterator + ?Sized> DoubleEndedIterator for &'a mut I {
    //     fn next_back(&mut self) -> Option<I::Item> { (**self).next_back() }
    // }

    type I = A<T>;

    #[test]
    fn next_back_test1() {
	let mut a: A<T> = A { begin: 0, end: 10 };
	let mut iter: &mut I = &mut a;

	for x in 0..10 {
	    let y: Option<T> = iter.next_back();
	    match y {
		Some(v) => { assert_eq!(v, 9 - x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(iter.next(), None::<<I as Iterator>::Item>);;
    }
}
