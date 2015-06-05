#![feature(core, collections, unboxed_closures)]

extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use core::result::fold;

    use core::ops::FnOnce;
    use core::ops::FnMut;

    use collections::vec;

    // #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
    // #[must_use]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub enum Result<T, E> {
    //     /// Contains the success value
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Ok(T),
    //
    //     /// Contains the error value
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Err(E)
    // }

    // pub fn fold<T,
    //             V,
    //             E,
    //             F: FnMut(V, T) -> V,
    //             Iter: Iterator<Item=Result<T, E>>>(
    //             iterator: Iter,
    //             mut init: V,
    //             mut f: F)
    //             -> Result<V, E> {
    //     for t in iterator {
    //         match t {
    //             Ok(v) => init = f(init, v),
    //             Err(u) => return Err(u)
    //         }
    //     }
    //     Ok(init)
    // }

    struct F;

    type T = u32;
    type E = &'static str;
    type V = vec::Vec<T>;

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

    type R = Result<T, E>;
    type VR = vec::Vec<R>;
    type Iter = vec::IntoIter<R>; // Iter:: Iterator<Item=Result<T, E>>

    #[test]
    fn fold_test1() {
	let x: VR = vec!(Ok::<T, E>(3), Ok::<T, E>(4));
	let iterator: Iter = x.into_iter();
	let init: V = vec!(1, 2);
	let f: F = F;
	let result: Result<V, E> = fold::<T, V, E, F, Iter>(iterator, init, f);

	assert_eq!(result, Ok::<V, E>(vec!(1, 2, 3, 4)));
    }

    #[test]
    fn fold_test2() {
	let x: VR = vec!(Err::<T, E>("error"), Ok::<T, E>(4));
	let iterator: Iter = x.into_iter();
	let init: V = vec!(1, 2);
	let f: F = F;
	let result: Result<V, E> = fold::<T, V, E, F, Iter>(iterator, init, f);

	assert_eq!(result, Err::<V, E>("error"));
    }

    #[test]
    fn fold_test3() {
	let x: VR = vec!(Ok::<T, E>(3), Err::<T, E>("error"));
	let iterator: Iter = x.into_iter();
	let init: V = vec!(1, 2);
	let f: F = F;
	let result: Result<V, E> = fold::<T, V, E, F, Iter>(iterator, init, f);

	assert_eq!(result, Err::<V, E>("error"));
    }

    #[test]
    fn fold_test4() {
	let x: VR = vec!(Err::<T, E>("error1"), Err::<T, E>("error2"));
	let iterator: Iter = x.into_iter();
	let init: V = vec!(1, 2);
	let f: F = F;
	let result: Result<V, E> = fold::<T, V, E, F, Iter>(iterator, init, f);

	assert_eq!(result, Err::<V, E>("error1"));
    }
}
