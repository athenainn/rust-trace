#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::DoubleEndedIterator;
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

    // impl<A: Clone> DoubleEndedIterator for Repeat<A> {
    //     #[inline]
    //     fn next_back(&mut self) -> Option<A> { self.idx(0) }
    // }

    // pub fn repeat<T: Clone>(elt: T) -> Repeat<T> {
    //     Repeat{element: elt}
    // }

    type T = i32;

    #[test]
    fn next_back_test1() {
	let elt: A<T> = A { index: 68 };
	let mut repeat: Repeat<A<T>> = repeat(elt);

	for _ in 0..10 {
	    let x: Option<A<T>> = repeat.next_back();
	    match x {
		Some(v) => assert_eq!(v.index, 68),
		None => assert!(false)
	    }
	}
    }
}
