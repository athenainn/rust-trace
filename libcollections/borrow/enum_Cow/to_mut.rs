#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::borrow::Cow::{self, Borrowed, Owned};
    use collections::borrow::Borrow;

    use core::default::Default;

    use core::clone::Clone;

    // pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned {
    //     /// Borrowed data.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Borrowed(&'a B),
    //
    //     /// Owned data.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Owned(<B as ToOwned>::Owned)
    // }

    // impl<'a, B: ?Sized> Clone for Cow<'a, B> where B: ToOwned {
    //     fn clone(&self) -> Cow<'a, B> {
    //         match *self {
    //             Borrowed(b) => Borrowed(b),
    //             Owned(ref o) => {
    //                 let b: &B = o.borrow();
    //                 Owned(b.to_owned())
    //             },
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

    type T = A<i32>; // T: clone
    type B = T; // B: ToOwned + ?Sized + 'a

    #[test]
    fn to_mut_test1() {
	let x: B = B::default();
	let x_ref: &B = x.borrow();
	let mut cow: Cow<B> = Borrowed::<B>(x_ref);
	let mut clone: Cow<B> = cow.clone();

	assert_eq!(x.value, 0);
	assert_eq!(cow.value, 0);
	assert_eq!(clone.value, 0);

	{
	    match cow {
		Borrowed(_) => assert!(true),
		Owned(_) => assert!(false)
	    }

	    let to_mut: &mut <B as ToOwned>::Owned = cow.to_mut();
	    to_mut.value = 68;

	    match cow {
		Borrowed(_) => assert!(false),
		Owned(_) => assert!(true)
	    }
	}

	assert_eq!(x.value, 0);
	assert_eq!(cow.value, 68);
	assert_eq!(clone.value, 0);

	{
	    match clone {
		Borrowed(_) => assert!(true),
		Owned(_) => assert!(false)
	    }

	    let to_mut: &mut <B as ToOwned>::Owned = clone.to_mut();
	    to_mut.value = 500;

	    match clone {
		Borrowed(_) => assert!(false),
		Owned(_) => assert!(true)
	    }
	}

	assert_eq!(x.value, 0);
	assert_eq!(cow.value, 68);
	assert_eq!(clone.value, 500);
    }

    #[test]
    fn clone_test2() {
	let x: B = B::default();
	let mut cow: Cow<B> = Owned::<B>(x);
	let mut clone: Cow<B> = cow.clone();

	assert_eq!(cow.value, 0);
	assert_eq!(clone.value, 0);

	{
	    let to_mut: &mut <B as ToOwned>::Owned = cow.to_mut();
	    to_mut.value = 68;
	}

	assert_eq!(cow.value, 68);
	assert_eq!(clone.value, 0);

	{
	    let to_mut: &mut <B as ToOwned>::Owned = clone.to_mut();
	    to_mut.value = 500;
	}

	assert_eq!(cow.value, 68);
	assert_eq!(clone.value, 500);
    }
}
