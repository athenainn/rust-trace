#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Take;

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

		// fn take(self, n: usize) -> Take<Self> where Self: Sized, {
		//     Take{iter: self, n: n}
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    #[test]
    fn take_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let n: usize = 5;
	let mut take: Take<A<T>> = a.take(n);

	for i in 0..(n as T) {
	    let x: Option<T> = take.next();
	    match x {
		Some(v) => { assert_eq!(v, i); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(take.next(), None::<T>);
    }
}
