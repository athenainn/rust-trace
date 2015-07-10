#![feature(core, unboxed_closures)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::clone::Clone;

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

    // impl<T> Option<T> {
    //     /////////////////////////////////////////////////////////////////////////
    //     // Querying the contained values
    //     /////////////////////////////////////////////////////////////////////////
    //
    //     /// Returns `true` if the option is a `Some` value
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x: Option<u32> = Some(2);
    //     /// assert_eq!(x.is_some(), true);
    //     ///
    //     /// let x: Option<u32> = None;
    //     /// assert_eq!(x.is_some(), false);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn is_some(&self) -> bool {
    //         match *self {
    //             Some(_) => true,
    //             None => false
    //         }
    //     }
    //
    //     /// Returns `true` if the option is a `None` value
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x: Option<u32> = Some(2);
    //     /// assert_eq!(x.is_none(), false);
    //     ///
    //     /// let x: Option<u32> = None;
    //     /// assert_eq!(x.is_none(), true);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn is_none(&self) -> bool {
    //         !self.is_some()
    //     }
    //
    //     /////////////////////////////////////////////////////////////////////////
    //     // Adapter for working with references
    //     /////////////////////////////////////////////////////////////////////////
    //
    //     /// Converts from `Option<T>` to `Option<&T>`
    //     ///
    //     /// # Examples
    //     ///
    //     /// Convert an `Option<String>` into an `Option<usize>`, preserving the original.
    //     /// The `map` method takes the `self` argument by value, consuming the original,
    //     /// so this technique uses `as_ref` to first take an `Option` to a reference
    //     /// to the value inside the original.
    //     ///
    //     /// ```
    //     /// let num_as_str: Option<String> = Some("10".to_string());
    //     /// // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    //     /// // then consume *that* with `map`, leaving `num_as_str` on the stack.
    //     /// let num_as_int: Option<usize> = num_as_str.as_ref().map(|n| n.len());
    //     /// println!("still can print num_as_str: {:?}", num_as_str);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn as_ref<'r>(&'r self) -> Option<&'r T> {
    //         match *self {
    //             Some(ref x) => Some(x),
    //             None => None
    //         }
    //     }
    //
    //     /// Converts from `Option<T>` to `Option<&mut T>`
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let mut x = Some(2);
    //     /// match x.as_mut() {
    //     ///     Some(v) => *v = 42,
    //     ///     None => {},
    //     /// }
    //     /// assert_eq!(x, Some(42));
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn as_mut<'r>(&'r mut self) -> Option<&'r mut T> {
    //         match *self {
    //             Some(ref mut x) => Some(x),
    //             None => None
    //         }
    //     }
    //
    //     /// Converts from `Option<T>` to `&mut [T]` (without copying)
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(core)]
    //     /// let mut x = Some("Diamonds");
    //     /// {
    //     ///     let v = x.as_mut_slice();
    //     ///     assert!(v == ["Diamonds"]);
    //     ///     v[0] = "Dirt";
    //     ///     assert!(v == ["Dirt"]);
    //     /// }
    //     /// assert_eq!(x, Some("Dirt"));
    //     /// ```
    //     #[inline]
    //     #[unstable(feature = "core",
    //                reason = "waiting for mut conventions")]
    //     pub fn as_mut_slice<'r>(&'r mut self) -> &'r mut [T] {
    //         match *self {
    //             Some(ref mut x) => {
    //                 let result: &mut [T] = slice::mut_ref_slice(x);
    //                 result
    //             }
    //             None => {
    //                 let result: &mut [T] = &mut [];
    //                 result
    //             }
    //         }
    //     }
    //
    //     /////////////////////////////////////////////////////////////////////////
    //     // Getting to contained values
    //     /////////////////////////////////////////////////////////////////////////
    //
    //     /// Unwraps an option, yielding the content of a `Some`
    //     ///
    //     /// # Panics
    //     ///
    //     /// Panics if the value is a `None` with a custom panic message provided by
    //     /// `msg`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x = Some("value");
    //     /// assert_eq!(x.expect("the world is ending"), "value");
    //     /// ```
    //     ///
    //     /// ```{.should_panic}
    //     /// let x: Option<&str> = None;
    //     /// x.expect("the world is ending"); // panics with `the world is ending`
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn expect(self, msg: &str) -> T {
    //         match self {
    //             Some(val) => val,
    //             None => panic!("{}", msg),
    //         }
    //     }
    //
    //     /// Moves the value `v` out of the `Option<T>` if it is `Some(v)`.
    //     ///
    //     /// # Panics
    //     ///
    //     /// Panics if the self value equals `None`.
    //     ///
    //     /// # Safety note
    //     ///
    //     /// In general, because this function may panic, its use is discouraged.
    //     /// Instead, prefer to use pattern matching and handle the `None`
    //     /// case explicitly.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x = Some("air");
    //     /// assert_eq!(x.unwrap(), "air");
    //     /// ```
    //     ///
    //     /// ```{.should_panic}
    //     /// let x: Option<&str> = None;
    //     /// assert_eq!(x.unwrap(), "air"); // fails
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn unwrap(self) -> T {
    //         match self {
    //             Some(val) => val,
    //             None => panic!("called `Option::unwrap()` on a `None` value"),
    //         }
    //     }
    //
    //     /// Returns the contained value or a default.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// assert_eq!(Some("car").unwrap_or("bike"), "car");
    //     /// assert_eq!(None.unwrap_or("bike"), "bike");
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn unwrap_or(self, def: T) -> T {
    //         match self {
    //             Some(x) => x,
    //             None => def
    //         }
    //     }
    //
    //     /// Returns the contained value or computes it from a closure.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let k = 10;
    //     /// assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
    //     /// assert_eq!(None.unwrap_or_else(|| 2 * k), 20);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
    //         match self {
    //             Some(x) => x,
    //             None => f()
    //         }
    //     }
    //
    //     /////////////////////////////////////////////////////////////////////////
    //     // Transforming contained values
    //     /////////////////////////////////////////////////////////////////////////
    //
    //     /// Maps an `Option<T>` to `Option<U>` by applying a function to a contained value
    //     ///
    //     /// # Examples
    //     ///
    //     /// Convert an `Option<String>` into an `Option<usize>`, consuming the original:
    //     ///
    //     /// ```
    //     /// let maybe_some_string = Some(String::from("Hello, World!"));
    //     /// // `Option::map` takes self *by value*, consuming `maybe_some_string`
    //     /// let maybe_some_len = maybe_some_string.map(|s| s.len());
    //     ///
    //     /// assert_eq!(maybe_some_len, Some(13));
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
    //         match self {
    //             Some(x) => Some(f(x)),
    //             None => None
    //         }
    //     }
    //
    //     /// Applies a function to the contained value (if any),
    //     /// or returns a `default` (if not).
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x = Some("foo");
    //     /// assert_eq!(x.map_or(42, |v| v.len()), 3);
    //     ///
    //     /// let x: Option<&str> = None;
    //     /// assert_eq!(x.map_or(42, |v| v.len()), 42);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
    //         match self {
    //             Some(t) => f(t),
    //             None => default,
    //         }
    //     }
    //
    //     /// Applies a function to the contained value (if any),
    //     /// or computes a `default` (if not).
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let k = 21;
    //     ///
    //     /// let x = Some("foo");
    //     /// assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);
    //     ///
    //     /// let x: Option<&str> = None;
    //     /// assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
    //         match self {
    //             Some(t) => f(t),
    //             None => default()
    //         }
    //     }
    //
    //     /// Transforms the `Option<T>` into a `Result<T, E>`, mapping `Some(v)` to
    //     /// `Ok(v)` and `None` to `Err(err)`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(core)]
    //     /// let x = Some("foo");
    //     /// assert_eq!(x.ok_or(0), Ok("foo"));
    //     ///
    //     /// let x: Option<&str> = None;
    //     /// assert_eq!(x.ok_or(0), Err(0));
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn ok_or<E>(self, err: E) -> Result<T, E> {
    //         match self {
    //             Some(v) => Ok(v),
    //             None => Err(err),
    //         }
    //     }
    //
    //     /// Transforms the `Option<T>` into a `Result<T, E>`, mapping `Some(v)` to
    //     /// `Ok(v)` and `None` to `Err(err())`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(core)]
    //     /// let x = Some("foo");
    //     /// assert_eq!(x.ok_or_else(|| 0), Ok("foo"));
    //     ///
    //     /// let x: Option<&str> = None;
    //     /// assert_eq!(x.ok_or_else(|| 0), Err(0));
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn ok_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> {
    //         match self {
    //             Some(v) => Ok(v),
    //             None => Err(err()),
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
    //     /// let x = Some(4);
    //     /// assert_eq!(x.iter().next(), Some(&4));
    //     ///
    //     /// let x: Option<u32> = None;
    //     /// assert_eq!(x.iter().next(), None);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn iter(&self) -> Iter<T> {
    //         Iter { inner: Item { opt: self.as_ref() } }
    //     }
    //
    //     /// Returns a mutable iterator over the possibly contained value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(core)]
    //     /// let mut x = Some(4);
    //     /// match x.iter_mut().next() {
    //     ///     Some(&mut ref mut v) => *v = 42,
    //     ///     None => {},
    //     /// }
    //     /// assert_eq!(x, Some(42));
    //     ///
    //     /// let mut x: Option<u32> = None;
    //     /// assert_eq!(x.iter_mut().next(), None);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn iter_mut(&mut self) -> IterMut<T> {
    //         IterMut { inner: Item { opt: self.as_mut() } }
    //     }
    //
    //     /////////////////////////////////////////////////////////////////////////
    //     // Boolean operations on the values, eager and lazy
    //     /////////////////////////////////////////////////////////////////////////
    //
    //     /// Returns `None` if the option is `None`, otherwise returns `optb`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x = Some(2);
    //     /// let y: Option<&str> = None;
    //     /// assert_eq!(x.and(y), None);
    //     ///
    //     /// let x: Option<u32> = None;
    //     /// let y = Some("foo");
    //     /// assert_eq!(x.and(y), None);
    //     ///
    //     /// let x = Some(2);
    //     /// let y = Some("foo");
    //     /// assert_eq!(x.and(y), Some("foo"));
    //     ///
    //     /// let x: Option<u32> = None;
    //     /// let y: Option<&str> = None;
    //     /// assert_eq!(x.and(y), None);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn and<U>(self, optb: Option<U>) -> Option<U> {
    //         match self {
    //             Some(_) => optb,
    //             None => None,
    //         }
    //     }
    //
    //     /// Returns `None` if the option is `None`, otherwise calls `f` with the
    //     /// wrapped value and returns the result.
    //     ///
    //     /// Some languages call this operation flatmap.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// fn sq(x: u32) -> Option<u32> { Some(x * x) }
    //     /// fn nope(_: u32) -> Option<u32> { None }
    //     ///
    //     /// assert_eq!(Some(2).and_then(sq).and_then(sq), Some(16));
    //     /// assert_eq!(Some(2).and_then(sq).and_then(nope), None);
    //     /// assert_eq!(Some(2).and_then(nope).and_then(sq), None);
    //     /// assert_eq!(None.and_then(sq).and_then(sq), None);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn and_then<U, F: FnOnce(T) -> Option<U>>(self, f: F) -> Option<U> {
    //         match self {
    //             Some(x) => f(x),
    //             None => None,
    //         }
    //     }
    //
    //     /// Returns the option if it contains a value, otherwise returns `optb`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x = Some(2);
    //     /// let y = None;
    //     /// assert_eq!(x.or(y), Some(2));
    //     ///
    //     /// let x = None;
    //     /// let y = Some(100);
    //     /// assert_eq!(x.or(y), Some(100));
    //     ///
    //     /// let x = Some(2);
    //     /// let y = Some(100);
    //     /// assert_eq!(x.or(y), Some(2));
    //     ///
    //     /// let x: Option<u32> = None;
    //     /// let y = None;
    //     /// assert_eq!(x.or(y), None);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn or(self, optb: Option<T>) -> Option<T> {
    //         match self {
    //             Some(_) => self,
    //             None => optb
    //         }
    //     }
    //
    //     /// Returns the option if it contains a value, otherwise calls `f` and
    //     /// returns the result.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// fn nobody() -> Option<&'static str> { None }
    //     /// fn vikings() -> Option<&'static str> { Some("vikings") }
    //     ///
    //     /// assert_eq!(Some("barbarians").or_else(vikings), Some("barbarians"));
    //     /// assert_eq!(None.or_else(vikings), Some("vikings"));
    //     /// assert_eq!(None.or_else(nobody), None);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn or_else<F: FnOnce() -> Option<T>>(self, f: F) -> Option<T> {
    //         match self {
    //             Some(_) => self,
    //             None => f()
    //         }
    //     }
    //
    //     /////////////////////////////////////////////////////////////////////////
    //     // Misc
    //     /////////////////////////////////////////////////////////////////////////
    //
    //     /// Takes the value out of the option, leaving a `None` in its place.
    ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let mut x = Some(2);
    //     /// x.take();
    //     /// assert_eq!(x, None);
    //     ///
    //     /// let mut x: Option<u32> = None;
    //     /// x.take();
    //     /// assert_eq!(x, None);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn take(&mut self) -> Option<T> {
    //         mem::replace(self, None)
    //     }
    //
    //     /// Converts from `Option<T>` to `&[T]` (without copying)
    //     #[inline]
    //     #[unstable(feature = "as_slice", since = "unsure of the utility here")]
    //     pub fn as_slice<'a>(&'a self) -> &'a [T] {
    //         match *self {
    //             Some(ref x) => slice::ref_slice(x),
    //             None => {
    //                 let result: &[_] = &[];
    //                 result
    //             }
    //         }
    //     }
    // }

    type T = u32;
    type U = T;

    struct F;

    type Args = (T,);

    impl FnOnce<Args> for F {
	type Output = Option<U>;

	extern "rust-call" fn call_once(self, (arg,): Args) -> Self::Output {
	    Some::<U>(arg * arg)
	}
    }

    impl Clone for F {
	fn clone(&self) -> Self {
	    F
	}
    }

    fn err(_: T) -> Option<U> {
	None::<U>
    }

    #[test]
    fn and_then_test1() {
	let x: Option<T> = Some::<T>(2);
	let f: F = F;
	let y: Option<U> = x.and_then::<U, F>(f.clone());
	assert_eq!(y, Some::<U>(4));

	let z: Option<U> = y.and_then::<U, F>(f);
	assert_eq!(z, Some::<U>(16));
    }

    #[test]
    fn and_then_test2() {
	let x: Option<T> = Some::<T>(2);
	let f: F = F;
	let y: Option<U> = x.and_then::<U, F>(f);
	assert_eq!(y, Some::<U>(4));

	let z: Option<U> = y.and_then::<U, _>(err);
	assert_eq!(z, None::<U>);
    }

    #[test]
    fn and_then_test3() {
	let x: Option<T> = Some::<T>(2);
	let f: F = F;
	let y: Option<U> = x.and_then::<U, _>(err);
	let z: Option<U> = y.and_then::<U, F>(f);

	assert_eq!(z, None::<U>);
    }

    #[test]
    fn and_then_test4() {
	let x: Option<T> = Some::<T>(2);
	let y: Option<U> = x.and_then::<U, _>(err);
	let z: Option<U> = y.and_then::<U, _>(err);

	assert_eq!(z, None::<U>);
    }

    #[test]
    fn and_then_test5() {
	let x: Option<T> = None::<T>;
	let f: F = F;
	let y: Option<U> = x.and_then::<U, F>(f.clone());
	let z: Option<U> = y.and_then::<U, F>(f);

	assert_eq!(z, None::<U>);
    }

    #[test]
    fn and_then_test6() {
	let x: Option<T> = None::<T>;
	let f: F = F;
	let y: Option<U> = x.and_then::<U, F>(f);
	let z: Option<U> = y.and_then::<U, _>(err);

	assert_eq!(z, None::<U>);
    }

    #[test]
    fn and_then_test7() {
	let x: Option<T> = None::<T>;
	let f: F = F;
	let y: Option<U> = x.and_then::<U, _>(err);
	let z: Option<U> = y.and_then::<U, F>(f);

	assert_eq!(z, None::<U>);
    }

    #[test]
    fn and_then_test8() {
	let x: Option<T> = None::<T>;
	let y: Option<U> = x.and_then::<U, _>(err);
	let z: Option<U> = y.and_then::<U, _>(err);

	assert_eq!(z, None::<U>);
    }
}
