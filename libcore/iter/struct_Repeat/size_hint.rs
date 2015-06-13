#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
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

    // pub fn repeat<T: Clone>(elt: T) -> Repeat<T> {
    //     Repeat{element: elt}
    // }

    type T = i32;

    #[test]
    fn size_hint_test1() {
	let elt: A<T> = A { index: 68 };
	let repeat: Repeat<A<T>> = repeat(elt);
	let (lower, upper): (usize, Option<usize>) = repeat.size_hint();

	assert_eq!(lower, usize::MAX);
	assert_eq!(upper, None::<usize>);
    }
}
