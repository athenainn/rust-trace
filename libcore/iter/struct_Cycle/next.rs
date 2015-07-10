#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Cycle;

    use core::clone::Clone;

    struct A<T> {
	begin: T,
	end: T
    }

    impl Clone for A<T> {
	fn clone(&self) -> Self {
	    A {
		begin: self.begin,
		end: self.end
	    }
	}
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

		// fn cycle(self) -> Cycle<Self> where Self: Sized + Clone {
		//     Cycle{orig: self.clone(), iter: self}
		// }
	    }
	}
    }

    type T = i32;
    Iterator_impl!(T);

    // impl<I> Iterator for Cycle<I> where I: Clone + Iterator {
    //     type Item = <I as Iterator>::Item;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<<I as Iterator>::Item> {
    //         match self.iter.next() {
    //             None => { self.iter = self.orig.clone(); self.iter.next() }
    //             y => y
    //         }
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         // the cycle iterator is either empty or infinite
    //         match self.orig.size_hint() {
    //             sz @ (0, Some(0)) => sz,
    //             (0, _) => (0, None),
    //             _ => (usize::MAX, None)
    //         }
    //     }
    // }

    #[test]
    fn next_test1() {
	let a: A<T> = A { begin: 0, end: 3 };
	let mut cycle: Cycle<A<T>> = a.cycle();

	for _ in 0..10 {
	    for x in 0..3 {
		let y: Option<T> = cycle.next();
		match y {
		    Some(v) => { assert_eq!(v, x); }
		    None => { assert!(false); }
		}
	    }
	}

	assert_eq!(cycle.next().is_none(), false);
    }
}
