#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::DoubleEndedIterator;
    use core::iter::Rev;

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

		// fn rev(self) -> Rev<Self> where Self: Sized + DoubleEndedIterator {
		//     Rev{iter: self}
		// }
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

    // impl<I> DoubleEndedIterator for Rev<I> where I: DoubleEndedIterator {
    //     #[inline]
    //     fn next_back(&mut self) -> Option<<I as Iterator>::Item> { self.iter.next() }
    // }

    #[test]
    fn next_back_test1() {
	let a: A<T> = A { begin: 1, end: 6 };
	let mut rev: Rev<A<T>> = a.rev();

	for x in 1..6 {
	    let y: Option<T> = rev.next_back();
	    match y {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(rev.next(), None::<T>);
    }
}
