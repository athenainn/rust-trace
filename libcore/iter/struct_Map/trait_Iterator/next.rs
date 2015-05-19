#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Map;

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

		// fn map<B, F>(self, f: F) -> Map<Self, F> where
		//     Self: Sized, F: FnMut(Self::Item) -> B,
		// {
		//     Map{iter: self, f: f}
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<B, I: Iterator, F> Iterator for Map<I, F> where F: FnMut(I::Item) -> B {
    //     type Item = B;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<B> {
    //         self.iter.next().map(|a| (self.f)(a))
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         self.iter.size_hint()
    //     }
    // }

    struct F;

    type B = T;
    type Item = T;
    type Args = (Item,);

    impl FnOnce<Args> for F {
	type Output = B;
	extern "rust-call" fn call_once(self, (item,): Args) -> Self::Output {
	    -item
	}
    }

    impl FnMut<Args> for F {
	extern "rust-call" fn call_mut(&mut self, (item,): Args) -> Self::Output {
	    -item
	}
    }

    #[test]
    fn next_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let f: F = F;
	let mut map: Map<A<T>, F> = a.map::<B, F>(f);

	for x in 0..10 {
	    let y: Option<B> = map.next();
	    match y {
		Some(v) => { assert_eq!(v, -x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(map.next(), None::<B>);
    }
}
