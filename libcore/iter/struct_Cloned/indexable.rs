#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::RandomAccessIterator;
    use core::iter::Cloned;

    use core::marker::PhantomData;

    struct A<'a, T: 'a + Clone> {
	begin: T,
	end: T,
	marker: PhantomData<&'a T>
    }

    macro_rules! Iterator_impl {
	($T:ty) => {
	    impl<'a> Iterator for A<'a, $T> {
		type Item = &'a $T;

		fn next(&mut self) -> Option<&'a $T> {
		    unsafe {
			static mut result: $T = 0;

			if self.begin < self.end {
			    result = self.begin;
			    self.begin = self.begin.wrapping_add(1);
			    Some::<&'a $T>(&result)
			} else {
			    None::<&'a $T>
			}
		    }
		}

		fn size_hint(&self) -> (usize, Option<usize>) {
		    debug_assert!(self.begin <= self.end);
		    let exact: usize = (self.end - self.begin) as usize;
		    (exact, Some::<usize>(exact))
		}

		// fn cloned<'a, T: 'a>(self) -> Cloned<Self>
		//     where Self: Sized + Iterator<Item=&'a T>, T: Clone
		// {
		//     Cloned { it: self }
		// }
	    }
	}
    }

    impl<'a> RandomAccessIterator for A<'a, T> {
	fn indexable(&self) -> usize {
	    let (exact, _) = self.size_hint();
	    exact
	}

	fn idx(&mut self, index: usize) -> Option<&'a T> {
	    unsafe {
		static mut result: T = 0;

		if index < self.indexable() {
		    result = self.begin + index as T;
		    Some::<&'a T>(&result)
		} else {
		    None::<&'a T>
		}
	    }
	}
    }

    type T = i32; // T: Clone
    Iterator_impl!(T);

    // impl<'a, I, T: 'a> RandomAccessIterator for Cloned<I>
    //     where I: RandomAccessIterator<Item=&'a T>, T: Clone
    // {
    //     #[inline]
    //     fn indexable(&self) -> usize {
    //         self.it.indexable()
    //     }
    //
    //     #[inline]
    //     fn idx(&mut self, index: usize) -> Option<T> {
    //         self.it.idx(index).cloned()
    //     }
    // }

    #[test]
    fn indexable_test1() {
	let a: A<T> = A { begin: 0, end: 10, marker: PhantomData::<&T> };
	let cloned: Cloned<A<T>> = a.cloned::<T>();

	assert_eq!(cloned.indexable(), 10);
    }

    #[test]
    fn indexable_test2() {
	let a: A<T> = A { begin: 0, end: 10, marker: PhantomData::<&T> };
	let mut cloned: Cloned<A<T>> = a.cloned::<T>();

	cloned.next();

	assert_eq!(cloned.indexable(), 9);
    }
}
