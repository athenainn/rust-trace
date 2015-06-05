#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
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

    // impl<T: fmt::Debug, E> Result<T, E> {
    //     /// Unwraps a result, yielding the content of an `Err`.
    //     ///
    //     /// # Panics
    //     ///
    //     /// Panics if the value is an `Ok`, with a custom panic message provided
    //     /// by the `Ok`'s value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```{.should_panic}
    //     /// let x: Result<u32, &str> = Ok(2);
    //     /// x.unwrap_err(); // panics with `2`
    //     /// ```
    //     ///
    //     /// ```
    //     /// let x: Result<u32, &str> = Err("emergency failure");
    //     /// assert_eq!(x.unwrap_err(), "emergency failure");
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn unwrap_err(self) -> E {
    //         match self {
    //             Ok(t) =>
    //                 panic!("called `Result::unwrap_err()` on an `Ok` value: {:?}", t),
    //             Err(e) => e
    //         }
    //     }
    // }

    type T = u32;
    type E = &'static str;

    #[test]
    #[should_panic]
    fn unwrap_err_test1() {
	let x: Result<T, E> = Ok::<T, E>(2);
	let _: E = x.unwrap_err(); // panicked at 'called `Result::unwrap_err()` on an `Ok` value: 2'
    }

    #[test]
    fn unwrap_err_test2() {
	let x: Result<T, E> = Err("emergency failure");
	let result: E = x.unwrap_err();

	assert_eq!(result, "emergency failure");
    }
}
