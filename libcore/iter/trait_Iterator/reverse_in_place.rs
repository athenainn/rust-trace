#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::DoubleEndedIterator;
    use core::slice;

    struct A<'a, T: 'a> {
	data: &'a mut [T]
    }

    impl<'a, T> A<'a, T> {
	#[inline]
	fn iter_mut(&mut self) -> IterMut<T> {
	    IterMut { inner: self.data.iter_mut() }
	}
    }

    struct IterMut<'a, T: 'a> { inner: slice::IterMut<'a, T>  }

    impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;

	#[inline]
	fn next(&mut self) -> Option<&'a mut T> { self.inner.next() }

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
	    let n = self.inner.len();
	    (n, Some::<usize>(n))
	}

	// fn reverse_in_place<'a, T: 'a>(&mut self) where
	//     Self: Sized + Iterator<Item=&'a mut T> + DoubleEndedIterator
	// {
	//     loop {
	//         match (self.next(), self.next_back()) {
	//             (Some(x), Some(y)) => mem::swap(x, y),
	//             _ => break
	//         }
	//     }
	// }
    }

    impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<&'a mut T> {
	    self.inner.next_back()
	}
    }

    type T = i32;

    #[test]
    fn reverse_in_place_test1() {
	let mut a: A<T> = A { data: &mut [1, 2, 3, 4, 5] };
	let mut iter_mut: IterMut<T> = a.iter_mut();

	for x in 1..6 {
	    let y: Option<<IterMut<T> as Iterator>::Item> = iter_mut.next();
	    match y {
		Some(v) => { assert_eq!(*v, x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(iter_mut.next(), None::<<IterMut<T> as Iterator>::Item>);
    }

    #[test]
    fn reverse_in_place_test2() {
	let mut a: A<T> = A { data: &mut [1, 2, 3, 4, 5] };

	{
	    let mut iter_mut: IterMut<T> = a.iter_mut();
	    iter_mut.reverse_in_place::<T>();
	}

	let mut iter_mut: IterMut<T> = a.iter_mut();

	for x in 1..6 {
	    let y: Option<<IterMut<T> as Iterator>::Item> = iter_mut.next();
	    match y {
		Some(v) => { assert_eq!(*v, 6 - x); }
		None => { assert!(false); }
	    }
	}

	assert_eq!(iter_mut.next(), None::<<IterMut<T> as Iterator>::Item>);
    }
}
