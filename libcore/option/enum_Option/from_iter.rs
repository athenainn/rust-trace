#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::vec::Vec;

    use core::iter::FromIterator;

    // #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub enum Option<T> {
    //     /// No value
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     None,
    //     /// Some value `T`
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Some(T)
    // }

    // impl<A, V: FromIterator<A>> FromIterator<Option<A>> for Option<V> {
    //     /// Takes each element in the `Iterator`: if it is `None`, no further
    //     /// elements are taken, and the `None` is returned. Should no `None` occur, a
    //     /// container with the values of each `Option` is returned.
    //     ///
    //     /// Here is an example which increments every integer in a vector,
    //     /// checking for overflow:
    //     ///
    //     /// ```
    //     /// use std::u16;
    //     ///
    //     /// let v = vec!(1, 2);
    //     /// let res: Option<Vec<u16>> = v.iter().map(|&x: &u16|
    //     ///     if x == u16::MAX { None }
    //     ///     else { Some(x + 1) }
    //     /// ).collect();
    //     /// assert!(res == Some(vec!(2, 3)));
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn from_iter<I: IntoIterator<Item=Option<A>>>(iter: I) -> Option<V> {
    //         // FIXME(#11084): This could be replaced with Iterator::scan when this
    //         // performance bug is closed.
    //
    //         struct Adapter<Iter> {
    //             iter: Iter,
    //             found_none: bool,
    //         }
    //
    //         impl<T, Iter: Iterator<Item=Option<T>>> Iterator for Adapter<Iter> {
    //             type Item = T;
    //
    //             #[inline]
    //             fn next(&mut self) -> Option<T> {
    //                 match self.iter.next() {
    //                     Some(Some(value)) => Some(value),
    //                     Some(None) => {
    //                         self.found_none = true;
    //                         None
    //                     }
    //                     None => None,
    //                 }
    //             }
    //         }
    //
    //         let mut adapter = Adapter { iter: iter.into_iter(), found_none: false };
    //         let v: V = FromIterator::from_iter(adapter.by_ref());
    //
    //         if adapter.found_none {
    //             None
    //         } else {
    //             Some(v)
    //         }
    //     }
    // }

    type T = u32;
    type A = T;
    type O = Option<A>;
    type I = Vec<O>;
    type V = Vec<A>;

    #[test]
    fn from_iter_test1() {
	let x: I = vec!(Some::<A>(2), Some::<A>(3));
	let result: Option<V> = Option::<V>::from_iter(x); // invoke x.into_iter()

	assert_eq!(result, Some::<V>(vec!(2, 3)));
    }

    #[test]
    fn from_iter_test2() {
	let x: I = vec!(None::<A>, Some::<A>(3));
	let result: Option<V> = Option::<V>::from_iter(x); // invoke x.into_iter()

	assert_eq!(result, None::<V>);
    }

    #[test]
    fn from_iter_test3() {
	let x: I = vec!(Some::<A>(2), None::<A>);
	let result: Option<V> = Option::<V>::from_iter(x); // invoke x.into_iter()

	assert_eq!(result, None::<V>);
    }

    #[test]
    fn from_iter_test4() {
	let x: I = vec!(None::<A>, None::<A>);
	let result: Option<V> = Option::<V>::from_iter(x); // invoke x.into_iter()

	assert_eq!(result, None::<V>);
    }
}
