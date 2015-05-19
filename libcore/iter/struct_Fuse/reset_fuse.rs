#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Fuse;

    struct A<T> {
	begin: T,
	end: T,
	def_begin: T,
	def_end: T
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
			let result = None::<Self::Item>;
			self.begin = self.def_begin;
			self.end = self.def_end;
			result
		    }
		}

		// fn fuse(self) -> Fuse<Self> where Self: Sized {
		//     Fuse{iter: self, done: false}
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<I> Fuse<I> {
    //     /// Resets the `Fuse` such that the next call to `.next()` or
    //     /// `.next_back()` will call the underlying iterator again even if it
    //     /// previously returned `None`.
    //     #[inline]
    //     #[unstable(feature = "core", reason = "seems marginal")]
    //     pub fn reset_fuse(&mut self) {
    //         self.done = false
    //     }
    // }

    #[test]
    fn reset_fuse_test1() {
	let mut a: A<T> = A { begin: 0, end: 10, def_begin: 100, def_end: 110 };
	let mut fuse: Fuse<A<T>> = a.fuse();

	for x in 0..10 {
	    let y: Option<T> = fuse.next();
	    match y {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(fuse.next(), None::<T>);

	fuse.reset_fuse();

	for x in 0..10 {
	    let y: Option<T> = fuse.next();
	    match y {
		Some(v) => { assert_eq!(v, 100 + x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(fuse.next(), None::<T>);
    }
}
