#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::borrow::Cow::{self, Borrowed, Owned};
    use collections::borrow::Borrow;

    // pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned {
    //     /// Borrowed data.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Borrowed(&'a B),
    //
    //     /// Owned data.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Owned(<B as ToOwned>::Owned)
    // }

    // impl<'a, B: ?Sized> fmt::Debug for Cow<'a, B> where
    //     B: fmt::Debug + ToOwned,
    //     <B as ToOwned>::Owned: fmt::Debug,
    // {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         match *self {
    //             Borrowed(ref b) => fmt::Debug::fmt(b, f),
    //             Owned(ref o) => fmt::Debug::fmt(o, f),
    //         }
    //     }
    // }

    // impl<'a, B: ?Sized> fmt::Display for Cow<'a, B> where
    //     B: fmt::Display + ToOwned,
    //     <B as ToOwned>::Owned: fmt::Display,
    // {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         match *self {
    //             Borrowed(ref b) => fmt::Display::fmt(b, f),
    //             Owned(ref o) => fmt::Display::fmt(o, f),
    //         }
    //     }
    // }

    // pub trait ToOwned {
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Owned: Borrow<Self>;
    //
    //     /// Creates owned data from borrowed data, usually by cloning.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn to_owned(&self) -> Self::Owned;
    // }

    // impl<T> ToOwned for T where T: Clone {
    //     type Owned = T;
    //     fn to_owned(&self) -> T { self.clone() }
    // }

    // impl<'a, B: ?Sized> Cow<'a, B> where B: ToOwned {
    //     /// Acquires a mutable reference to the owned form of the data.
    //     ///
    //     /// Copies the data if it is not already owned.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::borrow::Cow;
    //     ///
    //     /// let mut cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);
    //     ///
    //     /// let hello = cow.to_mut();
    //     ///
    //     /// assert_eq!(hello, &[1, 2, 3]);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn to_mut(&mut self) -> &mut <B as ToOwned>::Owned {
    //         match *self {
    //             Borrowed(borrowed) => {
    //                 *self = Owned(borrowed.to_owned());
    //                 self.to_mut()
    //             }
    //             Owned(ref mut owned) => owned
    //         }
    //     }
    //
    //     /// Extracts the owned data.
    //     ///
    //     /// Copies the data if it is not already owned.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::borrow::Cow;
    //     ///
    //     /// let cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);
    //     ///
    //     /// let hello = cow.into_owned();
    //     ///
    //     /// assert_eq!(vec![1, 2, 3], hello);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn into_owned(self) -> <B as ToOwned>::Owned {
    //         match self {
    //             Borrowed(borrowed) => borrowed.to_owned(),
    //             Owned(owned) => owned
    //         }
    //     }
    // }

    type T = i32;
    type B = T; // B: ?Sized + fmt::Debug + ToOwned, <B as ToOwned>::Owned: fmt::Debug
		// B: ?Sized + fmt::Display + ToOwned, <B as ToOwned>::Owned: fmt::Display

    #[test]
    fn fmt_test1() {
	let x: B = 68;
	let cow: Cow<B> = Borrowed::<B>(x.borrow());
	let message: String = format!("{}", cow);

	assert_eq!(message, "68".to_string());
    }

    #[test]
    fn fmt_test2() {
	let x: B = 68;
	let cow: Cow<T> = Owned::<B>(x);
	let message: String = format!("{}", cow);

	assert_eq!(message, "68".to_string());
    }
}
