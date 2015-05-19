#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::Iterator;
    use core::iter::Unfold;

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

    // impl<A, St, F> Iterator for Unfold<St, F> where F: FnMut(&mut St) -> Option<A> {
    //     type Item = A;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<A> {
    //         (self.f)(&mut self.state)
    //     }
    //
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         // no possible known bounds at this point
    //         (0, None)
    //     }
    // }

    struct F;

    type A = T;
    type St = T;
    type Args<'a> = (&'a mut St,);

    impl<'a> FnOnce<Args<'a>> for F {
	type Output = Option<A>;
	extern "rust-call" fn call_once(self, (st,): Args) -> Self::Output {
	    *st += 1;
	    Some::<A>(*st)
	}
    }

    impl<'a> FnMut<Args<'a>> for F {
	extern "rust-call" fn call_mut(&mut self, (st,): Args) -> Self::Output {
	    *st += 1;
	    Some::<A>(*st)
	}
    }

    #[test]
    fn new_test1() {
	let initial_state: St = 68;
	let f: F = F;
	let mut unfold = Unfold::<A, St, F>::new(initial_state, f);

	for n in 1..10 {
	    let x: Option<A> = unfold.next();
	    match x {
		Some(v) => assert_eq!(v, 68 + n),
		_ => assert!(false)
	    }
	}

	assert_eq!(unfold.state, 77);
    }
}
