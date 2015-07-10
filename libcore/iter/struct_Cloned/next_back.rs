#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::DoubleEndedIterator;
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

    impl<'a> DoubleEndedIterator for A<'a, T> {
	fn next_back(&mut self) -> Option<&'a T> {
	    unsafe {
		static mut result: T = 0;

		if self.begin < self.end {
		    self.end = self.end.wrapping_sub(1);
		    result = self.end;
		    Some::<&'a T>(&result)
		} else {
		    None::<&'a T>
		}
	    }
	}
    }

    type T = i32; // T: Clone
    Iterator_impl!(T);

    // impl<'a, I, T: 'a> DoubleEndedIterator for Cloned<I>
    //     where I: DoubleEndedIterator<Item=&'a T>, T: Clone
    // {
    //     fn next_back(&mut self) -> Option<T> {
    //         self.it.next_back().cloned()
    //     }
    // }

    #[test]
    fn next_back_test1() {
	let a: A<T> = A { begin: 0, end: 10, marker: PhantomData::<&T> };
	let mut cloned: Cloned<A<T>> = a.cloned::<T>();

	for x in 0..10 {
	    let y: Option<T> = cloned.next_back();
	    match y {
		Some(v) => { assert_eq!(v, 9 - x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(cloned.next_back(), None::<T>);
    }
}
