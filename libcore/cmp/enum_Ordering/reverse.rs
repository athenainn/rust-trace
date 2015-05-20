#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};

    // #[derive(Clone, Copy, PartialEq, Debug)]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub enum Ordering {
    //     /// An ordering where a compared value is less [than another].
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Less = -1,
    //     /// An ordering where a compared value is equal [to another].
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Equal = 0,
    //     /// An ordering where a compared value is greater [than another].
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Greater = 1,
    // }

    // impl Ordering {
    //     /// Reverse the `Ordering`.
    //     ///
    //     /// * `Less` becomes `Greater`.
    //     /// * `Greater` becomes `Less`.
    //     /// * `Equal` becomes `Equal`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// Basic behavior:
    //     ///
    //     /// ```
    //     /// use std::cmp::Ordering;
    //     ///
    //     /// assert_eq!(Ordering::Less.reverse(), Ordering::Greater);
    //     /// assert_eq!(Ordering::Equal.reverse(), Ordering::Equal);
    //     /// assert_eq!(Ordering::Greater.reverse(), Ordering::Less);
    //     /// ```
    //     ///
    //     /// This method can be used to reverse a comparison:
    //     ///
    //     /// ```
    //     /// use std::cmp::Ordering;
    //     //     ///
    //     /// let mut data: &mut [_] = &mut [2, 10, 5, 8];
    //     ///
    //     /// // sort the array from largest to smallest.
    //     /// data.sort_by(|a, b| a.cmp(b).reverse());
    //     ///
    //     /// let b: &mut [_] = &mut [10, 8, 5, 2];
    //     /// assert!(data == b);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn reverse(self) -> Ordering {
    //         unsafe {
    //             // this compiles really nicely (to a single instruction);
    //             // an explicit match has a pile of branches and
    //             // comparisons.
    //             //
    //             // NB. it is safe because of the explicit discriminants
    //             // given above.
    //             ::mem::transmute::<_, Ordering>(-(self as i8))
    //         }
    //     }
    // }

    #[test]
    fn reverse_test1() {
	let x: Ordering = Less;
	let y: Ordering = x.reverse();

	assert_eq!(y, Greater);
    }

    #[test]
    fn reverse_test2() {
	let x: Ordering = Equal;
	let y: Ordering = x.reverse();

	assert_eq!(y, Equal);
    }

    #[test]
    fn reverse_test3() {
	let x: Ordering = Greater;
	let y: Ordering = x.reverse();

	assert_eq!(y, Less);
    }
}
