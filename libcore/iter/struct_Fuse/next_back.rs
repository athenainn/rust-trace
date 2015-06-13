#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::DoubleEndedIterator;
    use core::iter::Fuse;

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

		// fn fuse(self) -> Fuse<Self> where Self: Sized {
		//     Fuse{iter: self, done: false}
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

    // impl<I> DoubleEndedIterator for Fuse<I> where I: DoubleEndedIterator {
    //     #[inline]
    //     fn next_back(&mut self) -> Option<<I as Iterator>::Item> {
    //         if self.done {
    //             None
    //         } else {
    //             let next = self.iter.next_back();
    //             self.done = next.is_none();
    //             next
    //         }
    //     }
    // }

    #[test]
    fn next_back_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let mut fuse: Fuse<A<T>> = a.fuse();

	for x in 0..10 {
	    let y: Option<T> = fuse.next_back();
	    match y {
		Some(v) => { assert_eq!(v, 9 - x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(fuse.next(), None::<T>);
    }
}
