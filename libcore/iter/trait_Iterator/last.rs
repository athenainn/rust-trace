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

		// fn last(self) -> Option<Self::Item> where Self: Sized {
		//     let mut last = None;
		//     for x in self { last = Some(x); }
		//     last
    		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    #[test]
    fn last_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let last: Option<T> = a.last();

	assert_eq!(last, Some::<T>(9));
    }
}
