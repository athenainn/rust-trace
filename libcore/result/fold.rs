#![feature(core, collections, unboxed_closures)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use core::result::fold;
    use core::ops::FnMut;
    use core::ops::FnOnce;

    use collections::vec;
    use collections::vec::Vec;

    type T = u32;
    type E = &'static str;
    type R = Result<T, E>;
    type V = Vec<T>;

    struct F;

    type Args = (V, T);

    impl FnOnce<Args> for F {
	type Output = V;
	extern "rust-call" fn call_once(mut self, args: Args) -> Self::Output {
	    self.call_mut(args)
	}
    }

    impl FnMut<Args> for F {
	extern "rust-call" fn call_mut(&mut self, (mut v, t): Args) -> Self::Output {
	    v.push(t);
	    v
	}
    }

    #[test]
    fn fold_test1() {
	type VR = Vec<R>;
	type I = vec::IntoIter<R>;

	let vr: VR = vec!(Ok(3), Ok(4));
	let iterator: I = vr.into_iter();
	let mut init: V = vec!(1, 2);
	let mut f: F = F;

	let res: Result<V, E> = fold(iterator, init, f);
	assert_eq!(res.unwrap(), vec!(1, 2, 3, 4));
    }
}
