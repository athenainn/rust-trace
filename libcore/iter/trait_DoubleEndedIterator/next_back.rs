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

    #[test]
    fn next_back_test1() {
	let mut a: A<T> = A { begin: 0, end: 10 };

	for x in 0..10 {
	    let y: Option<<A<T> as Iterator>::Item> = a.next_back();
	    match y {
		Some(v) => { assert_eq!(v, 9 - x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(a.next_back(), None::<<A<T> as Iterator>::Item>);
    }
}
