#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::FlatMap;

    use core::iter::Take;
    use core::ops::RangeFrom;

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

		// fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>
		//     where Self: Sized, U: IntoIterator, F: FnMut(Self::Item) -> U,
		// {
		//     FlatMap{iter: self, f: f, frontiter: None, backiter: None }
		// }
	    }
	}
    }

    type T = u32;
    Iterator_impl!(T);

    struct F;

    type Item = T;
    type U = Take<RangeFrom<Item>>;
    type Args = (Item,);

    impl FnOnce<Args> for F {
	type Output = U;
	extern "rust-call" fn call_once(self, (item,): Args) -> Self::Output {
	    (0..).take(item as usize)
	}
    }

    impl FnMut<Args> for F {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    (0..).take(item as usize)
	}
    }

    #[test]
    fn flat_map_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let f: F = F;
	let mut flat_map: FlatMap<A<T>, U, F> = a.flat_map::<U, F>(f);

	for n in 0 .. 10 {
	    for i in 0..n {
		let x: Option<<U as IntoIterator>::Item> = flat_map.next();
		match x {
		    Some(v) => { assert_eq!(v, i); }
		    None => { assert!(false); }
		}
	    }
	}

	assert_eq!(flat_map.next(), None::<Item>);
    }
}
