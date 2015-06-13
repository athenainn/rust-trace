#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::RandomAccessIterator;
    use core::iter::Repeat;
    use core::iter::repeat;

    use core::clone::Clone;

    use core::usize;

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
    fn indexable_test1() {
	let elt: A<T> = A { index: 68 };
	let repeat: Repeat<A<T>> = repeat(elt);
	let indexable: usize = repeat.indexable();

	assert_eq!(indexable, usize::MAX);
    }
}
