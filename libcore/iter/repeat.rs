#![feature(core)]

extern crate core;

#[cfg(test)]
mod tests {
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

    type T = i32;

    // pub type Iterate<T, F> = Unfold<IterateState<T, F>, fn(&mut IterateState<T, F>) -> Option<T>>;

    // pub fn iterate<T, F>(seed: T, f: F) -> Iterate<T, F> where
    //     T: Clone,
    //     F: FnMut(T) -> T,
    // {
    //     fn next<T, F>(st: &mut IterateState<T, F>) -> Option<T> where
    //         T: Clone,
    //         F: FnMut(T) -> T,
    //     {
    //         let &mut (ref mut f, ref mut val, ref mut first) = st;
    //         if *first {
    //             *first = false;
    //         } else if let Some(x) = val.take() {
    //             *val = Some((*f)(x))
    //         }
    //         val.clone()
    //     }
    //
    //     // coerce to a fn pointer
    //     let next: fn(&mut IterateState<T,F>) -> Option<T> = next;
    //
    //     Unfold::new((f, Some(seed), true), next)
    // }

    #[test]
    fn repet_test1() {
	let elt: A<T> = A { index: 68 };
	let mut repeat: Repeat<A<T>> = repeat(elt);

	for _ in 0..10 {
	    let x: Option<A<T>> = repeat.next();
	    match x {
		Some(v) => assert_eq!(v.index, 68),
		None => assert!(false)
	    }
	}
    }
}
