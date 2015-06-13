#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::RandomAccessIterator;
    use core::iter::Repeat;
    use core::iter::repeat;

    use core::clone::Clone;

    struct A<T> {
	index: T
    }

    impl Clone for A<T> {
	fn clone(&self) -> Self {
	    A { index: self.index }
	}
    }

    // impl<A: Clone> Iterator for Repeat<A> {
    //     type Item = A;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<A> { self.idx(0) }
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) { (usize::MAX, None) }
    // }

    // impl<A: Clone> RandomAccessIterator for Repeat<A> {
    //     #[inline]
    //     fn indexable(&self) -> usize { usize::MAX }
    //     #[inline]
    //     fn idx(&mut self, _: usize) -> Option<A> { Some(self.element.clone()) }
    // }

    // pub fn repeat<T: Clone>(elt: T) -> Repeat<T> {
    //     Repeat{element: elt}
    // }

    type T = i32;

    #[test]
    fn idx_test1() {
	let elt: A<T> = A { index: 68 };
	let mut repeat: Repeat<A<T>> = repeat(elt);

	for i in 0..10 {
	    let x: Option<A<T>> = repeat.idx(i);
	    match x {
		Some(v) => assert_eq!(v.index, 68),
		None => assert!(false)
	    }
	}
    }
}
