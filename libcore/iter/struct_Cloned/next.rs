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
    fn next_test1() {
	let a: A<T> = A { begin: 0, end: 10, marker: PhantomData::<&T> };
	let mut cloned: Cloned<A<T>> = a.cloned::<T>();

	for x in 0..10 {
	    let y: Option<T> = cloned.next();
	    match y {
		Some(v) => { assert_eq!(v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(cloned.next(), None::<T>);
    }
}
