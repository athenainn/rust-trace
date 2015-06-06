#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::vec::Vec;

    use core::iter::FromIterator;

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

    // impl<A, E, V: FromIterator<A>> FromIterator<Result<A, E>> for Result<V, E> {
    //     /// Takes each element in the `Iterator`: if it is an `Err`, no further
    //     /// elements are taken, and the `Err` is returned. Should no `Err` occur, a
    //     /// container with the values of each `Result` is returned.
    //     ///
    //     /// Here is an example which increments every integer in a vector,
    //     /// checking for overflow:
    //     ///
    //     /// ```
    //     /// use std::u32;
    //     ///
    //     /// let v = vec!(1, 2);
    //     /// let res: Result<Vec<u32>, &'static str> = v.iter().map(|&x: &u32|
    //     ///     if x == u32::MAX { Err("Overflow!") }
    //     ///     else { Ok(x + 1) }
    //     /// ).collect();
    //     /// assert!(res == Ok(vec!(2, 3)));
    //     /// ```
    //     #[inline]
    //     fn from_iter<I: IntoIterator<Item=Result<A, E>>>(iter: I) -> Result<V, E> {
    //         // FIXME(#11084): This could be replaced with Iterator::scan when this
    //         // performance bug is closed.
    //
    //         struct Adapter<Iter, E> {
    //             iter: Iter,
    //             err: Option<E>,
    //         }
    //
    //         impl<T, E, Iter: Iterator<Item=Result<T, E>>> Iterator for Adapter<Iter, E> {
    //             type Item = T;
    //
    //             #[inline]
    //             fn next(&mut self) -> Option<T> {
    //                 match self.iter.next() {
    //                     Some(Ok(value)) => Some(value),
    //                     Some(Err(err)) => {
    //                         self.err = Some(err);
    //                         None
    //                     }
    //                     None => None,
    //                 }
    //             }
    //         }
    //
    //         let mut adapter = Adapter { iter: iter.into_iter(), err: None };
    //         let v: V = FromIterator::from_iter(adapter.by_ref());
    //
    //         match adapter.err {
    //             Some(err) => Err(err),
    //             None => Ok(v),
    //         }
    //     }
    // }

    type T = u32;
    type A = T;
    type E = &'static str;
    type R = Result<A, E>;
    type I = Vec<R>;
    type V = Vec<A>;

    #[test]
    fn from_iter_test1() {
	let x: I = vec!(Ok::<A, E>(2), Ok::<A, E>(3));
	let result: Result<V, E> = Result::<V, E>::from_iter(x); // invoke x.into_iter()

	assert_eq!(result, Ok::<V, E>(vec!(2, 3)));
    }

    #[test]
    fn from_iter_test2() {
	let x: I = vec!(Err::<A, E>("error"), Ok::<A, E>(3));
	let result: Result<V, E> = Result::<V, E>::from_iter(x); // invoke x.into_iter()

	assert_eq!(result, Err::<V, E>("error"));
    }

    #[test]
    fn from_iter_test3() {
	let x: I = vec!(Ok::<A, E>(3), Err::<A, E>("error"));
	let result: Result<V, E> = Result::<V, E>::from_iter(x); // invoke x.into_iter()

	assert_eq!(result, Err::<V, E>("error"));
    }

    #[test]
    fn from_iter_test4() {
	let x: I = vec!(Err::<A, E>("error1"), Err::<A, E>("error2"));
	let result: Result<V, E> = Result::<V, E>::from_iter(x); // invoke x.into_iter()

	assert_eq!(result, Err::<V, E>("error1"));
    }
}
