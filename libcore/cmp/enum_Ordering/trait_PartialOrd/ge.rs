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

    // pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    //     /// This method returns an ordering between `self` and `other` values if one exists.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::cmp::Ordering;
    //     ///
    //     /// let result = 1.0.partial_cmp(&2.0);
    //     /// assert_eq!(result, Some(Ordering::Less));
    //     ///
    //     /// let result = 1.0.partial_cmp(&1.0);
    //     /// assert_eq!(result, Some(Ordering::Equal));
    //     ///
    //     /// let result = 2.0.partial_cmp(&1.0);
    //     /// assert_eq!(result, Some(Ordering::Greater));
    //     /// ```
    //     ///
    //     /// When comparison is impossible:
    //     ///
    //     /// ```
    //     /// let result = std::f64::NAN.partial_cmp(&1.0);
    //     /// assert_eq!(result, None);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
    //
    //     /// This method tests less than (for `self` and `other`) and is used by the `<` operator.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::cmp::Ordering;
    //     ///
    //     /// let result = 1.0 < 2.0;
    //     /// assert_eq!(result, true);
    //     ///
    //     /// let result = 2.0 < 1.0;
    //     /// assert_eq!(result, false);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn lt(&self, other: &Rhs) -> bool {
    //         match self.partial_cmp(other) {
    //             Some(Less) => true,
    //             _ => false,
    //         }
    //     }
    //
    //     /// This method tests less than or equal to (for `self` and `other`) and is used by the `<=`
    //     /// operator.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let result = 1.0 <= 2.0;
    //     /// assert_eq!(result, true);
    //     ///
    //     /// let result = 2.0 <= 2.0;
    //     /// assert_eq!(result, true);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn le(&self, other: &Rhs) -> bool {
    //         match self.partial_cmp(other) {
    //             Some(Less) | Some(Equal) => true,
    //             _ => false,
    //         }
    //     }
    //
    //     /// This method tests greater than (for `self` and `other`) and is used by the `>` operator.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let result = 1.0 > 2.0;
    //     /// assert_eq!(result, false);
    //     ///
    //     /// let result = 2.0 > 2.0;
    //     /// assert_eq!(result, false);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn gt(&self, other: &Rhs) -> bool {
    //         match self.partial_cmp(other) {
    //             Some(Greater) => true,
    //             _ => false,
    //         }
    //     }
    //
    //     /// This method tests greater than or equal to (for `self` and `other`) and is used by the `>=`
    //     /// operator.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let result = 2.0 >= 1.0;
    //     /// assert_eq!(result, true);
    //     ///
    //     /// let result = 2.0 >= 2.0;
    //     /// assert_eq!(result, true);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn ge(&self, other: &Rhs) -> bool {
    //         match self.partial_cmp(other) {
    //             Some(Greater) | Some(Equal) => true,
    //             _ => false,
    //         }
    //     }
    // }

    // impl PartialOrd for Ordering {
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn partial_cmp(&self, other: &Ordering) -> Option<Ordering> {
    //         (*self as i32).partial_cmp(&(*other as i32))
    //     }
    // }

    #[test]
    fn ge_test1() {
	let x: Ordering = Less;
	let y: Ordering = Less;
	let result: bool = x.ge(&y);

	assert_eq!(result, true);
	assert_eq!(result, x >= y);
    }

    #[test]
    fn ge_test2() {
	let x: Ordering = Equal;
	let y: Ordering = Less;
	let result: bool = x.ge(&y);

	assert_eq!(result, true);
	assert_eq!(result, x >= y);
    }

    #[test]
    fn ge_test3() {
	let x: Ordering = Greater;
	let y: Ordering = Less;
	let result: bool = x.ge(&y);

	assert_eq!(result, true);
	assert_eq!(result, x >= y);
    }

    #[test]
    fn ge_test4() {
	let x: Ordering = Less;
	let y: Ordering = Equal;
	let result: bool = x.ge(&y);

	assert_eq!(result, false);
	assert_eq!(result, x >= y);
    }

    #[test]
    fn ge_test5() {
	let x: Ordering = Equal;
	let y: Ordering = Equal;
	let result: bool = x.ge(&y);

	assert_eq!(result, true);
	assert_eq!(result, x >= y);
    }

    #[test]
    fn ge_test6() {
	let x: Ordering = Greater;
	let y: Ordering = Equal;
	let result: bool = x.ge(&y);

	assert_eq!(result, true);
	assert_eq!(result, x >= y);
    }

    #[test]
    fn ge_test7() {
	let x: Ordering = Less;
	let y: Ordering = Greater;
	let result: bool = x.ge(&y);

	assert_eq!(result, false);
	assert_eq!(result, x >= y);
    }

    #[test]
    fn ge_test8() {
	let x: Ordering = Equal;
	let y: Ordering = Greater;
	let result: bool = x.ge(&y);

	assert_eq!(result, false);
	assert_eq!(result, x >= y);
    }

    #[test]
    fn ge_test9() {
	let x: Ordering = Greater;
	let y: Ordering = Greater;
	let result: bool = x.ge(&y);

	assert_eq!(result, true);
	assert_eq!(result, x >= y);
    }
}
