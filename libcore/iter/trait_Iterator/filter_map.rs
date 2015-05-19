#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::FilterMap;

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

		// fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> where
		//     Self: Sized, F: FnMut(Self::Item) -> Option<B>,
		// {
		//     FilterMap { iter: self, f: f }
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    struct F;

    type B = T;
    type Item = T;
    type Args = (Item,);

    impl FnOnce<Args> for F {
	type Output = Option<B>;
	extern "rust-call" fn call_once(self, (item,): Args) -> Self::Output {
	    match item {
		_ if item >= 0 => Some(item),
		_ => None
	    }
	}
    }

    impl FnMut<Args> for F {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    match item {
		_ if item >= 0 => Some(item),
		_ => None
	    }
	}
    }

    #[test]
    fn filter_map_test1() {
	let a: A<T> = A { begin: -10, end: 10 };
	let f: F = F;
	let mut filter_map: FilterMap<A<T>, F> = a.filter_map::<B, F>(f);

	for x in 0..10 {
	    let y: Option<B> = filter_map.next();
	    match y {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(filter_map.next(), None::<B>);
    }
}
