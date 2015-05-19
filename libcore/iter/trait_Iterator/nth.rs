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

		// fn nth(&mut self, mut n: usize) -> Option<Self::Item> where Self: Sized {
		//     for x in self.by_ref() {
		//         if n == 0 { return Some(x) }
		//         n -= 1;
		//     }
		//     None
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    #[test]
    fn nth_test1() {
	let mut a: A<T> = A { begin: 0, end: 10 };

	for n in 0..10 {
	    let x: Option<T> = a.nth(0 as usize);
	    match x {
		Some(v) => { assert_eq!(v, n); }
		None => { assert!(false); }
	    }
	}

	for _ in 1..10 {
	    let x: Option<T> = a.nth(0 as usize);
	    match x {
		Some(_) => { assert!(false); }
		None => { assert!(true); }
	    }
	}

	assert_eq!(a.count(), 0);
    }

    #[test]
    fn nth_test2() {
	let mut a: A<T> = A { begin: 0, end: 10 };

	let x: Option<T> = a.nth(1 as usize);
	assert_eq!(x, Some::<T>(1));

	let y: Option<T> = a.nth(2 as usize);
	assert_eq!(y, Some::<T>(4));

	let z: Option<T> = a.nth(3 as usize);
	assert_eq!(z, Some::<T>(8));
    }
}
