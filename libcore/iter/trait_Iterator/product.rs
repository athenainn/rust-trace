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

		// fn product<P=<Self as Iterator>::Item>(self) -> P where
		//     P: Mul<Self::Item, Output=P> + One,
		//     Self: Sized,
		// {
		//     self.fold(One::one(), |p, e| p * e)
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    type P = T;

    #[test]
    fn product_test1() {
	let a: A<T> = A { begin: 1, end: 10 };
	let product: P = a.product::<P>();

	assert_eq!(product, 1 * 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9);
    }
}
