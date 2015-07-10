#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Scan;

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

		// fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>
		//     where Self: Sized, F: FnMut(&mut St, Self::Item) -> Option<B>,
		// {
		//     Scan{iter: self, f: f, state: initial_state}
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    struct F;

    type B = T;
    type St = T;
    type Item = T;
    type Args<'a> = (&'a mut St, Item);

    impl<'a> FnOnce<Args<'a>> for F {
	type Output = Option<B>;
	extern "rust-call" fn call_once(self, (st, item): Args) -> Self::Output {
	    *st += item;
	    Some::<B>(*st)
	}
    }

    impl<'a> FnMut<Args<'a>> for F {
	extern "rust-call" fn call_mut(&mut self, (st, item): Args) -> Self::Output {
	    *st += item;
	    Some::<B>(*st)
	}
    }

    #[test]
    fn scan_test1() {
	let a: A<T> = A { begin: 0, end: 10 };
	let st: St = 0;
	let f: F = F;
	let mut scan: Scan<A<T>, B, F> = a.scan::<St, B, F>(st, f);

	for n in  0..10 {
	    let x: Option<B> = scan.next();
	    match x {
		Some(v) => { assert_eq!(v, n * (n + 1) / 2); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(scan.next(), None::<B>);
    }
}
