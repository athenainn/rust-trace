#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::IntoIter;

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

    // impl<T> IntoIterator for Option<T> {
    //     type Item = T;
    //     type IntoIter = IntoIter<T>;
    //
    //     /// Returns a consuming iterator over the possibly contained value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x = Some("string");
    //     /// let v: Vec<&str> = x.into_iter().collect();
    //     /// assert_eq!(v, ["string"]);
    //     ///
    //     /// let x = None;
    //     /// let v: Vec<&str> = x.into_iter().collect();
    //     /// assert!(v.is_empty());
    //     /// ```
    //     #[inline]
    //     fn into_iter(self) -> IntoIter<T> {
    //         IntoIter { inner: Item { opt: self } }
    //     }
    // }

    type T = &'static str;

    #[test]
    fn into_iter_test1() {
	let x: Option<T> = Some::<T>("string");
	let mut into_iter: IntoIter<T> = x.into_iter();
	let result: Option<T> = into_iter.next();

	match result {
	    Some(v) => assert_eq!(v, "string"),
	    None => assert!(false)
	}
    }

    #[test]
    fn into_iter_test2() {
	let x: Option<T> = Some::<T>("string");
	let mut n: usize = 0;

	for v in x {
	    n += 1;
	    assert_eq!(v, "string");
	}

	assert_eq!(n, 1);
    }

    #[test]
    fn into_iter_test3() {
	let x: Option<T> = None::<T>;
	let mut into_iter: IntoIter<T> = x.into_iter();
	let result: Option<T> = into_iter.next();

	match result {
	    Some(_) => assert!(false),
	    None => assert!(true)
	}
    }

    #[test]
    fn into_iter_test4() {
	let x: Option<T> = None::<T>;
	let mut n: usize = 0;

	for v in x {
	    n += 1;
	    assert_eq!(v, "string");
	}

	assert_eq!(n, 0);
    }
}
