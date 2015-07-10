#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::result::Iter;

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

    // impl<T, E> Result<T, E> {
    //     /////////////////////////////////////////////////////////////////////////
    //     // Querying the contained values
    //     /////////////////////////////////////////////////////////////////////////
    //
    //     /// Returns true if the result is `Ok`
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x: Result<i32, &str> = Ok(-3);
    //     /// assert_eq!(x.is_ok(), true);
    //     ///
    //     /// let x: Result<i32, &str> = Err("Some error message");
    //     /// assert_eq!(x.is_ok(), false);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn is_ok(&self) -> bool {
    //         match *self {
    //             Ok(_) => true,
    //             Err(_) => false
    //         }
    //     }
    //
    //     /// Returns true if the result is `Err`
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x: Result<i32, &str> = Ok(-3);
    //     /// assert_eq!(x.is_err(), false);
    //     ///
    //     /// let x: Result<i32, &str> = Err("Some error message");
    //     /// assert_eq!(x.is_err(), true);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn is_err(&self) -> bool {
    //         !self.is_ok()
    //     }
    //
    //     /////////////////////////////////////////////////////////////////////////
    //     // Adapter for each variant
    //     /////////////////////////////////////////////////////////////////////////
    //
    //     /// Converts from `Result<T, E>` to `Option<T>`
    //     ///
    //     /// Converts `self` into an `Option<T>`, consuming `self`,
    //     /// and discarding the error, if any.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x: Result<u32, &str> = Ok(2);
    //     /// assert_eq!(x.ok(), Some(2));
    //     ///
    //     /// let x: Result<u32, &str> = Err("Nothing here");
    //     /// assert_eq!(x.ok(), None);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn ok(self) -> Option<T> {
    //         match self {
    //             Ok(x)  => Some(x),
    //             Err(_) => None,
    //         }
    //     }
    //
    //     /// Converts from `Result<T, E>` to `Option<E>`
    //     ///
    //     /// Converts `self` into an `Option<E>`, consuming `self`,
    //     /// and discarding the success value, if any.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x: Result<u32, &str> = Ok(2);
    //     /// assert_eq!(x.err(), None);
    //     ///
    //     /// let x: Result<u32, &str> = Err("Nothing here");
    //     /// assert_eq!(x.err(), Some("Nothing here"));
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn err(self) -> Option<E> {
    //         match self {
    //             Ok(_)  => None,
    //             Err(x) => Some(x),
    //         }
    //     }
    //
    //     /////////////////////////////////////////////////////////////////////////
    //     // Adapter for working with references
    //     /////////////////////////////////////////////////////////////////////////
    //
    //     /// Converts from `Result<T, E>` to `Result<&T, &E>`
    //     ///
    //     /// Produces a new `Result`, containing a reference
    //     /// into the original, leaving the original in place.
    //     ///
    //     /// ```
    //     /// let x: Result<u32, &str> = Ok(2);
    //     /// assert_eq!(x.as_ref(), Ok(&2));
    //     ///
    //     /// let x: Result<u32, &str> = Err("Error");
    //     /// assert_eq!(x.as_ref(), Err(&"Error"));
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn as_ref(&self) -> Result<&T, &E> {
    //         match *self {
    //             Ok(ref x) => Ok(x),
    //             Err(ref x) => Err(x),
    //         }
    //     }
    //
    //     /// Converts from `Result<T, E>` to `Result<&mut T, &mut E>`
    //     ///
    //     /// ```
    //     /// fn mutate(r: &mut Result<i32, i32>) {
    //     ///     match r.as_mut() {
    //     ///         Ok(&mut ref mut v) => *v = 42,
    //     ///         Err(&mut ref mut e) => *e = 0,
    //     ///     }
    //     /// }
    //     ///
    //     /// let mut x: Result<i32, i32> = Ok(2);
    //     /// mutate(&mut x);
    //     /// assert_eq!(x.unwrap(), 42);
    //     ///
    //     /// let mut x: Result<i32, i32> = Err(13);
    //     /// mutate(&mut x);
    //     /// assert_eq!(x.unwrap_err(), 0);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn as_mut(&mut self) -> Result<&mut T, &mut E> {
    //         match *self {
    //             Ok(ref mut x) => Ok(x),
    //             Err(ref mut x) => Err(x),
    //         }
    //     }
    //
    //     /// Converts from `Result<T, E>` to `&[T]` (without copying)
    //     #[inline]
    //     #[unstable(feature = "as_slice", since = "unsure of the utility here")]
    //     pub fn as_slice(&self) -> &[T] {
    //         match *self {
    //             Ok(ref x) => slice::ref_slice(x),
    //             Err(_) => {
    //                 // work around lack of implicit coercion from fixed-size array to slice
    //                 let emp: &[_] = &[];
    //                 emp
    //             }
    //         }
    //     }
    //
    //     /// Converts from `Result<T, E>` to `&mut [T]` (without copying)
    //     ///
    //     /// ```
    //     /// # #![feature(core)]
    //     /// let mut x: Result<&str, u32> = Ok("Gold");
    //     /// {
    //     ///     let v = x.as_mut_slice();
    //     ///     assert!(v == ["Gold"]);
    //     ///     v[0] = "Silver";
    //     ///     assert!(v == ["Silver"]);
    //     /// }
    //     /// assert_eq!(x, Ok("Silver"));
    //     ///
    //     /// let mut x: Result<&str, u32> = Err(45);
    //     /// assert!(x.as_mut_slice().is_empty());
    //     /// ```
    //     #[inline]
    //     #[unstable(feature = "core",
    //                reason = "waiting for mut conventions")]
    //     pub fn as_mut_slice(&mut self) -> &mut [T] {
    //         match *self {
    //             Ok(ref mut x) => slice::mut_ref_slice(x),
    //             Err(_) => {
    //                 // work around lack of implicit coercion from fixed-size array to slice
    //                 let emp: &mut [_] = &mut [];
    //                 emp
    //             }
    //         }
    //     }
    //
    //     /////////////////////////////////////////////////////////////////////////
    //     // Transforming contained values
    //     /////////////////////////////////////////////////////////////////////////
    //
    //     /// Maps a `Result<T, E>` to `Result<U, E>` by applying a function to an
    //     /// contained `Ok` value, leaving an `Err` value untouched.
    //     ///
    //     /// This function can be used to compose the results of two functions.
    //     ///
    //     /// # Examples
    //     ///
    //     /// Print the numbers on each line of a string multiplied by two.
    //     ///
    //     /// ```
    //     /// let line = "1\n2\n3\n4\n";
    //     ///
    //     /// for num in line.lines() {
    //     ///     match num.parse::<i32>().map(|i| i * 2) {
    //     ///         Ok(n) => println!("{}", n),
    //     ///         Err(..) => {}
    //     ///     }
    //     /// }
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn map<U, F: FnOnce(T) -> U>(self, op: F) -> Result<U,E> {
    //         match self {
    //             Ok(t) => Ok(op(t)),
    //             Err(e) => Err(e)
    //         }
    //     }
    //
    //     /// Maps a `Result<T, E>` to `Result<T, F>` by applying a function to an
    //     /// contained `Err` value, leaving an `Ok` value untouched.
    //     ///
    //     /// This function can be used to pass through a successful result while handling
    //     /// an error.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// fn stringify(x: u32) -> String { format!("error code: {}", x) }
    //     ///
    //     /// let x: Result<u32, u32> = Ok(2);
    //     /// assert_eq!(x.map_err(stringify), Ok(2));
    //     ///
    //     /// let x: Result<u32, u32> = Err(13);
    //     /// assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Result<T,F> {
    //         match self {
    //             Ok(t) => Ok(t),
    //             Err(e) => Err(op(e))
    //         }
    //     }
    //
    //     /////////////////////////////////////////////////////////////////////////
    //     // Iterator constructors
    //     /////////////////////////////////////////////////////////////////////////
    //
    //     /// Returns an iterator over the possibly contained value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x: Result<u32, &str> = Ok(7);
    //     /// assert_eq!(x.iter().next(), Some(&7));
    //     ///
    //     /// let x: Result<u32, &str> = Err("nothing!");
    //     /// assert_eq!(x.iter().next(), None);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn iter(&self) -> Iter<T> {
    //         Iter { inner: self.as_ref().ok() }
    //     }
    //
    //     /// Returns a mutable iterator over the possibly contained value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let mut x: Result<u32, &str> = Ok(7);
    //     /// match x.iter_mut().next() {
    //     ///     Some(&mut ref mut x) => *x = 40,
    //     ///     None => {},
    //     /// }
    //     /// assert_eq!(x, Ok(40));
    //     ///
    //     /// let mut x: Result<u32, &str> = Err("nothing!");
    //     /// assert_eq!(x.iter_mut().next(), None);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn iter_mut(&mut self) -> IterMut<T> {
    //         IterMut { inner: self.as_mut().ok() }
    //     }
    //
    //     ////////////////////////////////////////////////////////////////////////
    //     // Boolean operations on the values, eager and lazy
    //     /////////////////////////////////////////////////////////////////////////
    //
    //     /// Returns `res` if the result is `Ok`, otherwise returns the `Err` value of `self`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x: Result<u32, &str> = Ok(2);
    //     /// let y: Result<&str, &str> = Err("late error");
    //     /// assert_eq!(x.and(y), Err("late error"));
    //     ///
    //     /// let x: Result<u32, &str> = Err("early error");
    //     /// let y: Result<&str, &str> = Ok("foo");
    //     /// assert_eq!(x.and(y), Err("early error"));
    //     ///
    //     /// let x: Result<u32, &str> = Err("not a 2");
    //     /// let y: Result<&str, &str> = Err("late error");
    //     /// assert_eq!(x.and(y), Err("not a 2"));
    //     ///
    //     /// let x: Result<u32, &str> = Ok(2);
    //     /// let y: Result<&str, &str> = Ok("different result type");
    //     /// assert_eq!(x.and(y), Ok("different result type"));
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn and<U>(self, res: Result<U, E>) -> Result<U, E> {
    //         match self {
    //             Ok(_) => res,
    //             Err(e) => Err(e),
    //         }
    //     }
    //
    //     /// Calls `op` if the result is `Ok`, otherwise returns the `Err` value of `self`.
    //     ///
    //     /// This function can be used for control flow based on result values.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// fn sq(x: u32) -> Result<u32, u32> { Ok(x * x) }
    //     /// fn err(x: u32) -> Result<u32, u32> { Err(x) }
    //     ///
    //     /// assert_eq!(Ok(2).and_then(sq).and_then(sq), Ok(16));
    //     /// assert_eq!(Ok(2).and_then(sq).and_then(err), Err(4));
    //     /// assert_eq!(Ok(2).and_then(err).and_then(sq), Err(2));
    //     /// assert_eq!(Err(3).and_then(sq).and_then(sq), Err(3));
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn and_then<U, F: FnOnce(T) -> Result<U, E>>(self, op: F) -> Result<U, E> {
    //         match self {
    //             Ok(t) => op(t),
    //             Err(e) => Err(e),
    //         }
    //     }
    //
    //     /// Returns `res` if the result is `Err`, otherwise returns the `Ok` value of `self`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x: Result<u32, &str> = Ok(2);
    //     /// let y: Result<u32, &str> = Err("late error");
    //     /// assert_eq!(x.or(y), Ok(2));
    //     ///
    //     /// let x: Result<u32, &str> = Err("early error");
    //     /// let y: Result<u32, &str> = Ok(2);
    //     /// assert_eq!(x.or(y), Ok(2));
    //     ///
    //     /// let x: Result<u32, &str> = Err("not a 2");
    //     /// let y: Result<u32, &str> = Err("late error");
    //     /// assert_eq!(x.or(y), Err("late error"));
    //     ///
    //     /// let x: Result<u32, &str> = Ok(2);
    //     /// let y: Result<u32, &str> = Ok(100);
    //     /// assert_eq!(x.or(y), Ok(2));
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn or<F>(self, res: Result<T, F>) -> Result<T, F> {
    //         match self {
    //             Ok(v) => Ok(v),
    //             Err(_) => res,
    //         }
    //     }
    //
    //     /// Calls `op` if the result is `Err`, otherwise returns the `Ok` value of `self`.
    //     ///
    //     /// This function can be used for control flow based on result values.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// fn sq(x: u32) -> Result<u32, u32> { Ok(x * x) }
    //     /// fn err(x: u32) -> Result<u32, u32> { Err(x) }
    //     ///
    //     /// assert_eq!(Ok(2).or_else(sq).or_else(sq), Ok(2));
    //     /// assert_eq!(Ok(2).or_else(err).or_else(sq), Ok(2));
    //     /// assert_eq!(Err(3).or_else(sq).or_else(err), Ok(9));
    //     /// assert_eq!(Err(3).or_else(err).or_else(err), Err(3));
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn or_else<F, O: FnOnce(E) -> Result<T, F>>(self, op: O) -> Result<T, F> {
    //         match self {
    //             Ok(t) => Ok(t),
    //             Err(e) => op(e),
    //         }
    //     }
    //
    //     /// Unwraps a result, yielding the content of an `Ok`.
    //     /// Else it returns `optb`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let optb = 2;
    //     /// let x: Result<u32, &str> = Ok(9);
    //     /// assert_eq!(x.unwrap_or(optb), 9);
    //     ///
    //     /// let x: Result<u32, &str> = Err("error");
    //     /// assert_eq!(x.unwrap_or(optb), optb);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn unwrap_or(self, optb: T) -> T {
    //         match self {
    //             Ok(t) => t,
    //             Err(_) => optb
    //         }
    //     }
    //
    //     /// Unwraps a result, yielding the content of an `Ok`.
    //     /// If the value is an `Err` then it calls `op` with its value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// fn count(x: &str) -> usize { x.len() }
    //     ///
    //     /// assert_eq!(Ok(2).unwrap_or_else(count), 2);
    //     /// assert_eq!(Err("foo").unwrap_or_else(count), 3);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn unwrap_or_else<F: FnOnce(E) -> T>(self, op: F) -> T {
    //         match self {
    //             Ok(t) => t,
    //             Err(e) => op(e)
    //         }
    //     }
    // }

    // pub struct Iter<'a, T: 'a> { inner: Option<&'a T> }

    // impl<'a, T> Iterator for Iter<'a, T> {
    //     type Item = &'a T;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<&'a T> { self.inner.take() }
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) {
    //         let n = if self.inner.is_some() {1} else {0};
    //         (n, Some(n))
    //     }
    // }

    // impl<'a, T> Clone for Iter<'a, T> {
    //     fn clone(&self) -> Iter<'a, T> { Iter { inner: self.inner } }
    // }

    type T = u32;
    type E = &'static str;

    #[test]
    fn clone_test1() {
	let x: Result<T, E> = Ok::<T, E>(7);
	let mut iter: Iter<T> = x.iter();
	let mut clone: Iter<T> = iter.clone();

	let a: Option<&T> = iter.next();
	let b: Option<&T> = clone.next();

	assert_eq!(a, Some::<&T>(&7));
	assert_eq!(b, Some::<&T>(&7));
    }

    #[test]
    fn clone_test2() {
	let x: Result<T, E> = Err::<T, E>("nothing!");
	let mut iter: Iter<T> = x.iter();
	let mut clone: Iter<T> = iter.clone();

	let a: Option<&T> = iter.next();
	let b: Option<&T> = clone.next();

	assert_eq!(a, None::<&T>);
	assert_eq!(b, None::<&T>);

    }
}
