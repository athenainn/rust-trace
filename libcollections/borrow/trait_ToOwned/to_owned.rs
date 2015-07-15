#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::borrow::ToOwned;
    use collections::borrow::Borrow;

    use core::default::Default;

    use core::clone::Clone;

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

    // pub trait Borrow<Borrowed: ?Sized> {
    //     /// Immutably borrows from an owned value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::borrow::Borrow;
    //     ///
    //     /// fn check<T: Borrow<str>>(s: T) {
    //     ///     assert_eq!("Hello", s.borrow());
    //     /// }
    //     ///
    //     /// let s = "Hello".to_string();
    //     ///
    //     /// check(s);
    //     ///
    //     /// let s = "Hello";
    //     ///
    //     /// check(s);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn borrow(&self) -> &Borrowed;
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

    #[test]
    fn to_owned_test1() {
	let x: T = T::default();
	let mut to_owned: <T as ToOwned>::Owned = x.to_owned();

	to_owned.value = 68;

	assert_eq!(x.value, 0);
	assert_eq!(to_owned.value, 68);
    }

    #[test]
    fn to_owned_test2() {
	let x: T = T::default();
	let x_ref: &T = x.borrow();
	let mut to_owned: <T as ToOwned>::Owned = x_ref.to_owned();

	to_owned.value = 68;

	assert_eq!(x.value, 0);
	assert_eq!(to_owned.value, 68);
    }
}
