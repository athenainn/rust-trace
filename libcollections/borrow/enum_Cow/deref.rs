#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::borrow::Cow::{self, Borrowed, Owned};
    use collections::borrow::Borrow;

    use core::default::Default;

    use core::ops::Deref;

    // pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned {
    //     /// Borrowed data.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Borrowed(&'a B),
    //
    //     /// Owned data.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Owned(<B as ToOwned>::Owned)
    // }

    // impl<'a, B: ?Sized> Deref for Cow<'a, B> where B: ToOwned {
    //     type Target = B;
    //
    //     fn deref(&self) -> &B {
    //         match *self {
    //             Borrowed(borrowed) => borrowed,
    //             Owned(ref owned) => owned.borrow()
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

    struct A<T> {
	value: T
    }

    impl<T> A<T> {
	fn new(value: T) -> Self {
	    A { value: value }
	}
    }

    impl<T: Default> Default for A<T> {
	fn default() -> Self {
	    Self::new(Default::default())
	}
    }

    impl<T: Clone> Clone for A<T> {
	fn clone(&self) -> Self {
	    Self::new(self.value.clone())
	}
    }

    type T = A<i32>;
    type B = T; // B: ?Sized + ToOwned

    #[test]
    fn deref_test1() {
	let x: T = T::default();
	let x_ref: &T = x.borrow();
	let cow: Cow<B> = Borrowed::<B>(x_ref);
	let deref: &B = cow.deref();

	assert_eq!(x.value, 0);
	assert_eq!(cow.value, 0);
	assert_eq!(deref.value, 0);
    }

    #[test]
    fn deref_test2() {
	let x: T = T::default();
	let cow: Cow<B> = Owned::<B>(x);
	let deref: &B = cow.deref();

	assert_eq!(cow.value, 0);
	assert_eq!(deref.value, 0);
    }
}
