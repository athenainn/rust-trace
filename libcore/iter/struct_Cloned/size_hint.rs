#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
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

    type T = i32; // T: Clone
    Iterator_impl!(T);

    // impl<'a, I, T: 'a> Iterator for Cloned<I>
    //     where I: Iterator<Item=&'a T>, T: Clone
    // {
    //     type Item = T;
    //
    //     fn next(&mut self) -> Option<T> {
    //         self.it.next().cloned()
    //     }
    //
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         self.it.size_hint()
    //     }
    // }

    #[test]
    fn size_hint_test1() {
	let a: A<T> = A { begin: 0, end: 10, marker: PhantomData::<&T> };
	let cloned: Cloned<A<T>> = a.cloned::<T>();
	let (lower, upper): (usize, Option<usize>) = cloned.size_hint();

	assert_eq!(lower, 10);
	assert_eq!(upper, Some::<usize>(10));
    }

    #[test]
    fn size_hint_test2() {
	let a: A<T> = A { begin: 0, end: 10, marker: PhantomData::<&T> };
	let mut cloned: Cloned<A<T>> = a.cloned::<T>();

	cloned.next();

	let (lower, upper): (usize, Option<usize>) = cloned.size_hint();

	assert_eq!(lower, 9);
	assert_eq!(upper, Some::<usize>(9));
    }
}
