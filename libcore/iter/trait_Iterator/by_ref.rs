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

		// fn by_ref(&mut self) -> &mut Self where Self: Sized { self }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    #[test]
    fn by_ref_test1() {
	let mut a: A<T> = A { begin: 0, end: 10 };
	let by_ref: &mut A<T> = a.by_ref();

	for x in 0..10 {
	    let y: Option<<A<T> as Iterator>::Item> = by_ref.next();
	    match y {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(by_ref.next(), None::<T>);
    }
}
