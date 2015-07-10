#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Cycle;

    use core::clone::Clone;

    use core::usize;

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

		fn size_hint(&self) -> (usize, Option<usize>) {
		    debug_assert!(self.begin <= self.end);
		    let exact: usize = (self.end - self.begin) as usize;
		    (exact, Some::<usize>(exact))
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
    fn size_hint_test1() {
	let a: A<T> = A { begin: 0, end: 0 };
	let cycle: Cycle<A<T>> = a.cycle();

	assert_eq!(cycle.size_hint(), (0, Some::<usize>(0)));
    }

    #[test]
    fn size_hint_test2() {
	use core::marker::PhantomData;

	struct B<T> { marker: PhantomData<T> };
	impl Iterator for B<T> {
	    type Item = T;

	    fn next(&mut self) -> Option<Self::Item> {
		None
	    }

	    // fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
	}

	impl<T: Copy> Clone for B<T> {
	    fn clone(&self) -> Self {
		B { marker: self.marker }
	    }
	}

	let b: B<T> = B { marker: PhantomData::<T> };
	let cycle: Cycle<B<T>> = b.cycle();

	assert_eq!(cycle.size_hint(), (0, None::<usize>));
    }

    #[test]
    fn size_hint_test3() {
	let a: A<T> = A { begin: 0, end: 3 };
	let cycle: Cycle<A<T>> = a.cycle();

	assert_eq!(cycle.size_hint(), (usize::MAX, None::<usize>));
    }
}
