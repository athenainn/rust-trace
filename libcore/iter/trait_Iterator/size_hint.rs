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

		// fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    #[test]
    fn size_hint_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let x: (usize, Option<usize>) = a.size_hint();

	assert_eq!(x.0, 0);
	assert_eq!(x.1, None::<usize>);
    }
}
