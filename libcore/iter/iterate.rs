#![feature(core, unboxed_closures)]

extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterate;
    use core::iter::iterate;

    use core::clone::Clone;

    struct A<T> {
	index: T
    }

    impl Clone for A<T> {
	fn clone(&self) -> Self {
	    A { index: self.index }
	}
    }

    // pub struct Unfold<St, F> {
    //     f: F,
    //     /// Internal state that will be passed to the closure on the next iteration
    //     #[unstable(feature = "core")]
    //     pub state: St,
    // }

    // impl<A, St, F> Unfold<St, F> where F: FnMut(&mut St) -> Option<A> {
    //     /// Creates a new iterator with the specified closure as the "iterator
    //     /// function" and an initial state to eventually pass to the closure
    //     #[inline]
    //     pub fn new(initial_state: St, f: F) -> Unfold<St, F> {
    //         Unfold {
    //             f: f,
    //             state: initial_state
    //         }
    //     }
    // }

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

    struct F;

    type AA = A<T>;
    type Args = (AA,);

    impl FnOnce<Args> for F {
	type Output = AA;
	extern "rust-call" fn call_once(self, (mut seed,): Args) -> Self::Output {
	    seed.index += 1;
	    seed
	}
    }

    impl FnMut<Args> for F {
	extern "rust-call" fn call_mut(&mut self, (mut seed,): Args) -> Self::Output {
	    seed.index += 1;
	    seed
	}
    }

    #[test]
    fn iterate_test1() {
	let seed: AA = A { index: 68 };
	let f: F = F;
	let mut iterate: Iterate<AA, F> = iterate::<AA, F>(seed, f);

	for x in 0..10 {
	    let y: Option<AA> = iterate.next();
	    match y {
		Some(v) => assert_eq!(v.index, 68 + x),
		None => assert!(false)
	    }
	}
    }
}
