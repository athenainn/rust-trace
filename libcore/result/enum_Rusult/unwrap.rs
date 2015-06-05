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

    // impl<T, E: fmt::Debug> Result<T, E> {
    //     /// Unwraps a result, yielding the content of an `Ok`.
    //     ///
    //     /// # Panics
    //     ///
    //     /// Panics if the value is an `Err`, with a panic message provided by the
    //     /// `Err`'s value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x: Result<u32, &str> = Ok(2);
    //     /// assert_eq!(x.unwrap(), 2);
    //     /// ```
    //     ///
    //     /// ```{.should_panic}
    //     /// let x: Result<u32, &str> = Err("emergency failure");
    //     /// x.unwrap(); // panics with `emergency failure`
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn unwrap(self) -> T {
    //         match self {
    //             Ok(t) => t,
    //             Err(e) =>
    //                 panic!("called `Result::unwrap()` on an `Err` value: {:?}", e)
    //         }
    //     }
    // }

    type T = i32;
    type E = &'static str;

    #[test]
    fn unwrap_test1() {
	let x: Result<T, E> = Ok::<T, E>(2);
	let result: T = x.unwrap();

	assert_eq!(result, 2);
    }

    #[test]
    #[should_panic]
    fn unwrap_test2() {
	let x: Result<T, E> = Err("emergency failure");
	let _: T = x.unwrap(); // panics with `emergency failure`
    }
}
