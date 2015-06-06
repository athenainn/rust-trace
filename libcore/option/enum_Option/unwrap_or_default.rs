#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::default::Default;

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

    // impl<T: Default> Option<T> {
    //     /// Returns the contained value or a default
    //     ///
    //     /// Consumes the `self` argument then, if `Some`, returns the contained
    //     /// value, otherwise if `None`, returns the default value for that
    //     /// type.
    //     ///
    //     /// # Examples
    //     ///
    //     /// Convert a string to an integer, turning poorly-formed strings
    //     /// into 0 (the default value for integers). `parse` converts
    //     /// a string to any other type that implements `FromStr`, returning
    //     /// `None` on error.
    //     ///
    //     /// ```
    //     /// let good_year_from_input = "1909";
    //     /// let bad_year_from_input = "190blarg";
    //     /// let good_year = good_year_from_input.parse().ok().unwrap_or_default();
    //     /// let bad_year = bad_year_from_input.parse().ok().unwrap_or_default();
    //     ///
    //     /// assert_eq!(1909, good_year);
    //     /// assert_eq!(0, bad_year);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn unwrap_or_default(self) -> T {
    //         match self {
    //             Some(x) => x,
    //             None => Default::default()
    //         }
    //     }
    // }

    struct A<T> {
	value: T
    }

    impl Default for A<T> {
	fn default() -> Self
	{
	    A { value: 68  }
	}
    }

    type T = u32; // T: Default

    #[test]
    fn unwrap_or_default_test1() {
	let x: Option<A<T>> = Some::<A<T>>(A { value: 500 });
	let result: A<T> = x.unwrap_or_default();

	assert_eq!(result.value, 500);
    }

    #[test]
    fn unwrap_or_default_test2() {
	let x: Option<A<T>> = None::<A<T>>;
	let result: A<T> = x.unwrap_or_default();

	assert_eq!(result.value, 68);
    }
}
