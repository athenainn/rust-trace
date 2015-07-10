#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Zip;

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

		// fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter> where
		//     Self: Sized, U: IntoIterator
		// {
		//     Zip{a: self, b: other.into_iter()}
    		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    #[test]
    fn zip_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let other: A<T> = A { begin: 10, end: 15 };

	type U = A<T>;
	let mut zip: Zip<A<T>, U> = a.zip::<U>(other);

	assert_eq!(zip.next(), Some::<(T, T)>((0, 10)));
	assert_eq!(zip.next(), Some::<(T, T)>((1, 11)));
	assert_eq!(zip.next(), Some::<(T, T)>((2, 12)));
	assert_eq!(zip.next(), Some::<(T, T)>((3, 13)));
	assert_eq!(zip.next(), Some::<(T, T)>((4, 14)));
	assert_eq!(zip.next(), None::<(T, T)>);
    }
}
