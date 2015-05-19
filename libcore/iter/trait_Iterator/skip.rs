#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Skip;

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

		// fn skip(self, n: usize) -> Skip<Self> where Self: Sized {
		//     Skip{iter: self, n: n}
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    #[test]
    fn skip_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let n: usize = 3;
	let mut skip: Skip<A<T>> = a.skip(n);

	for i in (n as T)..10 {
	    let x: Option<T> = skip.next();
	    match x {
		Some(v) => { assert_eq!(v, i); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(skip.next(), None::<T>);
    }
}
