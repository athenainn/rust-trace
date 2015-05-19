#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Enumerate;

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

		// fn enumerate(self) -> Enumerate<Self> where Self: Sized {
		//     Enumerate { iter: self, count: 0 }
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    #[test]
    fn enumerate_test1() {
	let a: A<T> = A { begin: 10, end: 20 };
	let mut enumerate: Enumerate<A<T>> = a.enumerate();

	for x in 0..10 {
	    let y: Option<(usize, T)> = enumerate.next();
	    match y {
		Some(v) => { assert_eq!(v, (x as usize, 10 + x)); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(enumerate.next(), None::<(usize, T)>);
    }
}
