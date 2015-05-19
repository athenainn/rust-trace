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

    #[test]
    fn next_test1() {
	let mut a: A<T> = A { begin: 0, end: 10 };

	for x in 0..10 {
	    let y: Option<T> = a.next();
	    match y {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(a.next(), None::<T>);
    }
}
