#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::borrow::Cow::{self, Borrowed, Owned};
    use collections::borrow::Borrow;

    use core::cmp::Ordering::{self, Less, Equal, Greater};

    // pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned {
    //     /// Borrowed data.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Borrowed(&'a B),
    //
    //     /// Owned data.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Owned(<B as ToOwned>::Owned)
    // }

    // impl<'a, B: ?Sized> Ord for Cow<'a, B> where B: Ord + ToOwned {
    //     #[inline]
    //     fn cmp(&self, other: &Cow<'a, B>) -> Ordering {
    //         Ord::cmp(&**self, &**other)
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
    type B = T; // B: ?Sized + Ord + ToOwned

    macro_rules! cmp_test {
	($T:ty) => ({
	    {
		let x: $T = 68;
		let y: $T = 500;
		let cow: Cow<$T> = Borrowed::<$T>(x.borrow());
		let other: Cow<$T> = Owned::<$T>(y);
		let result: Ordering = cow.cmp(&other);

		assert_eq!(result, Less);
	    }
	    {
		let x: $T = 68;
		let y: $T = 68;
		let cow: Cow<$T> = Borrowed::<$T>(x.borrow());
		let other: Cow<$T> = Owned::<$T>(y);
		let result: Ordering = cow.cmp(&other);

		assert_eq!(result, Equal);
	    }
	    {
		let x: $T = 500;
		let y: $T = 68;
		let cow: Cow<$T> = Borrowed::<$T>(x.borrow());
		let other: Cow<$T> = Owned::<$T>(y);
		let result: Ordering = cow.cmp(&other);

		assert_eq!(result, Greater);
	    }
	})
    }

    #[test]
    fn cmp_test1() {
	cmp_test!( B );
    }
}
